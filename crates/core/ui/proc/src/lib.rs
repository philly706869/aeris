mod ast;
mod ir;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse;

#[proc_macro_attribute]
pub fn shard(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut errs = Vec::new();

    if !attr.is_empty() {
        let attr: proc_macro2::TokenStream = attr.into();
        errs.push(syn::Error::new_spanned(
            attr,
            "unexpected attribute argument",
        ));
    }

    match parse::<ast::Shard>(item) {
        Ok(shard) => {}
        Err(err) => {
            errs.push(err);
        }
    }

    match errs.into_iter().reduce(|mut acc, curr| {
        acc.combine(curr);
        acc
    }) {
        Some(err) => err.into_compile_error(),
        None => quote! {},
    }
    .into()
}
