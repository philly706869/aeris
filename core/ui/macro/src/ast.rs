use syn::{
    Ident, LitInt, Token, Visibility, braced, bracketed, parenthesized,
    parse::{Lookahead1, Parse, ParseStream},
    token::{Brace, Bracket, Paren},
};

//  Sequence -> "|"? L1Item+ ("|" L1Item+)*
//    L1Item -> (Ident ":")? L2Item
//    L2Item -> L3Item Quant?
//    L3Item -> Ident | LitStr | LitChar | Set | Group
//     Quant -> "*" | "+" | "?"
//       Set -> "{" "!"? LitChar* "}"
//     Group -> "(" Sequence ")"

#[derive(Debug)]
pub struct Syntax {}

impl Parse for Syntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Struct {
    visibility: Visibility,
    name: Ident,
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility: Visibility = input.parse()?;
        let _: Token![struct] = input.parse()?;
        let name: Ident = input.parse()?;
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(Brace) {
            let content;
            braced!(content in input);
        } else if lookahead1.peek(Paren) {
            let content;
            parenthesized!(content in input);
        } else {
            return Err(lookahead1.error());
        }
        Ok(Self { visibility, name })
    }
}

#[derive(Debug)]
pub struct Enum {
    visibility: Visibility,
    name: Ident,
}

impl Parse for Enum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility: Visibility = input.parse()?;
        let _: Token![enum] = input.parse()?;
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        Ok(Self { visibility, name })
    }
}

#[derive(Debug)]
pub struct Lambda {
    name: Ident,
}

impl Parse for Lambda {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        parenthesized!(content in input);
        Ok(Self { name })
    }
}

#[derive(Debug)]
pub struct NamedEntry {
    name: Ident,
    tuple: bool,
    entry: L1Entry,
}

impl Parse for NamedEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        let name: Ident;
        let tuple: bool;
        if lookahead1.peek(Ident) {
            name = input.parse()?;
            tuple = false;
        } else if lookahead1.peek(Paren) {
            let content;
            parenthesized!(content in input);
            name = content.parse()?;
            tuple = true;
        } else {
            return Err(lookahead1.error());
        }
        let _: Token![:] = input.parse()?;
        let entry: L1Entry = input.parse()?;
        Ok(Self { name, tuple, entry })
    }
}

#[derive(Debug)]
pub struct L1Entry {
    entry: L2Entry,
    quantifier: Quantifier,
}

impl Parse for L1Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let entry: L2Entry = input.parse()?;
        let quantifier: Quantifier = input.parse()?;
        Ok(Self { entry, quantifier })
    }
}

#[derive(Debug)]
pub struct L2Entry {}

impl Parse for L2Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub enum Quantifier {
    None,
    Plus,
    Star,
    Option,
    Val(LitInt),
    Min(LitInt),
    Max(LitInt),
    Range(LitInt, LitInt),
}

impl Parse for Quantifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let quant = if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            Self::Plus
        } else if input.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            Self::Star
        } else if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            Self::Option
        } else if input.peek(Bracket) {
            let content;
            bracketed!(content in input);
            let lookahead1 = content.lookahead1();
            if lookahead1.peek(LitInt) {
                if content.peek2(Token![,]) {
                    if content.peek3(LitInt) {
                        let min: LitInt = content.parse()?;
                        let _: Token![,] = content.parse()?;
                        let max: LitInt = content.parse()?;
                        Self::Range(min, max)
                    } else {
                        let min: LitInt = content.parse()?;
                        let _: Token![,] = content.parse()?;
                        Self::Min(min)
                    }
                } else {
                    let val: LitInt = content.parse()?;
                    Self::Val(val)
                }
            } else if lookahead1.peek(Token![,]) {
                let _: Token![,] = content.parse()?;
                let max: LitInt = content.parse()?;
                Self::Max(max)
            } else {
                return Err(lookahead1.error());
            }
        } else {
            Self::None
        };
        Ok(quant)
    }
}
