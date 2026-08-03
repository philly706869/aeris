use syn::{
    Attribute, Ident, LitChar, LitInt, LitStr, Token, Visibility, braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Bracket, Paren},
};

#[derive(Debug)]
pub struct ClusterAST {
    pub shards: Vec<ShardAST>,
    pub lambdas: Vec<LambdaAST>,
}

impl Parse for ClusterAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut shards = Vec::new();
        let mut lambdas = Vec::new();
        while !input.is_empty() {
            if input.peek(Ident) {
                let name = input.parse()?;
                let entry = input.parse()?;
                lambdas.push(LambdaAST { name, entry });
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
                ShapeAST::Struct(input.parse()?)
            } else {
                let content;
                braced!(content in input);
                let mut variants = Vec::new();
                while !content.is_empty() {
                    let name = content.parse()?;
                    let sequence = content.parse()?;
                    variants.push((name, sequence));
                }
                ShapeAST::Enum(variants)
            };

            shards.push(ShardAST {
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
pub struct ShardAST {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub trait_name: Ident,
    pub shape: ShapeAST,
}

#[derive(Debug)]
pub enum ShapeAST {
    Struct(SequenceAST),
    Enum(Vec<(Ident, SequenceAST)>),
}

#[derive(Debug)]
pub struct LambdaAST {
    pub name: Ident,
    pub entry: EntryAST,
}

#[derive(Debug)]
pub enum SequenceAST {
    Object(Vec<(Ident, EntryAST)>),
    Tuple(Vec<EntryAST>),
}

impl Parse for SequenceAST {
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
pub struct EntryAST {
    pub factor: FactorAST,
    pub quantifier: Option<QuantifierAST>,
}

impl Parse for EntryAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let factor = input.parse()?;
        let quantifier = 'quant: {
            let repeater = if input.parse::<Option<Token![+]>>()?.is_some() {
                RepeaterAST::Plus
            } else if input.parse::<Option<Token![*]>>()?.is_some() {
                RepeaterAST::Star
            } else if input.parse::<Option<Token![?]>>()?.is_some() {
                RepeaterAST::Option
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
                            RepeaterAST::Range(min, max)
                        } else {
                            let min: LitInt = content.parse()?;
                            content.parse::<Token![,]>()?;
                            RepeaterAST::Min(min)
                        }
                    } else {
                        let val: LitInt = content.parse()?;
                        RepeaterAST::Val(val)
                    }
                } else if lookahead1.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                    let max: LitInt = content.parse()?;
                    RepeaterAST::Max(max)
                } else {
                    return Err(lookahead1.error());
                }
            } else {
                break 'quant None;
            };
            let lazy = input.parse::<Option<Token![?]>>()?.is_some();
            Some(QuantifierAST { repeater, lazy })
        };

        Ok(Self { factor, quantifier })
    }
}

#[derive(Debug)]
pub enum FactorAST {
    Shard(Ident),
    LitStr(LitStr),
    LitChar(LitChar),
    Set(SetAST),
    Term(TermAST),
}

impl Parse for FactorAST {
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
pub struct QuantifierAST {
    pub repeater: RepeaterAST,
    pub lazy: bool,
}

#[derive(Debug)]
pub enum RepeaterAST {
    Plus,
    Star,
    Option,
    Val(LitInt),
    Min(LitInt),
    Max(LitInt),
    Range(LitInt, LitInt),
}

#[derive(Debug)]
pub struct TermAST {
    pub alts: Vec<Vec<EntryAST>>,
}

impl Parse for TermAST {
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
        if !entries.is_empty() {
            alts.push(entries);
        }
        Ok(Self { alts })
    }
}

#[derive(Debug)]
pub struct SetAST {
    pub exclusion: bool,
    pub entries: Vec<SetEntryAST>,
}

impl Parse for SetAST {
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
pub enum SetEntryAST {
    Single(LitChar),
    Range(LitChar, LitChar),
}

impl Parse for SetEntryAST {
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
