mod ast;

use proc_macro::TokenStream;
use syn::parse_macro_input;

// DUE 1/7

#[proc_macro]
pub fn syntax(stream: TokenStream) -> TokenStream {
    // let syntax = parse_macro_input!(stream as ast::Syntax);
    // let mut pubs = Vec::new();
    // let mut rules = Vec::new();
    // for statement in syntax.statements {
    //     match statement {
    //         ast::Statement::PubDecl(pub_decl) => pubs.push(pub_decl),
    //         ast::Statement::RuleDef(rule_def) => rules.push(rule_def),
    //     }
    // }

    TokenStream::new()
}
