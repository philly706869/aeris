use aeris::ui::shard;

// use crate::cluster::{JSONValue, WS};

#[shard]
pub struct JSONArray {
    bracket: x!["["],
    ws: WS,
    entries: Punctuated<Spanned<JSONValue>, Spanned<x![","]>>,
    bracket: x!["["],
}

#[shard]
pub struct Punctuated<T, P> {
    inner: Option<(T, Vec<(P, T)>)>,
}

mod mapping {
    use std::marker::PhantomData;

    #[derive(Debug)]
    pub struct WS<'i>(PhantomData<&'i Self>, &'i str);

    #[derive(Debug)]
    pub struct Punctuated<'i, T, P>(PhantomData<&'i Self>, Option<(T, Vec<(P, T)>)>);

    #[derive(Debug)]
    pub struct Spanned<'i, T> {
        _i: PhantomData<&'i Self>,
        inner: T,
        ws: WS<'i>,
    }

    #[derive(Debug)]
    pub struct JSONArray<'i> {
        _i: PhantomData<&'i Self>,
        bracket: (&'i str, &'i str),
        ws: WS<'i>,
        entries: Punctuated<'i, Spanned<'i, JSONValue<'i>>, Spanned<'i, &'i str>>,
    }

    #[derive(Debug)]
    pub struct JSONValue<'i> {
        _i: PhantomData<&'i Self>,
    }
}
