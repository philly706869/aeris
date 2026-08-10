use syn::{
    Attribute, Ident, Token, Visibility, braced,
    parse::{Parse, ParseStream},
    token::Brace,
};

mod keyword {
    syn::custom_keyword!(Shard);
}

pub struct Shard {
    pub vis: Visibility,
    pub keyword: Token![mod],
    pub ident: Ident,
    pub brace: Brace,
    pub attrs: Vec<Attribute>,
    pub binding: Binding,
}

impl Parse for Shard {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            vis: input.parse()?,
            keyword: input.parse()?,
            ident: input.parse()?,
            brace: braced!(content in input),
            attrs: Attribute::parse_outer(&content)?,
            binding: content.parse()?,
        })
    }
}

pub enum Binding {
    Struct(StructBinding),
    Enum(EnumBinding),
    Forward(ForwardBinding),
}

impl Parse for Binding {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        Ok(if lookahead.peek(Token![struct]) {
            Self::Struct(input.parse()?)
        } else if lookahead.peek(Token![enum]) {
            Self::Enum(input.parse()?)
        } else if lookahead.peek(Ident) {
            Self::Forward(input.parse()?)
        } else {
            return Err(lookahead.error());
        })
    }
}

pub struct StructBinding {
    pub keyword: Token![struct],
    pub ident: keyword::Shard,
}

impl Parse for StructBinding {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            keyword: input.parse()?,
            ident: input.parse()?,
        })
    }
}

pub struct EnumBinding {
    pub keyword: Token![struct],
    pub ident: keyword::Shard,
}

impl Parse for EnumBinding {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            keyword: input.parse()?,
            ident: input.parse()?,
        })
    }
}

pub struct ForwardBinding {
    pub keyword: keyword::Shard,
    pub bang: Token![!],
    pub brace: Brace,
}

impl Parse for ForwardBinding {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            keyword: input.parse()?,
            bang: input.parse()?,
            brace: braced!(content in input),
        })
    }
}
