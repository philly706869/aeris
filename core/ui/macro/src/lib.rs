use std::collections::HashMap;

use proc_macro::TokenStream;
use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct Syntax {
    main_rule: Ident,
    rules: HashMap<Ident, Vec<Vec<Char>>>,
}

enum Char {
    Terminal(LitStr),
    NonTerminal(Option<Ident>, Ident),
}

impl Parse for Syntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut main_rule: Option<Ident> = None;
        let mut rules: HashMap<Ident, Vec<Vec<Char>>> = HashMap::new();
        'outer: loop {
            let mut stack: Vec<Char> = Vec::new();
            let name: Ident = input.parse()?;
            input.parse::<Token![->]>()?;
            loop {
                if let Ok(rule) = input.parse::<LitStr>() {
                    stack.push(Char::Terminal(rule));
                } else if let Ok(rule) = input.parse::<Ident>() {
                    stack.push(Char::NonTerminal(None, rule));
                } else {
                    return Err(input.error("expected Terminal or Non-Terminal"));
                }

                if input.is_empty() {
                    break 'outer;
                }

                if input.peek2(Token![->]) {
                    break;
                }
            }
            main_rule.get_or_insert_with(|| name.clone());
            rules.entry(name).or_insert_with(|| Vec::new()).push(stack);
        }
        Ok(Syntax {
            main_rule: main_rule.unwrap(),
            rules,
        })
    }
}

#[proc_macro]
pub fn syntax(stream: TokenStream) -> TokenStream {
    // parse_macro_input!(stream as Syntax);
    TokenStream::new()
}
