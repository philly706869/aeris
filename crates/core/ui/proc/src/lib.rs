mod ast;
mod hir;

use proc_macro::TokenStream;
use syn::parse;

use crate::hir::ClusterHIR;

#[proc_macro]
pub fn cluster(input: TokenStream) -> TokenStream {
    match parse(input) {
        Ok(ast) => ClusterHIR::lower(&ast).expand(),
        Err(err) => err.to_compile_error(),
    }
    .into()
}
