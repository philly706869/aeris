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
