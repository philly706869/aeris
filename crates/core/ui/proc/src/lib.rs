mod ast;
mod ir;

use proc_macro::TokenStream;
use syn::parse;

use crate::ir::ClusterIR;

#[proc_macro]
pub fn cluster(input: TokenStream) -> TokenStream {
    match parse(input) {
        Ok(ast) => ClusterIR::lower(&ast).expand(),
        Err(err) => err.to_compile_error(),
    }
    .into()
}
