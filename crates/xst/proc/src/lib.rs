use proc_macro::TokenStream;

mod ast;
mod shard;

///
#[proc_macro_attribute]
pub fn shard(attr: TokenStream, item: TokenStream) -> TokenStream {
    shard::shard(attr.into(), item.into()).into()
}

/// # XST x! rune
#[proc_macro]
pub fn x(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xbox! rune
#[proc_macro]
pub fn xbox(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xopt! rune
#[proc_macro]
pub fn xopt(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xvec! rune
#[proc_macro]
pub fn xvec(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xoptz! rune
#[proc_macro]
pub fn xoptz(_: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// # XST xvecz! rune
#[proc_macro]
pub fn xvecz(_: TokenStream) -> TokenStream {
    TokenStream::new()
}
