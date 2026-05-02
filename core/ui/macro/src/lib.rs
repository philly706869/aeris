use proc_macro::TokenStream;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct SyntaxDefinition {}

impl Parse for SyntaxDefinition {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        let mut pubs: Vec<Ident> = Vec::new();
        let mut rules: Vec<Ident> = Vec::new();

        loop {
            if stream.parse::<Token![pub]>().is_ok() {
                pubs.push(stream.parse()?);
            } else if let Ok(ident) = stream.parse::<Ident>() {
                stream.parse::<Token![->]>()?;
                loop {
                    if let Ok(ident) = stream.parse::<Ident>() {
                    } else if let Ok(lit_str) = stream.parse::<LitStr>() {
                    } else {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        Ok(SyntaxDefinition {})
    }
}

#[proc_macro]
pub fn syntax(stream: TokenStream) -> TokenStream {
    let syntax_definition = parse_macro_input!(stream as SyntaxDefinition);
    TokenStream::new()
}
