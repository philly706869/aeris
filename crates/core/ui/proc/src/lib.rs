mod ast;
mod ir;

use proc_macro::TokenStream;
use syn::parse;

use crate::ir::ClusterIR;

#[proc_macro]
pub fn cluster(input: TokenStream) -> TokenStream {
    parse(input)
        .and_then(|ast| ClusterIR::lower(&ast))
        .map(|ir| ir.expand())
        .unwrap_or_else(|err| err.into_compile_error())
        .into()
}
