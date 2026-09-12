use syn::{
    Ident, Token, Visibility, braced, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Brace, Paren},
};

#[derive(Debug)]
pub enum Shard {
    Struct(StructShard),
    Enum(EnumShard),
    Type(TypeShard),
}

impl Parse for Shard {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        fork.parse::<Visibility>()?;
        let lookahead = fork.lookahead1();
        if lookahead.peek(Token![struct]) {
            Ok(Self::Struct(input.parse()?))
        } else if lookahead.peek(Token![enum]) {
            Ok(Self::Enum(input.parse()?))
        } else if lookahead.peek(Token![type]) {
            Ok(Self::Type(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
pub struct StructShard {
    pub vis: Visibility,
    pub struct_token: Token![struct],
    pub ident: Ident,
    pub params: Params,
    pub brace: Brace,
    pub fields: Punctuated<Field, Token![,]>,
}

impl Parse for StructShard {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            vis: input.parse()?,
            struct_token: input.parse()?,
            ident: input.parse()?,
            params: input.parse()?,
            brace: braced!(content in input),
            fields: Punctuated::parse_terminated(&content)?,
        })
    }
}

#[derive(Debug)]
pub struct EnumShard {
    pub vis: Visibility,
    pub enum_token: Token![enum],
    pub ident: Ident,
    pub params: Params,
    pub brace: Brace,
    pub variants: Punctuated<Variant, Token![,]>,
}

impl Parse for EnumShard {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            vis: input.parse()?,
            enum_token: input.parse()?,
            ident: input.parse()?,
            params: input.parse()?,
            brace: braced!(content in input),
            variants: Punctuated::parse_terminated(&content)?,
        })
    }
}

#[derive(Debug)]
pub struct TypeShard {
    pub vis: Visibility,
    pub type_token: Token![type],
    pub ident: Ident,
    pub params: Params,
    pub eq_token: Token![=],
    pub x_expr: rust_expr::XExpr,
    pub semi_token: Token![;],
}

impl Parse for TypeShard {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            vis: input.parse()?,
            type_token: input.parse()?,
            ident: input.parse()?,
            params: input.parse()?,
            eq_token: input.parse()?,
            x_expr: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Field {
    pub ident: Ident,
    pub colon_token: Token![:],
    pub expr: rust_expr::Expr,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            colon_token: input.parse()?,
            expr: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Variant {
    pub ident: Ident,
    pub paren: Paren,
    pub shard_expr: rust_expr::ShardExpr,
}

impl Parse for Variant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            ident: input.parse()?,
            paren: parenthesized!(content in input),
            shard_expr: content.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Params {
    pub lt_token: Option<Token![<]>,
    pub idents: Punctuated<Ident, Token![,]>,
    pub gt_token: Option<Token![>]>,
}

impl Parse for Params {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![<]) {
            Ok(Self {
                lt_token: Some(input.parse()?),
                idents: Punctuated::parse_separated_nonempty(input)?,
                gt_token: Some(input.parse()?),
            })
        } else {
            Ok(Self {
                lt_token: None,
                idents: Punctuated::default(),
                gt_token: None,
            })
        }
    }
}

pub mod rust_expr {
    use syn::{
        Ident, LitInt, Token, braced, bracketed, parenthesized,
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        token::{Brace, Bracket, Paren},
    };

    use super::prim_expr;

    pub mod keyword {
        syn::custom_keyword!(x);
        syn::custom_keyword!(xbox);
        syn::custom_keyword!(xopt);
        syn::custom_keyword!(xvec);
        syn::custom_keyword!(xlopt);
        syn::custom_keyword!(xlvec);
    }

    #[derive(Debug)]
    pub enum Expr {
        X(XExpr),
        XBox(XBoxExpr),
        XOpt(XOptExpr),
        XVec(XVecExpr),
        XLOpt(XLOptExpr),
        XLVec(XLVecExpr),
        Tuple(TupleExpr),
        Shard(ShardExpr),
    }

