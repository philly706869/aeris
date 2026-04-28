mod parser;

use proc_macro::{Span, TokenStream};
use syn::{
    LitStr,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

use crate::parser::parse_syntax;

struct Paths {
    values: Vec<LitStr>,
}

impl Parse for Paths {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut paths = Vec::new();
        while !input.is_empty() {
            paths.push(input.parse()?);
        }
        Ok(Paths { values: paths })
    }
}

#[proc_macro]
pub fn syntax(stream: TokenStream) -> TokenStream {
    let mut current_path = Span::call_site()
        .local_file()
        .expect("cannot get local file");
    current_path.pop();

    let paths = parse_macro_input!(stream as Paths);

    for lit in paths.values {
        let path = current_path.join(lit.value());
        let content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                return syn::Error::new(lit.span(), err).into_compile_error().into();
            }
        };
        let res = parse_syntax(&content);
    }

    TokenStream::new()
}
