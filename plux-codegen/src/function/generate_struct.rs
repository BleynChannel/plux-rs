use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{
    Error, GenericArgument, ItemFn, PathArguments, Result, ReturnType, Signature, Type, TypePath,
};

use super::{
    generate_function::generate_function,
    utils::{get_inputs, get_literal_type},
};

pub(crate) fn generate_struct(
    ast: &ItemFn,
    sig: &Signature,
    ident: &Ident,
    exts: &Vec<(Ident, &Type)>,
) -> Result<TokenStream> {
    let externals = generate_externals(exts);

    let ins = get_inputs(sig.inputs.iter().skip(1));

    let name = generate_name(&ident.to_string());
    // //TODO: Внедрить описание функций в August
    // // let description = generate_description(
    // //     attrs.get("description"),
    // //     &"Description is missing".to_string(),
    // // );
    let inputs = generate_inputs(&ins)?;
    let output = generate_output(&sig.output)?;

    let function = generate_function(
        &exts,
        &ins,
        &sig.output,
        ast.sig.inputs.to_token_stream(),
        ast.block.as_ref().to_token_stream(),
    );

    Ok(quote! {
        struct Function { #externals }

        impl plux_rs::function::Function for Function {
            type Output = plux_rs::function::FunctionOutput;

            fn name(&self) -> String {
                #name
            }

            fn inputs(&self) -> Vec<plux_rs::function::Arg> {
                #inputs
            }

            fn output(&self) -> Option<plux_rs::function::Arg> {
                #output
            }

            fn call(&self, args: &[plux_rs::variable::Variable]) -> Self::Output {
                #function
            }
        }
    })
}

fn generate_externals(exts: &Vec<(Ident, &Type)>) -> TokenStream {
    let exts: Vec<TokenStream> = exts
        .iter()
        .map(|(name, ty)| {
            quote! { #name: #ty }
        })
        .collect();

    quote! { #(#exts),* }
}

fn generate_name(name: &String) -> TokenStream {
    quote! { #name.to_string() }
}

//TODO: Внедрить описание функций в August
// fn generate_description(description: Option<&String>, or: &String) -> TokenStream {
//     let description = description.map(|x| x.clone()).unwrap_or(or.to_string());
//     quote! { #description }
// }

fn generate_inputs(inputs: &Vec<(Ident, &Type)>) -> Result<TokenStream> {
    let mut result = vec![];

    for (name, ty) in inputs {
        result.push(generate_arg(&name.to_string(), *ty)?);
    }

    Ok(quote! { vec![#(#result),*] })
}

fn generate_output(output: &ReturnType) -> Result<TokenStream> {
    match output {
        syn::ReturnType::Default => Ok(quote! { None }),
        syn::ReturnType::Type(_, ty) => match ty.as_ref() {
            Type::Infer(_) | Type::Never(_) => Ok(quote! { None }),
            Type::Tuple(tuple) if tuple.elems.is_empty() => Ok(quote! { None }),
            Type::Path(path) => match path.path.segments.last() {
                Some(segment) if segment.ident.to_string() == "Result" => {
                    match &segment.arguments {
                        PathArguments::AngleBracketed(args) => {
                            if let Some(GenericArgument::Type(ty)) = args.args.first() {
                                match ty {
                                    Type::Tuple(tuple) => tuple
                                        .elems
                                        .is_empty()
                                        .then(|| Ok(quote! { None }))
                                        .unwrap_or_else(|| panic!("Wrong type")),
                                    _ => {
                                        let arg = generate_arg(&"output".to_string(), &*ty)?;
                                        Ok(quote! { Some(#arg) })
                                    }
                                }
                            } else {
                                panic!("Wrong type")
                            }
                        }
                        _ => panic!("Wrong type"),
                    }
                }
                _ => {
                    let arg = generate_arg(&"output".to_string(), &*ty)?;
                    Ok(quote! { Some(#arg) })
                }
            },
            _ => panic!("Wrong type"),
        },
    }
}

fn generate_arg(name: &String, ty: &Type) -> Result<TokenStream> {
    let ty = get_variable_type_path(get_literal_type(ty))?;
    Ok(quote! { plux_rs::function::Arg::new(#name, #ty) })
}

const VARIABLE_TYPES: [(&str, &str); 15] = [
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
    ("Vec", "List"),
    ("Variable", "Let"),
];

fn get_variable_type_path(path: &TypePath) -> Result<TokenStream> {
    let ident = path.path.segments.last().unwrap().ident.to_string();

    match VARIABLE_TYPES.into_iter().find(|(name, _)| **name == ident) {
        Some((_, token)) => {
            let token = format_ident!("{}", token);
            Ok(quote! { plux_rs::variable::VariableType::#token })
        }
        None => Err(Error::new_spanned(path, "type is not supported")),
    }
}
