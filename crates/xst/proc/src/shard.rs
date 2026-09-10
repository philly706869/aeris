mod ast;

use proc_macro2::TokenStream;
use syn::parse2;

pub fn shard(attr: TokenStream, item: TokenStream) -> TokenStream {
    let expanded_attr = expand_attr(attr);
    let expanded_item = expand_item(item);
    let mut expanded = TokenStream::new();
    expanded.extend(expanded_item);
    expanded.extend(expanded_attr);
    expanded
}

fn expand_attr(attr: TokenStream) -> TokenStream {
    if attr.is_empty() {
        return TokenStream::new();
    }
    syn::Error::new_spanned(attr, "unexpected attribute argument").into_compile_error()
}

fn expand_item(item: TokenStream) -> TokenStream {
    match parse2::<ast::Shard>(item) {
        Ok(_shard) => TokenStream::new(), // Code generation is a separate step.
        Err(error) => error.into_compile_error(),
    }
}