    impl Parse for Expr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(keyword::x) {
                Ok(Self::X(input.parse()?))
            } else if lookahead.peek(keyword::xbox) {
                Ok(Self::XBox(input.parse()?))
            } else if lookahead.peek(keyword::xopt) {
                Ok(Self::XOpt(input.parse()?))
            } else if lookahead.peek(keyword::xvec) {
                Ok(Self::XVec(input.parse()?))
            } else if lookahead.peek(keyword::xlopt) {
                Ok(Self::XLOpt(input.parse()?))
            } else if lookahead.peek(keyword::xlvec) {
                Ok(Self::XLVec(input.parse()?))
            } else if lookahead.peek(Paren) {
                Ok(Self::Tuple(input.parse()?))
            } else if lookahead.peek(Ident) {
                Ok(Self::Shard(input.parse()?))
            } else {
                Err(lookahead.error())
            }
        }
    }

    #[derive(Debug)]
    pub struct XExpr {
        pub x_token: keyword::x,
        pub bang_token: Token![!],
        pub brace: Brace,
        pub sequence: prim_expr::Sequence,
    }

    impl Parse for XExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                x_token: input.parse()?,
                bang_token: input.parse()?,
                brace: braced!(content in input),
                sequence: content.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct XBoxExpr {
        pub xbox_token: keyword::xbox,
        pub bang_token: Token![!],
        pub bracket: Bracket,
        pub expr: Box<Expr>,
    }

    impl Parse for XBoxExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                xbox_token: input.parse()?,
                bang_token: input.parse()?,
                bracket: bracketed!(content in input),
                expr: content.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct XOptExpr {
        pub xopt_token: keyword::xopt,
        pub bang_token: Token![!],
        pub bracket: Bracket,
        pub expr: Box<Expr>,
    }

    impl Parse for XOptExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                xopt_token: input.parse()?,
                bang_token: input.parse()?,
                bracket: bracketed!(content in input),
                expr: content.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct XVecExpr {
        pub xvec_token: keyword::xvec,
        pub bang_token: Token![!],
        pub bracket: Bracket,
        pub expr: Box<Expr>,
        pub comma_token: Token![,],
        pub limit: Limit,
    }

    impl Parse for XVecExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                xvec_token: input.parse()?,
                bang_token: input.parse()?,
                bracket: bracketed!(content in input),
                expr: content.parse()?,
                comma_token: content.parse()?,
                limit: content.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct XLOptExpr {
        pub xlopt_token: keyword::xlopt,
        pub bang_token: Token![!],
        pub bracket: Bracket,
        pub expr: Box<Expr>,
    }

    impl Parse for XLOptExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                xlopt_token: input.parse()?,
                bang_token: input.parse()?,
                bracket: bracketed!(content in input),
                expr: content.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct XLVecExpr {
        pub xlvec_token: keyword::xlvec,
        pub bang_token: Token![!],
        pub bracket: Bracket,
        pub expr: Box<Expr>,
        pub comma_token: Token![,],
        pub limit: RangeLimit,
    }

    impl Parse for XLVecExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                xlvec_token: input.parse()?,
                bang_token: input.parse()?,
                bracket: bracketed!(content in input),
                expr: content.parse()?,
                comma_token: content.parse()?,
                limit: content.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct TupleExpr {
        pub paren: Paren,
        pub exprs: Punctuated<Expr, Token![,]>,
    }

    impl Parse for TupleExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                paren: parenthesized!(content in input),
                exprs: Punctuated::parse_terminated(&content)?,
            })
        }
    }

    #[derive(Debug)]
    pub struct ShardExpr {
        pub ident: Ident,
        pub args: Args,
    }

    impl Parse for ShardExpr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                ident: input.parse()?,
                args: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub enum Limit {
        Exact(ExactLimit),
        Range(RangeLimit),
    }

    impl Parse for Limit {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(LitInt) {
                if input.peek2(Token![..]) {
                    Ok(Self::Range(input.parse()?))
                } else {
                    Ok(Self::Exact(input.parse()?))
                }
            } else if lookahead.peek(Token![..]) {
                Ok(Self::Range(input.parse()?))
            } else {
                Err(lookahead.error())
            }
        }
    }

    #[derive(Debug)]
    pub struct ExactLimit {
        pub count: LitInt,
    }

    impl Parse for ExactLimit {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                count: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct RangeLimit {
        pub start: Option<LitInt>,
        pub dot_dot_token: Token![..],
        pub end: Option<LitInt>,
    }

    impl Parse for RangeLimit {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                start: input.parse()?,
                dot_dot_token: input.parse()?,
                end: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct Args {
        pub lt_token: Option<Token![<]>,
        pub exprs: Punctuated<Expr, Token![,]>,
        pub gt_token: Option<Token![>]>,
    }

    impl Parse for Args {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            if input.peek(Token![<]) {
                Ok(Self {
                    lt_token: Some(input.parse()?),
                    exprs: Punctuated::parse_separated_nonempty(input)?,
                    gt_token: Some(input.parse()?),
                })
            } else {
                Ok(Self {
                    lt_token: None,
                    exprs: Punctuated::default(),
                    gt_token: None,
                })
            }
        }
    }
}

