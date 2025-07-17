use std::collections::HashMap;

use proc_macro2::{Ident, Span, TokenStream};
use syn::{Error, FnArg, Pat, Result, Type, TypePath};

pub(crate) fn get_literal_type(ty: &Type) -> &TypePath {
    match ty {
        Type::Path(path) => path,
        Type::Reference(r) => match &*r.elem {
            Type::Path(path) => path,
            _ => panic!("Wrong type"),
        },
        _ => panic!("Wrong type"),
    }
}

pub(crate) fn get_attributes(attr: &TokenStream) -> HashMap<String, String> {
    let attrs_str = attr.to_string();
    match attrs_str.is_empty() {
        true => HashMap::new(),
        false => attrs_str
            .split(',')
            .map(|attr| {
                let attr: Vec<&str> = attr.split('=').map(|token| token.trim()).collect();
                (attr[0].to_string(), attr[1].trim_matches('"').to_string())
            })
            .collect(),
    }
}

pub(crate) fn get_externals(arg: &FnArg) -> Vec<(Ident, &Type)> {
    match arg {
        FnArg::Receiver(_) => panic!("Receiver is not supported"),
        FnArg::Typed(pat_type) => match &*pat_type.pat {
            Pat::Tuple(pat_tuple) => match &*pat_type.ty {
                Type::Tuple(ty_tuple) => pat_tuple
                    .elems
                    .iter()
                    .zip(ty_tuple.elems.iter())
                    .filter_map(|(pat, ty)| match ty {
                        Type::Reference(r) => {
                            pat_to_ident(pat).unwrap().map(|name| (name, &*r.elem))
                        }
                        _ => panic!("type must contain only references (&T)"),
                    })
                    .collect(),
                _ => panic!("Wrong type"),
            },
            pat => match pat_to_ident(pat).unwrap() {
                Some(name) => match &*pat_type.ty {
                    Type::Reference(r) => vec![(name, &*r.elem)],
                    _ => panic!("type must contain only references (&T)"),
                },
                None => vec![],
            },
        },
    }
}

pub(crate) fn get_inputs<'a, I>(args: I) -> Vec<(Ident, &'a Type)>
where
    I: Iterator<Item = &'a FnArg>,
{
    args.map(|arg| match arg {
        FnArg::Receiver(_) => panic!("Receiver is not supported"),
        FnArg::Typed(pat) => (
            pat_to_ident(&*pat.pat)
                .unwrap()
                .unwrap_or(Ident::new("arg", Span::call_site())),
            pat.ty.as_ref(),
        ),
    })
    .collect()
}

pub(crate) fn pat_to_ident(pat: &Pat) -> Result<Option<Ident>> {
    match pat {
        Pat::Const(_) => Ok(None),
        Pat::Ident(pat) => Ok(Some(pat.ident.clone())),
        Pat::Lit(_) => Ok(None),
        Pat::Macro(_) => Ok(None),
        Pat::Or(_) => Ok(None),
        Pat::Paren(_) => Ok(None),
        Pat::Path(pat) => Ok(Some(pat.path.get_ident().unwrap().clone())),
        Pat::Range(_) => Ok(None),
        Pat::Reference(pat) => pat_to_ident(&pat.pat),
        Pat::Rest(_) => Ok(None),
        Pat::Type(pat) => pat_to_ident(&pat.pat),
        Pat::Wild(_) => Ok(None),
        _ => Err(Error::new_spanned(pat, "Wrong type")),
    }
}

pub(crate) fn clear_ref(ty: &Type) -> Type {
    match ty {
        Type::Path(path) => {
            let mut path = path.clone();
            match &mut path.path.segments.last_mut().unwrap().arguments {
                syn::PathArguments::AngleBracketed(args) => {
                    args.args.iter_mut().for_each(|arg| match arg {
                        syn::GenericArgument::Type(ty) => {
                            *arg = syn::GenericArgument::Type(clear_ref(ty))
                        }
                        _ => panic!("Wrong type"),
                    });
                }
                _ => panic!("Wrong type"),
            }
            Type::Path(path)
        }
        Type::Reference(r) => match &*r.elem {
            Type::Path(path) => Type::Path(path.clone()),
            _ => panic!("Wrong type"),
        },
        _ => panic!("Wrong type"),
    }
}
