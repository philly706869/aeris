use syn::{
    Attribute, Ident, Token, Visibility, braced,
    parse::{Parse, ParseStream},
    token::Brace,
};

#[derive(Debug)]
pub struct Shard {
    pub vis: Visibility,
    pub keyword: Token![mod],
    pub ident: Ident,
    pub brace: Brace,
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
            binding: content.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Binding {
    pub attrs: Vec<Attribute>,
    pub shape: Shape,
}

impl Parse for Binding {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: Attribute::parse_outer(input)?,
            shape: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub enum Shape {
    Struct(StructShape),
    Enum(EnumShape),
    Forward(ForwardShape),
}

impl Parse for Shape {
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
pub struct StructShape {
    pub keyword: Token![struct],
    pub ident: Ident,
    pub semi: Option<Token![;]>,
}

impl Parse for StructShape {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            keyword: input.parse()?,
            ident: input.parse()?,
            semi: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct EnumShape {
    pub keyword: Token![enum],
    pub ident: Ident,
    pub brace: Brace,
}

impl Parse for EnumShape {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            keyword: input.parse()?,
            ident: input.parse()?,
            brace: braced!(content in input),
        })
    }
}

#[derive(Debug)]
pub struct ForwardShape {
    pub keyword: Ident,
    pub bang: Token![!],
    pub brace: Brace,
}

impl Parse for ForwardShape {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            keyword: input.parse()?,
            bang: input.parse()?,
            brace: braced!(content in input),
        })
    }
}
