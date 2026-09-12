use syn::{
    Ident, Token, Visibility,
    punctuated::Punctuated,
    token::{Brace, Paren},
};

#[derive(Debug)]
pub enum Shard {
    Struct(StructShard),
    Enum(EnumShard),
    Type(TypeShard),
}

#[derive(Debug)]
pub struct StructShard {
    pub vis: Visibility,
    pub struct_token: Token![struct],
    pub ident: Ident,
    pub params: Option<Params>,
    pub brace: Brace,
    pub fields: Punctuated<Field, Token![,]>,
}

#[derive(Debug)]
pub struct EnumShard {
    pub vis: Visibility,
    pub enum_token: Token![enum],
    pub ident: Ident,
    pub params: Option<Params>,
    pub brace: Brace,
    pub variants: Punctuated<Variant, Token![,]>,
}

#[derive(Debug)]
pub struct TypeShard {
    pub vis: Visibility,
    pub type_token: Token![type],
    pub ident: Ident,
    pub params: Option<Params>,
    pub eq_token: Token![=],
    pub x_expr: rust_expr::XExpr,
    pub semi_token: Token![;],
}

#[derive(Debug)]
pub struct Field {
    pub ident: Ident,
    pub colon_token: Token![:],
    pub expr: rust_expr::Expr,
}

#[derive(Debug)]
pub struct Variant {
    pub paren: Paren,
}

#[derive(Debug)]
pub struct Params {
    pub lt_token: Token![<],
    pub idents: Punctuated<Ident, Token![,]>,
    pub gt_token: Token![>],
}

pub mod rust_expr {
    use syn::{
        Ident, LitInt, Token,
        punctuated::Punctuated,
        token::{Brace, Bracket, Paren},
    };

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

    #[derive(Debug)]
    pub struct XExpr {
        pub x_token: keyword::x,
        pub bang_token: Token![!],
        pub brace: Brace,
    }

    #[derive(Debug)]
    pub struct XBoxExpr {
        pub xbox_token: keyword::xbox,
        pub bang_token: Token![!],
        pub bracket: Bracket,
        pub expr: Box<Expr>,
    }

    #[derive(Debug)]
    pub struct XOptExpr {
        pub xopt_token: keyword::xopt,
        pub bang_token: Token![!],
        pub bracket: Bracket,
        pub expr: Box<Expr>,
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

    #[derive(Debug)]
    pub struct XLOptExpr {
        pub xlopt_token: keyword::xlopt,
        pub bang_token: Token![!],
        pub bracket: Bracket,
        pub expr: Box<Expr>,
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

    #[derive(Debug)]
    pub struct TupleExpr {
        pub paren: Paren,
        pub exprs: Punctuated<Expr, Token![,]>,
    }

    #[derive(Debug)]
    pub struct ShardExpr {
        pub ident: Ident,
        pub args: Option<Args>,
    }

    #[derive(Debug)]
    pub enum Limit {
        Exact(ExactLimit),
        Range(RangeLimit),
    }

    #[derive(Debug)]
    pub struct ExactLimit {
        pub count: LitInt,
    }

    #[derive(Debug)]
    pub struct RangeLimit {
        pub start: Option<LitInt>,
        pub dot_dot_token: Token![..],
        pub end: Option<LitInt>,
    }

    #[derive(Debug)]
    pub struct Args {
        pub lt_token: Token![<],
        pub exprs: Punctuated<Expr, Token![,]>,
        pub gt_token: Token![>],
    }
}

pub mod prim_expr {
    use syn::{
        Ident, LitChar, LitInt, LitStr, Token,
        punctuated::Punctuated,
        token::{Brace, Bracket, Paren},
    };

    #[derive(Debug)]
    pub struct ExprSequence {
        pub exprs: Vec<Expr>,
    }

    #[derive(Debug)]
    pub struct Expr {
        pub atom: Atom,
        pub modifier: Modifier,
    }

    #[derive(Debug)]
    pub enum Atom {
        Lit(LitAtom),
        Set(SetAtom),
        Seq(SeqAtom),
        Alt(AltAtom),
        Shard(ShardAtom),
    }

    #[derive(Debug)]
    pub struct LitAtom {
        pub text: LitStr,
    }

    #[derive(Debug)]
    pub struct SetAtom {
        pub brace: Brace,
        pub bang_token: Option<Token![!]>,
        pub entries: Vec<SetEntry>,
    }

    #[derive(Debug)]
    pub struct SetEntry {
        pub start: LitChar,
        pub dot_dot_token: Option<Token![..]>,
        pub end: Option<LitChar>,
    }

    #[derive(Debug)]
    pub struct SeqAtom {
        pub paren: Paren,
        pub sequence: ExprSequence,
    }

    #[derive(Debug)]
    pub struct AltAtom {
        pub bracket: Bracket,
        pub entries: Vec<AltEntry>,
    }

    #[derive(Debug)]
    pub struct AltEntry {
        pub or_token: Token![|],
        pub sequence: ExprSequence,
    }

    #[derive(Debug)]
    pub struct ShardAtom {
        pub ident: Ident,
        pub args: Option<Args>,
    }

    #[derive(Debug)]
    pub struct Args {
        pub lt_token: Token![<],
        pub exprs: Punctuated<Expr, Token![,]>,
        pub gt_token: Token![>],
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

    #[derive(Debug)]
    pub enum Limit {
        Exact {
            count: LitInt,
        },
        Range {
            start: Option<LitInt>,
            dot_dot_token: Token![..],
            end: Option<LitInt>,
            question_token: Option<Token![?]>,
        },
    }
}
