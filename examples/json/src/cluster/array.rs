use xst::shard;

// use crate::cluster::{JSONValue, WS};

#[shard]
pub struct JSONArray {
    bracket: x! { "[" },
    ws: WS,
    entries: Punctuated<Spanned<xbox![JSONValue]>, Spanned<x! { "," }>>,
    bracket: x! { "]" },
}
