mod ast;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::ast::{Statement, Statements};

#[proc_macro]
pub fn syntax(input: TokenStream) -> TokenStream {
    let statements = parse_macro_input!(input as Statements);
    let mut expanded = proc_macro2::TokenStream::new();
    for statement in statements.0 {
        let stream = match statement {
            Statement::Struct(s) => {
                let attrs = &s.attrs;
                let vis = &s.vis;
                let name = &s.name;
                quote! {
                    #(#attrs)*
                    #vis struct #name {}

                    impl aeris_ui::SyntaxShard for #name {}
                }
            }
            Statement::Enum(e) => {
                let attrs = &e.attrs;
                let vis = &e.vis;
                let name = &e.name;
                quote! {
                    #(#attrs)*
                    #vis enum #name {}

                    impl aeris_ui::SyntaxShard for #name {}
                }
            }
            _ => quote! {},
        };
        expanded.extend(stream);
    }
    expanded.into()
}
