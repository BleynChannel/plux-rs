mod function;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn function(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemFn);

    function::derive(ast, attr.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
