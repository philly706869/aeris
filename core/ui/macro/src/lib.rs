use proc_macro::TokenStream;

#[proc_macro]
pub fn token(input: TokenStream) -> TokenStream {
    match lib::token(input.into()) {
        Ok(expanded) => expanded.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

mod lib {
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{
        Ident,
        parse::{Parse, ParseStream},
        parse2,
    };

    struct Data {
        name: Ident,
    }

    impl Parse for Data {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let name: Ident = input.parse()?;
            Ok(Self { name })
        }
    }

    pub fn token(input: TokenStream) -> syn::Result<TokenStream> {
        let data: Data = parse2(input)?;

        let expanded = quote! {};

        Ok(expanded)
    }
}
