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
            if lookahead1.peek(Token![struct]) {
                let keyword = input.parse()?;
                let name = input.parse()?;
                let trait_keyword = input.parse()?;
                let trait_name = input.parse()?;
                let sequence = input.parse()?;
                shards.push(Shard {
                    attrs,
                    vis,
                    name,
                    trait_keyword,
                    trait_name,
                    shape: Shape::Struct { keyword, sequence },
                });
            } else if lookahead1.peek(Token![enum]) {
                let keyword = input.parse()?;
                let name = input.parse()?;
                let trait_keyword = input.parse()?;
                let trait_name = input.parse()?;
                let content;
                braced!(content in input);
                let mut variants = Vec::new();
                while !content.is_empty() {
                    let name = content.parse()?;
                    let sequence = content.parse()?;
                    variants.push((name, sequence));
                }
                shards.push(Shard {
                    attrs,
                    vis,
                    name,
                    trait_keyword,
                    trait_name,
                    shape: Shape::Enum { keyword, variants },
                });
            } else {
                return Err(lookahead1.error());
            }
        }
        Ok(Self { shards, lambdas })
    }
}

#[derive(Debug)]
pub struct Shard {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub trait_keyword: Token![trait],
    pub trait_name: Ident,
    pub shape: Shape,
}

#[derive(Debug)]
pub enum Shape {
    Struct {
        keyword: Token![struct],
        sequence: Sequence,
    },
    Enum {
        keyword: Token![enum],
        variants: Vec<(Ident, Sequence)>,
    },
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
    Ident(Ident),
    LitStr(LitStr),
    LitChar(LitChar),
    Set(Set),
    Term(Term),
}

impl Parse for Factor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(Ident) {
            Ok(Self::Ident(input.parse()?))
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
    repeater: Repeater,
    lazy: bool,
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
        'alts: loop {
            let mut entries = Vec::new();
            loop {
                entries.push(content.parse()?);
                if content.peek(Token![|]) {
                    alts.push(entries);
                    content.parse::<Token![|]>()?;
                    continue 'alts;
                }
                if content.is_empty() {
                    alts.push(entries);
                    break 'alts;
                }
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
        let exclusion;
        let mut entries = Vec::new();
        let lookahead1 = content.lookahead1();
        if lookahead1.peek(Token![!]) {
            content.parse::<Token![!]>()?;
            exclusion = true;
        } else if lookahead1.peek(LitChar) {
            entries.push(content.parse()?);
            exclusion = false;
        } else {
            return Err(lookahead1.error());
        }
        while content.peek(LitChar) {
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
        if input.peek2(Token![..]) {
            let start: LitChar = input.parse()?;
            input.parse::<Token![..]>()?;
            let end: LitChar = input.parse()?;
            Ok(Self::Range(start, end))
        } else {
            let char: LitChar = input.parse()?;
            Ok(Self::Single(char))
        }
    }
}
