use proc_macro::TokenStream;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

/*
NCFG
extern Ident RArrow LitStr

Definition -> Pub | Rule
Pub -> "pub" Ident
Rule -> Ident "->" Item
Item -> Ident | Ident ":" Item | LitStr | "{" "}"

DCFG

*/

struct SyntaxDefinition {}

enum Statement {
    Definition,
    PubIdent,
    RuleArrow,
    Rule,
}

impl Parse for SyntaxDefinition {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        let mut state = Statement::Definition;
        let mut pubs: Vec<Ident> = Vec::new();
        let mut rules: Vec<Ident> = Vec::new();

        loop {
            match state {
                Statement::Definition => {
                    if stream.parse::<Token![pub]>().is_ok() {
                        state = Statement::PubIdent;
                    } else if let Ok(ident) = stream.parse::<Ident>() {
                        state = Statement::RuleArrow;
                    } else {
                        break;
                    }
                }
                Statement::PubIdent => {
                    if let Ok(ident) = stream.parse::<Ident>() {
                    } else if let Ok(lit_str) = stream.parse::<LitStr>() {
                    } else {
                        break;
                    }
                }
                Statement::RuleArrow => {
                    if stream.parse::<Token![->]>().is_ok() {
                    } else {
                        break;
                    }
                }
                Statement::Rule => {}
            }
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
