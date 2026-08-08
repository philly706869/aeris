mod ast;
mod ir;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro_attribute]
pub fn shard(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: TokenStream2 = attr.into();
    let item: TokenStream2 = item.into();

    if !attr.is_empty() {
        return syn::Error::new_spanned(attr, "unexpected attribute argument")
            .into_compile_error()
            .into();
    }

    quote! {}.into()
}
