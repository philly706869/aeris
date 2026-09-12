use syn::{
    Ident, LitInt, Token, Visibility,
    punctuated::Punctuated,
    token::{Brace, Bracket, Paren},
};

mod keyword {
    syn::custom_keyword!(x);
    syn::custom_keyword!(xbox);
    syn::custom_keyword!(xopt);
    syn::custom_keyword!(xvec);
}

#[derive(Debug)]
pub struct Shard {
    pub variant: ShardVariant,
}

#[derive(Debug)]
pub enum ShardVariant {
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
pub struct Field {
    pub ident: Ident,
    pub colon_token: Token![:],
    pub expr: Expr,
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
pub struct Variant {
    pub paren: Paren,
}

#[derive(Debug)]
pub struct TypeShard {
    pub vis: Visibility,
    pub type_token: Token![type],
    pub ident: Ident,
    pub params: Option<Params>,
    pub eq_token: Token![=],
    pub x_expr: XExpr,
    pub semi_token: Token![;],
}

#[derive(Debug)]
pub struct Params {
    pub lt_token: Token![<],
    pub idents: Punctuated<Ident, Token![,]>,
    pub gt_token: Token![>],
}

#[derive(Debug)]
pub enum Expr {
    X(XExpr),
    XBox(XBoxExpr),
    XOpt(XOptExpr),
    XVec(XVecExpr),
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
    pub range: Range,
}

#[derive(Debug)]
pub struct Range {
    pub start: Option<Box<LitInt>>,
    pub dot_dot_token: Token![..],
    pub end: Option<Box<LitInt>>,
}

#[derive(Debug)]
pub struct ShardExpr {
    pub ident: Ident,
    pub args: Option<Args>,
}

#[derive(Debug)]
pub struct Args {
    pub lt_token: Token![<],
    pub exprs: Punctuated<Expr, Token![,]>,
    pub gt_token: Token![>],
}
