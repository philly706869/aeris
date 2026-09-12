use proc_macro::TokenStream;

mod shard;

///
#[proc_macro_attribute]
pub fn shard(attr: TokenStream, item: TokenStream) -> TokenStream {
    shard::shard(attr.into(), item.into()).into()
}

/// # XST x!
#[proc_macro]
pub fn x(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xbox!
#[proc_macro]
pub fn xbox(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xopt!
#[proc_macro]
pub fn xopt(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xvec!
#[proc_macro]
pub fn xvec(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xlopt!
#[proc_macro]
pub fn xlopt(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xlvec!
#[proc_macro]
pub fn xlvec(_: TokenStream) -> TokenStream {
    TokenStream::new()
}
