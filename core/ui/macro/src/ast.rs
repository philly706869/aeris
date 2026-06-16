use syn::{
    Attribute, Ident, LitChar, LitInt, LitStr, Token, Visibility, braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Bracket, Paren},
};

#[derive(Debug)]
pub struct Statements(pub Vec<Statement>);

impl Parse for Statements {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut statements = Vec::new();
        while !input.is_empty() {
            statements.push(input.parse()?);
        }
        Ok(Self(statements))
    }
}

#[derive(Debug)]
pub enum Statement {
    Struct(Struct),
    Enum(Enum),
    Lambda(Lambda),
}

impl Parse for Statement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) {
            return Ok(Self::Lambda(input.parse()?));
        }

        let fork = input.fork();
        Attribute::parse_outer(&fork)?;
        fork.parse::<Visibility>()?;
        let lookahead1 = fork.lookahead1();
        if lookahead1.peek(Token![struct]) {
            Ok(Self::Struct(input.parse()?))
        } else if lookahead1.peek(Token![enum]) {
            Ok(Self::Enum(input.parse()?))
        } else {
            Err(lookahead1.error())
        }
    }
}

#[derive(Debug)]
pub struct Struct {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub keyword: Token![struct],
    pub name: Ident,
    pub trait_keyword: Token![trait],
    pub trait_name: Ident,
    pub body: Body,
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: Attribute::parse_outer(input)?,
            vis: input.parse()?,
            keyword: input.parse()?,
            name: input.parse()?,
            trait_keyword: input.parse()?,
            trait_name: input.parse()?,
            body: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Enum {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub keyword: Token![enum],
    pub name: Ident,
    pub trait_keyword: Token![trait],
    pub trait_name: Ident,
    pub brace: Brace,
    pub variants: Vec<Variant>,
}

impl Parse for Enum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            attrs: Attribute::parse_outer(input)?,
            vis: input.parse()?,
            keyword: input.parse()?,
            name: input.parse()?,
            trait_keyword: input.parse()?,
            trait_name: input.parse()?,
            brace: braced!(content in input),
            variants: {
                let mut variants = Vec::new();
                while !content.is_empty() {
                    variants.push(content.parse()?);
                }
                variants
            },
        })
    }
}

#[derive(Debug)]
pub struct Variant {
    pub name: Ident,
    pub body: Body,
}

impl Parse for Variant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            body: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Lambda {
    pub name: Ident,
    pub sequence: Sequence,
}

impl Parse for Lambda {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            sequence: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub enum Body {
    NamedBody(NamedBody),
    TupleBody(TupleBody),
}

impl Parse for Body {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(Brace) {
            Ok(Self::NamedBody(input.parse()?))
        } else if lookahead1.peek(Paren) {
            Ok(Self::TupleBody(input.parse()?))
        } else {
            Err(lookahead1.error())
        }
    }
}

#[derive(Debug)]
pub struct NamedBody {
    pub brace: Brace,
    pub entries: Vec<NamedEntry>,
}

impl Parse for NamedBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            brace: braced!(content in input),
            entries: {
                let mut entries = Vec::new();
                while !content.is_empty() {
                    entries.push(content.parse()?);
                }
                entries
            },
        })
    }
}

#[derive(Debug)]
pub struct TupleBody {
    pub paren: Paren,
    pub entries: Vec<L1Entry>,
}

impl Parse for TupleBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in input),
            entries: {
                let mut entries = Vec::new();
                while !content.is_empty() {
                    entries.push(content.parse()?);
                }
                entries
            },
        })
    }
}

#[derive(Debug)]
pub struct NamedEntry {
    pub name: EntryName,
    _colon: Token![:],
    pub entry: L1Entry,
}

impl Parse for NamedEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _colon: input.parse()?,
            entry: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub enum EntryName {
    Single(Ident),
    Tuple(Ident),
}

impl Parse for EntryName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead1 = input.lookahead1();
        if lookahead1.peek(Ident) {
            Ok(Self::Single(input.parse()?))
        } else if lookahead1.peek(Paren) {
            let content;
            parenthesized!(content in input);
            Ok(Self::Tuple(content.parse()?))
        } else {
            return Err(lookahead1.error());
        }
    }
}

#[derive(Debug)]
pub struct L1Entry {
    pub entry: L2Entry,
    pub quantifier: Quantifier,
}

impl Parse for L1Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            entry: input.parse()?,
            quantifier: input.parse()?,
        })
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
    LazyPlus,
    LazyStar,
    LazyOption,
    LazyVal(LitInt),
    LazyMin(LitInt),
    LazyMax(LitInt),
    LazyRange(LitInt, LitInt),
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
                        content.parse::<Token![,]>()?;
                        let max: LitInt = content.parse()?;
                        Self::Range(min, max)
                    } else {
                        let min: LitInt = content.parse()?;
                        content.parse::<Token![,]>()?;
                        Self::Min(min)
                    }
                } else {
                    let val: LitInt = content.parse()?;
                    Self::Val(val)
                }
            } else if lookahead1.peek(Token![,]) {
                content.parse::<Token![,]>()?;
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
    Sequence(Sequence),
}

impl Parse for L2Entry {
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
            Ok(Self::Sequence(input.parse()?))
        } else {
            Err(lookahead1.error())
        }
    }
}

#[derive(Debug)]
pub struct Sequence {
    _paren: Paren,
    _pipe: Option<Token![|]>,
    pub alts: Vec<Vec<L1Entry>>,
}

impl Parse for Sequence {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _paren: parenthesized!(content in input),
            _pipe: content.parse()?,
            alts: {
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
                alts
            },
        })
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
            input.parse::<Token![..]>()?;
            let end: LitChar = input.parse()?;
            Ok(Self::Range(start, end))
        } else {
            let char: LitChar = input.parse()?;
            Ok(Self::Single(char))
        }
    }
}
