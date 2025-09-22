use proc_macro2::TokenStream;
use syn::{
    Error, FnArg, GenericArgument, ItemFn, Pat, PathArguments, Result, Signature, Type, TypePath,
};

use super::utils::pat_to_ident;

pub(crate) fn validate(ast: &ItemFn, _: &TokenStream) -> Result<()> {
    if !ast.sig.generics.params.is_empty() {
        return Err(Error::new_spanned(ast, "generics are not supported"));
    }

    validate_function(&ast.sig)
}

fn validate_function(sig: &Signature) -> Result<()> {
    validate_externals(&sig.inputs[0])?;
    validate_args(sig.inputs.iter().skip(1))?;

    if let syn::ReturnType::Type(_, ref ty) = sig.output {
        validate_output_type(ty.as_ref())?;
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

fn validate_output_type(ty: &Type) -> Result<()> {
    match ty {
        Type::Infer(_) | Type::Never(_) => Ok(()),
        Type::Tuple(tuple) if tuple.elems.is_empty() => Ok(()),
        Type::Path(path) => match path.path.segments.last() {
            Some(segment) if segment.ident == "Result" => match &segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    if let Some(GenericArgument::Type(ty)) = args.args.first() {
                        match ty {
                            Type::Tuple(tuple) => tuple.elems.is_empty().then(|| Ok(())).unwrap_or_else(|| {
                                Err(Error::new_spanned(
                                    tuple,
                                    "type must contain only (), _, !, T, Result<T, ...> or Result<(), ...>",
                                ))
                            }),
                            _ => validate_type(ty, false),
                        }
                    } else {
                        Err(Error::new_spanned(
                            args.args.first().unwrap(),
                            "Result must contain only a type",
                        ))
                    }
                }
                _ => Err(Error::new_spanned(
                    segment,
                    "type must contain only (), _, !, T, Result<T, ...> or Result<(), ...>",
                )),
            },
            _ => validate_type_path(path, false),
        },
        _ => Err(Error::new_spanned(
            ty,
            "type must contain only (), _, !, T or Result<T, ...> or Result<(), ...>",
        )),
    }
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
