use syn::{
    Attribute, Ident, LitChar, LitInt, LitStr, Token, Visibility, braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Bracket, Paren},
};

#[derive(Debug)]
pub struct Cluster {
    pub shards: Vec<Shard>,
    pub lambdas: Vec<Lambda>,
}

impl Parse for Cluster {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut shards = Vec::new();
        let mut lambdas = Vec::new();
        while !input.is_empty() {
            if input.peek(Ident) {
                let name = input.parse()?;
                let entry = input.parse()?;
                lambdas.push(Lambda { name, entry });
                continue;
            }

            let attrs = Attribute::parse_outer(input)?;
            let vis = input.parse()?;
            let lookahead1 = input.lookahead1();
            let is_struct = if lookahead1.peek(Token![struct]) {
                input.parse::<Token![struct]>()?;
                true
            } else if lookahead1.peek(Token![enum]) {
                input.parse::<Token![enum]>()?;
                false
            } else {
                return Err(lookahead1.error());
            };

            let name = input.parse()?;
            input.parse::<Token![trait]>()?;
            let trait_name = input.parse()?;
            let shape = if is_struct {
                Shape::Struct(input.parse()?)
            } else {
                let content;
                braced!(content in input);
                let mut variants = Vec::new();
                while !content.is_empty() {
                    let name = content.parse()?;
                    let sequence = content.parse()?;
                    variants.push((name, sequence));
                }
                Shape::Enum(variants)
            };

            shards.push(Shard {
                attrs,
                vis,
                name,
                trait_name,
                shape,
            });
        }
        Ok(Self { shards, lambdas })
    }
}

#[derive(Debug)]
pub struct Shard {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub trait_name: Ident,
    pub shape: Shape,
}

#[derive(Debug)]
pub enum Shape {
    Struct(Sequence),
    Enum(Vec<(Ident, Sequence)>),
}

#[derive(Debug)]
pub struct Lambda {
    pub name: Ident,
    pub entry: Entry,
}

#[derive(Debug)]
pub enum Sequence {
    Object(Vec<(Ident, Entry)>),
    Tuple(Vec<Entry>),
}

impl Parse for Sequence {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(Brace) {
            let content;
            braced!(content in input);
            let mut entries = Vec::new();
            while !content.is_empty() {
                let name = content.parse()?;
                content.parse::<Token![:]>()?;
                let entry = content.parse()?;
                entries.push((name, entry));
            }
            Ok(Self::Object(entries))
        } else if lookahead1.peek(Paren) {
            let content;
            parenthesized!(content in input);
            let mut entries = Vec::new();
            while !content.is_empty() {
                let entry = content.parse()?;
                entries.push(entry);
            }
            Ok(Self::Tuple(entries))
        } else {
            Err(lookahead1.error())
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    pub factor: Factor,
    pub quantifier: Option<Quantifier>,
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let factor = input.parse()?;
        let quantifier = 'quant: {
            let repeater = if input.parse::<Option<Token![+]>>()?.is_some() {
                Repeater::Plus
            } else if input.parse::<Option<Token![*]>>()?.is_some() {
                Repeater::Star
            } else if input.parse::<Option<Token![?]>>()?.is_some() {
                Repeater::Option
            } else if input.peek(Bracket) {
                let content;
                bracketed!(content in input);
                let lookahead1 = content.lookahead1();
                if lookahead1.peek(LitInt) {
                    if content.peek2(Token![,]) {
                        if content.peek3(LitInt) {
                            let min: LitInt = content.parse()?;
                            content.parse::<Token![,]>()?;
                            let max: LitInt = content.parse()?;
                            Repeater::Range(min, max)
                        } else {
                            let min: LitInt = content.parse()?;
                            content.parse::<Token![,]>()?;
                            Repeater::Min(min)
                        }
                    } else {
                        let val: LitInt = content.parse()?;
                        Repeater::Val(val)
                    }
                } else if lookahead1.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                    let max: LitInt = content.parse()?;
                    Repeater::Max(max)
                } else {
                    return Err(lookahead1.error());
                }
            } else {
                break 'quant None;
            };
            let lazy = input.parse::<Option<Token![?]>>()?.is_some();
            Some(Quantifier { repeater, lazy })
        };

        Ok(Self { factor, quantifier })
    }
}

#[derive(Debug)]
pub enum Factor {
    Shard(Ident),
    LitStr(LitStr),
    LitChar(LitChar),
    Set(Set),
    Term(Term),
}

impl Parse for Factor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(Ident) {
            Ok(Self::Shard(input.parse()?))
        } else if lookahead1.peek(LitStr) {
            Ok(Self::LitStr(input.parse()?))
        } else if lookahead1.peek(LitChar) {
            Ok(Self::LitChar(input.parse()?))
        } else if lookahead1.peek(Brace) {
            Ok(Self::Set(input.parse()?))
        } else if lookahead1.peek(Paren) {
            Ok(Self::Term(input.parse()?))
        } else {
            Err(lookahead1.error())
        }
    }
}

#[derive(Debug)]
pub struct Quantifier {
    pub repeater: Repeater,
    pub lazy: bool,
}

#[derive(Debug)]
pub enum Repeater {
    Plus,
    Star,
    Option,
    Val(LitInt),
    Min(LitInt),
    Max(LitInt),
    Range(LitInt, LitInt),
}

#[derive(Debug)]
pub struct Term {
    pub alts: Vec<Vec<Entry>>,
}

impl Parse for Term {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        content.parse::<Option<Token![|]>>()?;
        let mut alts = Vec::new();
        let mut entries = Vec::new();
        while !content.is_empty() {
            if content.parse::<Option<Token![|]>>()?.is_some() {
                alts.push(entries);
                entries = Vec::new();
            } else {
                entries.push(content.parse()?);
            }
        }
        Ok(Self { alts })
    }
}

#[derive(Debug)]
pub struct Set {
    pub exclusion: bool,
    pub entries: Vec<SetEntry>,
}

impl Parse for Set {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);
        let exclusion = content.parse::<Option<Token![!]>>()?.is_some();
        let mut entries = Vec::new();
        while !content.is_empty() {
            entries.push(content.parse()?)
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
        let start = input.parse()?;
        if input.parse::<Option<Token![..]>>()?.is_some() {
            let end = input.parse()?;
            Ok(Self::Range(start, end))
        } else {
            Ok(Self::Single(start))
        }
    }
}
