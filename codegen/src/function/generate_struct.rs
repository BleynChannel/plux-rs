use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Error, ItemFn, Result, ReturnType, Signature, Type, TypePath};

use super::{
    generate_function::generate_function,
    utils::{get_attributes, get_inputs, get_literal_type},
};

pub(crate) fn generate_struct(
    ast: &ItemFn,
    sig: &Signature,
    ident: &Ident,
    attr: &TokenStream,
    exts: &Vec<(Ident, &Type)>,
) -> Result<TokenStream> {
    let attrs = get_attributes(attr);

    let externals = generate_externals(exts);

    let ins = get_inputs(sig.inputs.iter().skip(1));

    let name = generate_name(attrs.get("name"), &ident.to_string());
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

        impl plux::function::Function for Function {
            type Output = plux::function::FunctionOutput;

            fn name(&self) -> String {
                #name
            }

            fn inputs(&self) -> Vec<plux::function::Arg> {
                #inputs
            }

            fn output(&self) -> Option<plux::function::Arg> {
                #output
            }

            fn call(&self, args: &[plux::variable::Variable]) -> Self::Output {
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

fn generate_name(name: Option<&String>, or: &String) -> TokenStream {
    let name = name.map(|x| x.clone()).unwrap_or(or.to_string());
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
        syn::ReturnType::Type(_, ty) => {
            let arg = generate_arg(&"output".to_string(), &*ty)?;
            Ok(quote! { Some(#arg) })
        }
    }
}

fn generate_arg(name: &String, ty: &Type) -> Result<TokenStream> {
    let ty = get_variable_type_path(get_literal_type(ty))?;
    Ok(quote! { plux::function::Arg::new(#name, #ty) })
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
            Ok(quote! { plux::variable::VariableType::#token })
        }
        None => Err(Error::new_spanned(path, "type is not supported")),
    }
}
