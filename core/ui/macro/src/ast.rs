use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub struct Syntax {}

impl Parse for Syntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Struct {}

impl Parse for Struct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Enum {}

impl Parse for Enum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Lambda {}

impl Parse for Lambda {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub struct L3Entry {}

impl Parse for L3Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub struct L2Entry {}

impl Parse for L2Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub struct L1Entry {}

impl Parse for L1Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}
