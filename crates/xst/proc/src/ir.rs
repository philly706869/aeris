use proc_macro2::TokenStream;
use syn::{Ident, Visibility};

#[derive(Debug)]
pub struct Shard {
    pub vis: Visibility,
    pub ident: Ident,
    pub params: Vec<Ident>,
    pub core_ident: Ident,
    pub marker_ident: Ident,
    pub kind: Kind,
    pub data: TokenStream,
    pub closures: Vec<Closure>,
}

#[derive(Debug)]
pub enum Kind {
    Struct(Vec<(Ident, TokenStream)>),
    Enum(Vec<(Ident, TokenStream)>),
    Type,
}

#[derive(Debug)]
pub struct Closure {
    pub index: usize,
    pub ident: Ident,
    pub output: TokenStream,
    pub data: TokenStream,
}
