use syn::{
    Ident, LitChar, LitInt, LitStr, Token, Visibility, braced, bracketed, parenthesized,
    parse::{Parse, ParseStream, discouraged::Speculative},
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
pub struct Syntax {
    statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Struct(Struct),
    Enum(Enum),
    Lambda(Lambda),
}

impl Parse for Syntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut statements = Vec::new();
        loop {
            let fork = input.fork();
            if let Ok(struct_) = fork.parse() {
                input.advance_to(&fork);
                statements.push(Statement::Struct(struct_));
                continue;
            }

            let fork = input.fork();
            if let Ok(enum_) = fork.parse() {
                input.advance_to(&fork);
                statements.push(Statement::Enum(enum_));
                continue;
            }

            let fork = input.fork();
            if let Ok(lambda) = fork.parse() {
                input.advance_to(&fork);
                statements.push(Statement::Lambda(lambda));
                continue;
            }

            break;
        }
        Ok(Self { statements })
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

#[derive(Debug)]
pub enum L2Entry {
    Ident(Ident),
    LitStr(LitStr),
    LitChar(LitChar),
    Set(Set),
    Sequence(L1Sequence),
}

impl Parse for L2Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(Ident) {
            let ident: Ident = input.parse()?;
            Ok(Self::Ident(ident))
        } else if lookahead1.peek(LitStr) {
            let lit_str: LitStr = input.parse()?;
            Ok(Self::LitStr(lit_str))
        } else if lookahead1.peek(LitChar) {
            let lit_char: LitChar = input.parse()?;
            Ok(Self::LitChar(lit_char))
        } else if lookahead1.peek(Brace) {
            let set: Set = input.parse()?;
            Ok(Self::Set(set))
        } else if lookahead1.peek(Paren) {
            let sequence: L1Sequence = input.parse()?;
            Ok(Self::Sequence(sequence))
        } else {
            Err(lookahead1.error())
        }
    }
}

#[derive(Debug)]
pub struct L1Sequence {
    sequences: Vec<L2Sequence>,
}

impl Parse for L1Sequence {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        todo!();
    }
}

#[derive(Debug)]
pub struct L2Sequence {
    entries: Vec<L1Entry>,
}

impl Parse for L2Sequence {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let mut entries = Vec::new();
        loop {
            let fork = content.fork();
            if let Ok(entry) = fork.parse::<L1Entry>() {
                content.advance_to(&fork);
                entries.push(entry);
            } else {
                break;
            }
        }
        Ok(Self { entries })
    }
}

#[derive(Debug)]
pub struct Set {
    exclusion: bool,
    entries: Vec<SetEntry>,
}

impl Parse for Set {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);
        let exclusion;
        let mut entries = Vec::new();
        let lookahead1 = content.lookahead1();
        if lookahead1.peek(Token![!]) {
            let _: Token![!] = content.parse()?;
            exclusion = true;
        } else if lookahead1.peek(LitChar) {
            let entry: SetEntry = content.parse()?;
            entries.push(entry);
            exclusion = false;
        } else {
            return Err(lookahead1.error());
        }
        while content.peek(LitChar) {
            let entry: SetEntry = content.parse()?;
            entries.push(entry)
        }
        Ok(Self { exclusion, entries })
    }
}

#[derive(Debug)]
pub enum SetEntry {
    Single(LitChar),
    Range(LitChar, LitChar),
}

impl Parse for SetEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(Token![..]) {
            let start: LitChar = input.parse()?;
            let _: Token![..] = input.parse()?;
            let end: LitChar = input.parse()?;
            Ok(Self::Range(start, end))
        } else {
            let char: LitChar = input.parse()?;
            Ok(Self::Single(char))
        }
    }
}
