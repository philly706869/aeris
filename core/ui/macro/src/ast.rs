use syn::{
    Ident, LitChar, LitStr, Token,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
};

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

#[derive(Debug)]
pub struct Syntax {
    pub statements: Vec<Statement>,
}

impl Parse for Syntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut statements = Vec::new();
        while !input.is_empty() {
            statements.push(input.parse()?);
        }
        Ok(Syntax { statements })
    }
}

#[derive(Debug)]
pub enum Statement {
    PubDecl(PubDecl),
    RuleDef(RuleDef),
}

impl Parse for Statement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![pub]) {
            Ok(Statement::PubDecl(input.parse()?))
        } else {
            Ok(Statement::RuleDef(input.parse()?))
        }
    }
}

#[derive(Debug)]
pub struct PubDecl {
    pub name: Ident,
}

impl Parse for PubDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![pub]>()?;
        let name = input.parse()?;
        Ok(PubDecl { name })
    }
}

#[derive(Debug)]
pub struct RuleDef {
    pub name: Ident,
    pub sequence: Sequence,
}

impl Parse for RuleDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![->]>()?;
        let sequence = input.parse()?;
        Ok(RuleDef { name, sequence })
    }
}

#[derive(Debug)]
pub struct Sequence {
    pub alternatives: Vec<Vec<L1Item>>,
}

impl Parse for Sequence {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut alternatives = Vec::new();
        loop {
            let mut items = Vec::new();
            while !input.is_empty() {
                if input.peek(Token![|]) {
                    break;
                }

                if input.peek(Token![pub]) || (input.peek(Ident) && input.peek2(Token![->])) {
                    break;
                }

                items.push(input.parse()?);
            }
            alternatives.push(items);
            if input.peek(Token![|]) {
                input.parse::<Token![|]>()?;
            } else {
                break;
            }
        }
        Ok(Sequence { alternatives })
    }
}

#[derive(Debug)]
pub struct L1Item {
    pub name: Option<Ident>,
    pub item: L2Item,
}

impl Parse for L1Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = if input.peek(Ident) && input.peek2(Token![:]) {
            let name = input.parse()?;
            input.parse::<Token![:]>()?;
            Some(name)
        } else {
            None
        };
        let item = input.parse()?;
        Ok(L1Item { name, item })
    }
}

#[derive(Debug)]
pub struct L2Item {
    pub item: L3Item,
    pub quant: Option<Quant>,
}

impl Parse for L2Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item = input.parse()?;
        let quant = if input.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            Some(Quant::Star)
        } else if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            Some(Quant::Plus)
        } else if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            Some(Quant::Optional)
        } else {
            None
        };
        Ok(L2Item { item, quant })
    }
}

#[derive(Debug)]
pub enum L3Item {
    Ident(Ident),
    LitStr(LitStr),
    LitChar(LitChar),
    Set(Set),
    Group(Group),
}

impl Parse for L3Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            Ok(L3Item::Ident(input.parse()?))
        } else if lookahead.peek(LitStr) {
            Ok(L3Item::LitStr(input.parse()?))
        } else if lookahead.peek(LitChar) {
            Ok(L3Item::LitChar(input.parse()?))
        } else if lookahead.peek(Brace) {
            Ok(L3Item::Set(input.parse()?))
        } else if lookahead.peek(Paren) {
            Ok(L3Item::Group(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
pub enum Quant {
    Star,
    Plus,
    Optional,
}

#[derive(Debug)]
pub struct Set {
    pub negated: bool,
    pub chars: Vec<LitChar>,
}

impl Parse for Set {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::braced!(content in input);
        let negated = if content.peek(Token![!]) {
            content.parse::<Token![!]>()?;
            true
        } else {
            false
        };
        let mut chars = Vec::new();
        while !content.is_empty() {
            chars.push(content.parse()?);
        }
        Ok(Set { negated, chars })
    }
}

#[derive(Debug)]
pub struct Group {
    pub sequence: Sequence,
}

impl Parse for Group {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let sequence = content.parse()?;
        Ok(Group { sequence })
    }
}
