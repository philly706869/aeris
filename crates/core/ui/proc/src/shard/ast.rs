use syn::{
    Attribute, Ident, Token, Visibility, braced, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Brace, Paren},
};

mod keyword {
    syn::custom_keyword!(Shard);
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct StructBinding {
    pub keyword: Token![struct],
    pub ident: keyword::Shard,
    pub body: StructBody,
}

impl Parse for StructBinding {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            keyword: input.parse()?,
            ident: input.parse()?,
            body: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub enum StructBody {
    Named(NamedStructBody),
    Unnamed(UnnamedStructBody),
}

impl Parse for StructBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        Ok(if lookahead.peek(Brace) {
            Self::Named(input.parse()?)
        } else if lookahead.peek(Paren) {
            Self::Unnamed(input.parse()?)
        } else {
            return Err(lookahead.error());
        })
    }
}

#[derive(Debug)]
pub struct NamedStructBody {
    pub brace: Brace,
    pub fields: Punctuated<NamedStructField, Token![,]>,
}

impl Parse for NamedStructBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            brace: braced!(content in input),
            fields: Punctuated::parse_terminated(&content)?,
        })
    }
}

#[derive(Debug)]
pub struct NamedStructField {
    pub ident: Ident,
    pub colon: Token![:],
}

impl Parse for NamedStructField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            colon: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct UnnamedStructBody {
    pub paren: Paren,
}

impl Parse for UnnamedStructBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            paren: parenthesized!(content in input),
        })
    }
}

pub struct UnnamedStructField {}

#[derive(Debug)]
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

#[derive(Debug)]
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
