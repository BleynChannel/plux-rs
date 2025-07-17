use proc_macro2::TokenStream;
use syn::{ItemFn, Result};

mod generate_function;
mod generate_struct;
mod generator;
mod utils;
mod validator;

pub fn derive(ast: ItemFn, attr: TokenStream) -> Result<TokenStream> {
    validator::validate(&ast, &attr)?;
    Ok(generator::generate(&ast, &attr)?)
}
