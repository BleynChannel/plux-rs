use super::{generate_struct::generate_struct, utils::get_externals};

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{ItemFn, Result, Type};

pub(crate) fn generate(ast: &ItemFn, attr: &TokenStream) -> Result<TokenStream> {
    let sig = &ast.sig;
    let ident = &sig.ident;

    let exts = get_externals(&sig.inputs[0]);
    let exts_args = generate_externals_atributes(&exts);
    let externals = generate_externals(&exts);

    let structure = generate_struct(ast, sig, ident, attr, &exts)?;

    Ok(quote! {
        pub fn #ident(#exts_args) -> impl august_plugin_system::function::Function<Output = august_plugin_system::function::FunctionOutput> {
            #structure

            Function { #externals }
        }
    })
}

fn generate_externals_atributes(exts: &Vec<(Ident, &Type)>) -> TokenStream {
    let exts: Vec<_> = exts
        .iter()
        .map(|(name, ty)| {
            quote! { #name: #ty }
        })
        .collect();

    quote! { #(#exts),* }
}

fn generate_externals(exts: &Vec<(Ident, &Type)>) -> TokenStream {
    let exts = exts.iter().map(|(name, _)| name).collect::<Vec<_>>();

    quote! { #(#exts),* }
}
