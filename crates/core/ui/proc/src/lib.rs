mod ast;
mod hir;

use quote::quote;
use syn::parse;

use crate::ast::ClusterAST;

#[proc_macro]
pub fn cluster(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let cluster: ClusterAST = match parse(input) {
        Ok(cluster) => cluster,
        Err(err) => return err.to_compile_error().into(),
    };
    quote! {}.into()
}
