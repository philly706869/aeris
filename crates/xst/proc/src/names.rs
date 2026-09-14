use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::format_ident;
use rustc_hash::FxHashSet;

pub struct Names {
    used: FxHashSet<String>,
}

impl Names {
    pub fn new(input: TokenStream) -> Self {
        let mut names = Self {
            used: FxHashSet::default(),
        };
        names.reserve(input);
        names
    }

    fn reserve(&mut self, tokens: TokenStream) {
        for token in tokens {
            match token {
                TokenTree::Ident(ident) => {
                    let name = ident.to_string();
                    self.used
                        .insert(name.strip_prefix("r#").unwrap_or(&name).to_owned());
                }
                TokenTree::Group(group) => self.reserve(group.stream()),
                _ => {}
            }
        }
    }

    pub fn fresh(&mut self, kind: &str) -> Ident {
        for index in 0usize.. {
            let name = format!("__xst_{kind}_{index}");
            if self.used.insert(name.clone()) {
                return format_ident!("{name}");
            }
        }
        unreachable!("generated identifier space exhausted")
    }
}
