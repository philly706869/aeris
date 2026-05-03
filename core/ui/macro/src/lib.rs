use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
};

/*
NCFG

S -> StatementList
StatementList -> Statement | Statement StatementList
Statement -> PubDecl | RuleDef
PubDecl -> "pub" Ident
RuleDef -> Ident "->" Body
Body -> Sequence | Sequence "|" Body
Sequence -> Item | Item Sequence
Item -> Atom | Atom Quantifier | Ident ":" Atom | Ident ":" Atom Quantifier
Atom -> Ident | LitStr | LitChar | Set | Group
Quantifier -> "*" | "+" | "?"
Set -> "{" SetElement "}" | "{" "!" SetElement "}"
SetElement -> LitChar SetElement | eps
Group -> "(" Body ")"

DCFG

*/

struct SyntaxDefinition {}

impl Parse for SyntaxDefinition {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        Ok(SyntaxDefinition {})
    }
}

#[proc_macro]
pub fn syntax(stream: TokenStream) -> TokenStream {
    parse_macro_input!(stream as SyntaxDefinition);
    TokenStream::new()
}
