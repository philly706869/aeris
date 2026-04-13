use proc_macro::TokenStream;

#[proc_macro]
pub fn token(stream: TokenStream) -> TokenStream {
    lib::token(stream.into()).into()
}

mod lib {
    use proc_macro2::TokenStream;

    pub fn token(stream: TokenStream) -> TokenStream {
        todo!()
    }
}
