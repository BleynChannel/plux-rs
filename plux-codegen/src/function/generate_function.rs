use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{GenericArgument, PathArguments, ReturnType, Type, TypePath};

use crate::function::utils::{clear_ref, get_literal_type};

pub(crate) fn generate_function(
    externals: &Vec<(Ident, &Type)>,
    inputs: &Vec<(Ident, &Type)>,
    output: &ReturnType,
    args: TokenStream,
    block: TokenStream,
) -> TokenStream {
    let exts = generate_exts(externals);
    let ins = generate_inputs(inputs);
    let call = function_call(exts, ins, output);
    let out = return_output(output);

    quote! {
        let func = move |#args| #output #block;
        #call
        #out
    }
}

fn generate_exts(externals: &Vec<(Ident, &Type)>) -> TokenStream {
    let exts: Vec<TokenStream> = externals
        .iter()
        .map(|(name, _)| {
            quote! { &self.#name }
        })
        .collect();

    quote! { (#(#exts), *) }
}

fn generate_inputs(inputs: &Vec<(Ident, &Type)>) -> TokenStream {
    let args: Vec<TokenStream> = inputs
        .iter()
        .enumerate()
        .map(|(index, (_, ty))| {
            let ty = clear_ref(*ty);
            let type_name = ty.path.segments.last().unwrap().ident.to_string();
            if type_name == "Variable" {
                quote! { &args[#index].clone() }
            } else {
                quote! { args[#index].try_parse_ref::<#ty>()? }
            }
        })
        .collect();

    quote! { #(#args), * }
}

fn function_call(exts: TokenStream, args: TokenStream, output: &ReturnType) -> TokenStream {
    let (output_token, question_token) = match output {
        syn::ReturnType::Default => (None, None),
        syn::ReturnType::Type(_, ty) => match ty.as_ref() {
            Type::Path(path) => match path.path.segments.last() {
                Some(segment) if segment.ident.to_string() == "Result" => (
                    match &segment.arguments {
                        PathArguments::AngleBracketed(args) => {
                            if let Some(GenericArgument::Type(ty)) = args.args.first() {
                                match ty {
                                    Type::Tuple(tuple) => (tuple.elems.len() == 0)
                                        .then(|| None)
                                        .unwrap_or_else(|| panic!("Wrong type")),
                                    _ => Some(quote! { let result = }),
                                }
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    Some(quote! { ? }),
                ),
                _ => (Some(quote! { let result = }), None),
            },
            _ => (None, None),
        },
    };

    quote! { #output_token func(#exts, #args)#question_token; }
}

fn return_output(output: &ReturnType) -> TokenStream {
    match output {
        syn::ReturnType::Default => quote! { Ok(None) },
        syn::ReturnType::Type(_, ty) => match ty.as_ref() {
            Type::Infer(_) | Type::Never(_) => quote! { Ok(None) },
            Type::Tuple(tuple) if tuple.elems.is_empty() => quote! { Ok(None) },
            Type::Path(path) => match path.path.segments.last() {
                Some(segment) if segment.ident == "Result" => match &segment.arguments {
                    PathArguments::AngleBracketed(args) => {
                        if let Some(GenericArgument::Type(ty)) = args.args.first() {
                            match ty {
                                Type::Tuple(tuple) => tuple
                                    .elems
                                    .is_empty()
                                    .then(|| quote! { Ok(None) })
                                    .unwrap_or_else(|| panic!("Wrong type")),
                                _ => {
                                    let result = serialize_output(get_literal_type(ty));
                                    quote! { Ok(Some(#result)) }
                                }
                            }
                        } else {
                            panic!("Wrong type")
                        }
                    }
                    _ => panic!("Wrong type"),
                },
                _ => {
                    let result = serialize_output(get_literal_type(ty.as_ref()));
                    quote! { Ok(Some(#result)) }
                }
            },
            _ => panic!("Wrong type"),
        },
    }
}

const VARIABLE_DATAS: [(&str, &str); 13] = [
    ("i8", "I8"),
    ("i16", "I16"),
    ("i32", "I32"),
    ("i64", "I64"),
    ("u8", "U8"),
    ("u16", "U16"),
    ("u32", "U32"),
    ("u64", "U64"),
    ("f32", "F32"),
    ("f64", "F64"),
    ("bool", "Bool"),
    ("char", "Char"),
    ("String", "String"),
];

fn serialize_output(ty: &TypePath) -> TokenStream {
    let type_name = ty.path.segments.last().unwrap().ident.to_string();

    if let Some((_, token)) = VARIABLE_DATAS.iter().find(|(name, _)| **name == type_name) {
        let token = format_ident!("{}", *token);
        quote! { plux_rs::variable::Variable::#token (result) }
    } else if type_name == "Vec" {
        quote! { plux_rs::variable::Variable::List(result.into_iter().map(|item| item.into()).collect()) }
    } else if type_name == "Variable" {
        quote! { result }
    } else {
        TokenStream::new()
    }
}
