mod old {
    use syn::{
        Attribute, Ident, Token, Visibility, braced, bracketed, parenthesized,
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        token::{Brace, Bracket, Paren},
    };

    mod keyword {
        mod private {
            syn::custom_keyword!(Shard);
            syn::custom_keyword!(x);
        }
        pub use private::Shard as ShardToken;
        pub use private::x as XToken;
    }

    use keyword::{ShardToken, XToken};

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
        pub ident: ShardToken,
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
        pub entry: Entry,
    }

    impl Parse for NamedStructField {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                ident: input.parse()?,
                colon: input.parse()?,
                entry: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct UnnamedStructBody {
        pub paren: Paren,
        pub fields: Punctuated<UnnamedStructField, Token![,]>,
    }

    impl Parse for UnnamedStructBody {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                paren: parenthesized!(content in input),
                fields: Punctuated::parse_terminated(&content)?,
            })
        }
    }

    #[derive(Debug)]
    pub struct UnnamedStructField {
        pub entry: Entry,
    }

    impl Parse for UnnamedStructField {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                entry: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct EnumBinding {
        pub keyword: Token![enum],
        pub ident: ShardToken,
        pub brace: Brace,
        pub variants: Punctuated<EnumVariant, Token![,]>,
    }

    impl Parse for EnumBinding {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                keyword: input.parse()?,
                ident: input.parse()?,
                brace: braced!(content in input),
                variants: Punctuated::parse_terminated(&content)?,
            })
        }
    }

    #[derive(Debug)]
    pub struct EnumVariant {
        pub ident: Ident,
        pub body: EnumVariantBody,
    }

    impl Parse for EnumVariant {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                ident: input.parse()?,
                body: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub enum EnumVariantBody {
        Named(NamedEnumVariantBody),
        Unnamed(UnnamedEnumVariantBody),
    }

    impl Parse for EnumVariantBody {
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
    pub struct NamedEnumVariantBody {
        pub brace: Brace,
        pub fields: Punctuated<NamedEnumVariantField, Token![,]>,
    }

    impl Parse for NamedEnumVariantBody {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                brace: braced!(content in input),
                fields: Punctuated::parse_terminated(&content)?,
            })
        }
    }

    #[derive(Debug)]
    pub struct NamedEnumVariantField {
        pub ident: Ident,
        pub colon: Token![:],
        pub entry: Entry,
    }

    impl Parse for NamedEnumVariantField {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                ident: input.parse()?,
                colon: input.parse()?,
                entry: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct UnnamedEnumVariantBody {
        pub paren: Paren,
        pub fields: Punctuated<UnnamedEnumVariantField, Token![,]>,
    }

    impl Parse for UnnamedEnumVariantBody {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                paren: parenthesized!(content in input),
                fields: Punctuated::parse_terminated(&content)?,
            })
        }
    }

    #[derive(Debug)]
    pub struct UnnamedEnumVariantField {
        pub entry: Entry,
    }

    impl Parse for UnnamedEnumVariantField {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                entry: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct ForwardBinding {
        pub keyword: ShardToken,
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

    #[derive(Debug)]
    pub enum Entry {
        Shard(ShardEntry),
        Terminal(TerminalEntry),
    }

    impl Parse for Entry {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(if input.peek(XToken) && input.peek2(Token![!]) {
                Self::Terminal(input.parse()?)
            } else if input.peek(Ident) {
                Self::Shard(input.parse()?)
            } else {
                return Err(input.error("expected Shard Entry"));
            })
        }
    }

    #[derive(Debug)]
    pub struct ShardEntry {
        pub ident: Ident,
        pub generic: Option<ShardEntryGeneric>,
    }

    impl Parse for ShardEntry {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                ident: input.parse()?,
                generic: if input.peek(Token![<]) {
                    Some(input.parse()?)
                } else {
                    None
                },
            })
        }
    }

    #[derive(Debug)]
    pub struct ShardEntryGeneric {
        pub lt: Token![<],
        pub entries: Punctuated<Entry, Token![,]>,
        pub gt: Token![>],
    }

    impl Parse for ShardEntryGeneric {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                lt: input.parse()?,
                entries: Punctuated::parse_separated_nonempty(input)?,
                gt: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct TerminalEntry {
        pub x: XToken,
        pub bang: Token![!],
        pub expression: TerminalExpression,
    }

    impl Parse for TerminalEntry {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                x: input.parse()?,
                bang: input.parse()?,
                expression: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub enum TerminalExpression {
        Single(SingleExpression),
        Multi(MultiExpression),
    }

    impl Parse for TerminalExpression {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let lookahead = input.lookahead1();
            Ok(if lookahead.peek(Bracket) {
                Self::Single(input.parse()?)
            } else if lookahead.peek(Brace) {
                Self::Single(input.parse()?)
            } else {
                return Err(lookahead.error());
            })
        }
    }

    #[derive(Debug)]
    pub struct SingleExpression {
        pub bracket: Bracket,
    }

    impl Parse for SingleExpression {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                bracket: bracketed!(content in input),
            })
        }
    }

    #[derive(Debug)]
    pub struct MultiExpression {
        pub brace: Brace,
        pub items: Vec<MultiExpressionItem>,
    }

    impl Parse for MultiExpression {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                brace: braced!(content in input),
                items: todo!(),
            })
        }
    }

    #[derive(Debug)]
    pub struct MultiExpressionItem {
        pub pipe: Token![|],
        pub sequence: Sequence,
    }

    impl Parse for MultiExpressionItem {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                pipe: input.parse()?,
                sequence: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct Sequence {}

    impl Parse for Sequence {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {})
        }
    }
}
