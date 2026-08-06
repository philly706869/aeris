use syn::{
    Attribute, Ident, LitChar, LitInt, LitStr, Token, Visibility, braced, bracketed, parenthesized,
    parse::{Parse, ParseStream, discouraged::Speculative},
    token::Bracket,
};

#[derive(Debug)]
pub struct ClusterAST {
    pub shards: Vec<ShardAST>,
}

impl Parse for ClusterAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut shards = Vec::new();
        while !input.is_empty() {
            shards.push(input.parse()?);
        }
        Ok(Self { shards })
    }
}

#[derive(Debug)]
pub enum ShardAST {
    Struct(StructAST),
    Enum(EnumAST),
    Lambda(LambdaAST),
}

impl Parse for ShardAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        if let Ok(struct_) = fork.parse::<StructAST>() {
            input.advance_to(&fork);
            return Ok(Self::Struct(struct_));
        }
        let fork = input.fork();
        if let Ok(enum_) = fork.parse::<EnumAST>() {
            input.advance_to(&fork);
            return Ok(Self::Enum(enum_));
        }
        let fork = input.fork();
        if let Ok(lambda) = fork.parse::<LambdaAST>() {
            input.advance_to(&fork);
            return Ok(Self::Lambda(lambda));
        }
        Err(input.error("expected struct, enum, or lambda"))
    }
}

#[derive(Debug)]
pub struct StructAST {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub trait_name: Ident,
    pub sequence: SequenceAST,
}

impl Parse for StructAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = Attribute::parse_outer(input)?;
        let vis = input.parse()?;
        input.parse::<Token![struct]>()?;
        let name = input.parse()?;
        input.parse::<Token![trait]>()?;
        let trait_name = input.parse()?;
        let sequence = input.parse()?;
        Ok(Self {
            attrs,
            vis,
            name,
            trait_name,
            sequence,
        })
    }
}

#[derive(Debug)]
pub struct EnumAST {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub trait_name: Ident,
    pub variants: Vec<VariantAST>,
}

impl Parse for EnumAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = Attribute::parse_outer(input)?;
        let vis = input.parse()?;
        input.parse::<Token![enum]>()?;
        let name = input.parse()?;
        input.parse::<Token![trait]>()?;
        let trait_name = input.parse()?;
        let content;
        braced!(content in input);
        let mut variants = Vec::new();
        while !content.is_empty() {
            variants.push(content.parse()?);
        }
        Ok(Self {
            attrs,
            vis,
            name,
            trait_name,
            variants,
        })
    }
}

#[derive(Debug)]
pub struct VariantAST {
    pub name: Ident,
    pub sequence: SequenceAST,
}

impl Parse for VariantAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let sequence = input.parse()?;
        Ok(Self { name, sequence })
    }
}

#[derive(Debug)]
pub struct LambdaAST {
    pub name: Ident,
    pub entry: EntryAST,
}

impl Parse for LambdaAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let entry = input.parse()?;
        Ok(Self { name, entry })
    }
}

#[derive(Debug)]
pub enum SequenceAST {
    Object(ObjectSequenceAST),
    Tuple(TupleSequenceAST),
}

impl Parse for SequenceAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        if let Ok(object) = fork.parse::<ObjectSequenceAST>() {
            input.advance_to(&fork);
            return Ok(Self::Object(object));
        }
        let fork = input.fork();
        if let Ok(tuple) = fork.parse::<TupleSequenceAST>() {
            input.advance_to(&fork);
            return Ok(Self::Tuple(tuple));
        }
        Err(input.error("expected object or tuple sequence"))
    }
}

#[derive(Debug)]
pub struct ObjectSequenceAST {
    pub fields: Vec<ObjectFieldAST>,
}

impl Parse for ObjectSequenceAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);
        let mut fields = Vec::new();
        while !content.is_empty() {
            fields.push(content.parse()?);
        }
        Ok(Self { fields })
    }
}

#[derive(Debug)]
pub struct ObjectFieldAST {
    pub name: Ident,
    pub entry: EntryAST,
}

