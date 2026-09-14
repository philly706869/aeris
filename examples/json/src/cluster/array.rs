use xst::shard;

use crate::cluster::{JSONValue, Punctuated, Spanned, WS};

#[shard]
pub struct JSONArray {
    bracket: x! { "[" },
    ws: WS,
    entries: Punctuated<Spanned<JSONValue>, Spanned<x! { "," }>>,
    bracket: x! { "]" },
}
