use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
};

// DUE 0/7

// Syntax Definition NCFG
//         X -> Statement*
// Statement -> PubDecl | RuleDef
//   PubDecl -> "pub" Ident
//   RuleDef -> Ident "->" Sequence
//  Sequence -> L1Item+ ("|" L1Item+)*
//    L1Item -> (Ident ":")? L2Item
//    L2Item -> L3Item Quant?
//    L3Item -> Ident | LitStr | LitChar | Set | Group
//     Quant -> "*" | "+" | "?"
//       Set -> "{" "!"? LitChar* "}"
//     Group -> "(" Sequence ")"

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
