use aeris::ui::shard;

// use crate::cluster::{JSONValue, WS};

#[shard]
pub struct JSONArray {
    bracket: x!["["],
    ws: WS,
    entries: Punctuated<Spanned<Box<JSONValue>>, Spanned<x![","]>>,
    bracket: x!["["],
}