impl Parse for ObjectFieldAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let entry = input.parse()?;
        Ok(Self { name, entry })
    }
}

#[derive(Debug)]
pub struct TupleSequenceAST {
    pub fields: Vec<TupleFieldAST>,
}

impl Parse for TupleSequenceAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let mut fields = Vec::new();
        while !content.is_empty() {
            fields.push(content.parse()?);
        }
        Ok(Self { fields })
    }
}

#[derive(Debug)]
pub struct TupleFieldAST {
    pub entry: EntryAST,
}

impl Parse for TupleFieldAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let entry = input.parse()?;
        Ok(Self { entry })
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
        let fork = input.fork();
        let quantifier = fork.parse().ok().map(|quantifier| {
            input.advance_to(&fork);
            quantifier
        });
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
        if input.peek(Ident) {
            return Ok(Self::Shard(input.parse()?));
        }
        if input.peek(LitStr) {
            return Ok(Self::LitStr(input.parse()?));
        }
        if input.peek(LitChar) {
            return Ok(Self::LitChar(input.parse()?));
        }
        let fork = input.fork();
        if let Ok(set) = fork.parse::<SetAST>() {
            input.advance_to(&fork);
            return Ok(Self::Set(set));
        }
        let fork = input.fork();
        if let Ok(term) = fork.parse::<TermAST>() {
            input.advance_to(&fork);
            return Ok(Self::Term(term));
        }
        Err(input.error("expected shard, literal string, literal char, set, or term"))
    }
}

#[derive(Debug)]
pub struct QuantifierAST {
    pub repeater: RepeaterAST,
    pub lazy: bool,
}

impl Parse for QuantifierAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let repeater = input.parse()?;
        let lazy = if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            true
        } else {
            false
        };
        Ok(Self { repeater, lazy })
    }
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

impl Parse for RepeaterAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(Token![+]) {
            input.parse::<Token![+]>()?;
            Ok(RepeaterAST::Plus)
        } else if lookahead1.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            Ok(RepeaterAST::Star)
        } else if lookahead1.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            Ok(RepeaterAST::Option)
        } else if lookahead1.peek(Bracket) {
            let content;
            bracketed!(content in input);
            let lookahead1 = content.lookahead1();
            if lookahead1.peek(LitInt) {
                if content.peek2(Token![,]) {
                    if content.peek3(LitInt) {
                        let min = content.parse()?;
                        content.parse::<Token![,]>()?;
                        let max = content.parse()?;
                        Ok(RepeaterAST::Range(min, max))
                    } else {
                        let min = content.parse()?;
                        content.parse::<Token![,]>()?;
                        Ok(RepeaterAST::Min(min))
                    }
                } else {
                    let val = content.parse()?;
                    Ok(RepeaterAST::Val(val))
                }
            } else if lookahead1.peek(Token![,]) {
                content.parse::<Token![,]>()?;
                let max = content.parse()?;
                Ok(RepeaterAST::Max(max))
            } else {
                return Err(lookahead1.error());
            }
        } else {
            Err(lookahead1.error())
        }
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
        let exclusion = if content.peek(Token![!]) {
            content.parse::<Token![!]>()?;
            true
        } else {
            false
        };
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
        if input.peek(Token![..]) {
            input.parse::<Token![..]>()?;
            let end = input.parse()?;
            Ok(Self::Range(start, end))
        } else {
            Ok(Self::Single(start))
        }
    }
}

#[derive(Debug)]
pub struct TermAST {
    pub alts: Vec<TermAltAST>,
}

impl Parse for TermAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let mut alts = Vec::new();
        while !content.is_empty() {
            alts.push(content.parse()?);
        }
        Ok(Self { alts })
    }
}

#[derive(Debug)]
pub struct TermAltAST {
    pub entries: Vec<EntryAST>,
}

impl Parse for TermAltAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![|]>()?;
        let mut entries = Vec::new();
        while !input.is_empty() {
            let fork = input.fork();
            if let Ok(entry) = fork.parse() {
                input.advance_to(&fork);
                entries.push(entry);
                continue;
            }
            break;
        }
        Ok(Self { entries })
    }
}
