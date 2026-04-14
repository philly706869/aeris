use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Ident,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct TokenData {
    name: Ident,
}

impl Parse for TokenData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        Ok(Self { name })
    }
}

#[proc_macro]
pub fn token(input: TokenStream) -> TokenStream {
    let data = parse_macro_input!(input as TokenData);

    quote! {}.into()
}