pub mod prim_expr {
    use syn::{
        Ident, LitChar, LitInt, LitStr, Token, braced, bracketed, parenthesized,
        parse::{Parse, ParseStream, discouraged::Speculative},
        punctuated::Punctuated,
        token::{Brace, Bracket, Paren},
    };

    #[derive(Debug)]
    pub struct Sequence {
        pub exprs: Vec<Expr>,
    }

    impl Parse for Sequence {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let mut exprs = Vec::new();
            loop {
                let fork = input.fork();
                if let Ok(expr) = fork.parse() {
                    input.advance_to(&fork);
                    exprs.push(expr);
                } else {
                    break;
                }
            }
            Ok(Self { exprs })
        }
    }

    #[derive(Debug)]
    pub struct Expr {
        pub atom: Atom,
        pub modifier: Modifier,
    }

    impl Parse for Expr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                atom: input.parse()?,
                modifier: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub enum Atom {
        Lit(LitAtom),
        Set(SetAtom),
        Seq(SeqAtom),
        Alt(AltAtom),
        Shard(ShardAtom),
    }

    impl Parse for Atom {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(LitStr) {
                Ok(Self::Lit(input.parse()?))
            } else if lookahead.peek(Brace) {
                Ok(Self::Set(input.parse()?))
            } else if lookahead.peek(Paren) {
                Ok(Self::Seq(input.parse()?))
            } else if lookahead.peek(Bracket) {
                Ok(Self::Alt(input.parse()?))
            } else if lookahead.peek(Ident) {
                Ok(Self::Shard(input.parse()?))
            } else {
                Err(lookahead.error())
            }
        }
    }

    #[derive(Debug)]
    pub struct LitAtom {
        pub text: LitStr,
    }

    impl Parse for LitAtom {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                text: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct SetAtom {
        pub brace: Brace,
        pub bang_token: Option<Token![!]>,
        pub entries: Vec<SetEntry>,
    }

    impl Parse for SetAtom {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            let brace = braced!(content in input);
            let bang_token = content.parse()?;
            let mut entries = Vec::new();
            loop {
                let fork = content.fork();
                if let Ok(entry) = fork.parse() {
                    content.advance_to(&fork);
                    entries.push(entry);
                } else {
                    break;
                }
            }
            Ok(Self {
                brace,
                bang_token,
                entries,
            })
        }
    }

    #[derive(Debug)]
    pub struct SetEntry {
        pub start: LitChar,
        pub dot_dot_token: Option<Token![..]>,
        pub end: Option<LitChar>,
    }

    impl Parse for SetEntry {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            if input.peek2(Token![..]) {
                Ok(Self {
                    start: input.parse()?,
                    dot_dot_token: Some(input.parse()?),
                    end: Some(input.parse()?),
                })
            } else {
                Ok(Self {
                    start: input.parse()?,
                    dot_dot_token: None,
                    end: None,
                })
            }
        }
    }

    #[derive(Debug)]
    pub struct SeqAtom {
        pub paren: Paren,
        pub sequence: Sequence,
    }

    impl Parse for SeqAtom {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            Ok(Self {
                paren: parenthesized!(content in input),
                sequence: content.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct AltAtom {
        pub bracket: Bracket,
        pub entries: Vec<AltEntry>,
    }

    impl Parse for AltAtom {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let content;
            let bracket = bracketed!(content in input);
            let mut entries = Vec::new();
            loop {
                let fork = content.fork();
                if let Ok(entry) = fork.parse() {
                    content.advance_to(&fork);
                    entries.push(entry);
                } else {
                    break;
                }
            }
            Ok(Self { bracket, entries })
        }
    }

    #[derive(Debug)]
    pub struct AltEntry {
        pub or_token: Token![|],
        pub sequence: Sequence,
    }

    impl Parse for AltEntry {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                or_token: input.parse()?,
                sequence: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct ShardAtom {
        pub ident: Ident,
        pub args: Args,
    }

    impl Parse for ShardAtom {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                ident: input.parse()?,
                args: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct Args {
        pub lt_token: Option<Token![<]>,
        pub exprs: Punctuated<Expr, Token![,]>,
        pub gt_token: Option<Token![>]>,
    }

    impl Parse for Args {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            if input.peek(Token![<]) {
                Ok(Self {
                    lt_token: Some(input.parse()?),
                    exprs: Punctuated::parse_separated_nonempty(input)?,
                    gt_token: Some(input.parse()?),
                })
            } else {
                Ok(Self {
                    lt_token: None,
                    exprs: Punctuated::default(),
                    gt_token: None,
                })
            }
        }
    }

    #[derive(Debug)]
    pub enum Modifier {
        None,
        Star {
            star_token: Token![*],
            question_token: Option<Token![?]>,
        },
        Plus {
            plus_token: Token![+],
            question_token: Option<Token![?]>,
        },
        Caret {
            caret_token: Token![^],
            bracket: Bracket,
            limit: Limit,
        },
    }

    impl Parse for Modifier {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            if input.peek(Token![*]) {
                Ok(Self::Star {
                    star_token: input.parse()?,
                    question_token: input.parse()?,
                })
            } else if input.peek(Token![+]) {
                Ok(Self::Plus {
                    plus_token: input.parse()?,
                    question_token: input.parse()?,
                })
            } else if input.peek(Token![^]) {
                let content;
                Ok(Self::Caret {
                    caret_token: input.parse()?,
                    bracket: bracketed!(content in input),
                    limit: content.parse()?,
                })
            } else {
                Ok(Self::None)
            }
        }
    }

    #[derive(Debug)]
    pub enum Limit {
        Exact(ExactLimit),
        Range(RangeLimit),
    }

    impl Parse for Limit {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(LitInt) {
                if input.peek2(Token![..]) {
                    Ok(Self::Range(input.parse()?))
                } else {
                    Ok(Self::Exact(input.parse()?))
                }
            } else if lookahead.peek(Token![..]) {
                Ok(Self::Range(input.parse()?))
            } else {
                Err(lookahead.error())
            }
        }
    }

    #[derive(Debug)]
    pub struct ExactLimit {
        pub count: LitInt,
    }

    impl Parse for ExactLimit {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                count: input.parse()?,
            })
        }
    }

    #[derive(Debug)]
    pub struct RangeLimit {
        pub start: Option<LitInt>,
        pub dot_dot_token: Token![..],
        pub end: Option<LitInt>,
        pub question_token: Option<Token![?]>,
    }

    impl Parse for RangeLimit {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                start: input.parse()?,
                dot_dot_token: input.parse()?,
                end: input.parse()?,
                question_token: input.parse()?,
            })
        }
    }
}
