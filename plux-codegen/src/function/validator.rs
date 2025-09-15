use proc_macro2::TokenStream;
use syn::{
    Error, FnArg, GenericArgument, ItemFn, Pat, PathArguments, Result, Signature, Type, TypePath,
};

use super::utils::pat_to_ident;

pub(crate) fn validate(ast: &ItemFn, attr: &TokenStream) -> Result<()> {
    if !ast.sig.generics.params.is_empty() {
        return Err(Error::new_spanned(ast, "generics are not supported"));
    }

    validate_attributes(attr)?;
    validate_function(&ast.sig)
}

//TODO: Внедрить описание функций в August
// const VALIDATE_ATTRIBUTES: [&str; 2] = ["name", "description"];
// const VALIDATE_STRING_ATTRIBUTES: [&str; 2] = ["name", "description"];

const VALIDATE_ATTRIBUTES: [&str; 1] = ["name"];
const VALIDATE_STRING_ATTRIBUTES: [&str; 1] = ["name"];

fn validate_attributes(attrs: &TokenStream) -> Result<()> {
    let attrs_str = attrs.to_string();
    if !attrs_str.is_empty() {
        for attr in attrs_str.split(',') {
            let attr: Vec<&str> = attr.split('=').map(|token| token.trim()).collect();

            if attr.len() != 2 {
                return Err(Error::new_spanned(
                    attrs,
                    "attributes must have the format `path = data`",
                ));
            }

            let path = attr[0];

            if !VALIDATE_ATTRIBUTES.iter().any(|attr| *attr == path) {
                return Err(Error::new_spanned(
                    attrs,
                    format!("attribute `{}` does not exist", path),
                ));
            }

            if VALIDATE_STRING_ATTRIBUTES.iter().any(|attr| *attr == path) {
                let data: Vec<char> = attr[1].chars().collect();
                if data.first() != Some(&'"') || data.last() != Some(&'"') {
                    return Err(Error::new_spanned(
                        attrs,
                        format!("attribute `{}` must contain string", path),
                    ));
                }
            }
        }
    }

    Ok(())
}

fn validate_function(sig: &Signature) -> Result<()> {
    validate_externals(&sig.inputs[0])?;
    validate_args(sig.inputs.iter().skip(1))?;

    if let syn::ReturnType::Type(_, ref ty) = sig.output {
        validate_type(ty.as_ref(), false)?;
    }

    Ok(())
}

fn validate_externals(exts: &FnArg) -> Result<()> {
    match exts {
        FnArg::Receiver(_) => Err(Error::new_spanned(exts, "Receiver is not supported")),
        FnArg::Typed(pat) => match &*pat.ty {
            Type::Tuple(_) => match &*pat.pat {
                Pat::Tuple(tuple) => tuple
                    .elems
                    .iter()
                    .try_for_each(|pat| validate_externals_name(pat)),
                pat => validate_externals_name(pat),
            },
            _ => validate_externals_name(&*pat.pat),
        },
    }
}

fn validate_externals_name(pat: &Pat) -> Result<()> {
    match pat_to_ident(pat) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::new_spanned(
            pat,
            "name of the external is specified incorrectly",
        )),
    }
}

fn validate_args<'a, I>(mut args: I) -> Result<()>
where
    I: Iterator<Item = &'a FnArg>,
{
    args.try_for_each(|arg| match arg {
        FnArg::Receiver(_) => Err(Error::new_spanned(arg, "Receiver is not supported")),
        FnArg::Typed(pat) => validate_type(&*pat.ty, true),
    })?;

    Ok(())
}

fn validate_type(ty: &Type, is_ref: bool) -> Result<()> {
    match is_ref {
        true => match ty {
            Type::Path(path) => validate_type_path(&path, is_ref),
            Type::Reference(r) => match r.mutability {
                None => validate_type(&*r.elem, false),
                _ => Err(Error::new_spanned(
                    ty,
                    "type must not contain a mutated reference",
                )),
            },
            ty => Err(Error::new_spanned(
                ty,
                "type must contain only references (&T) or Vec<&T>",
            )),
        },
        false => match ty {
            Type::Path(path) => validate_type_path(&path, is_ref),
            ty => Err(Error::new_spanned(
                ty,
                "type must contain only literals (T)",
            )),
        },
    }
}

const VALIDATE_TYPE: [&str; 15] = [
    "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool", "char", "String",
    "Vec", "Variable",
];

fn validate_type_path(path: &TypePath, is_ref: bool) -> Result<()> {
    let segment = path.path.segments.last().unwrap();
    let ty = segment.ident.to_string();

    if VALIDATE_TYPE.contains(&ty.as_str()) {
        if ty == "Vec" {
            match &segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    let arg = args.args.first().unwrap();
                    match arg {
                        GenericArgument::Type(ty) => return validate_type(ty, is_ref),
                        _ => return Err(Error::new_spanned(arg, "Vec must contain only a type")),
                    }
                }
                _ => (),
            }
        } else if is_ref {
            return Err(Error::new_spanned(
                path,
                "type must contain only references (&T) or Vec<&T>",
            ));
        }
    } else {
        return Err(Error::new_spanned(path, "type is not supported"));
    }

    Ok(())
}
