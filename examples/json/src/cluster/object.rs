use aeris::ui::shard;

// use crate::cluster::{JSONString, JSONValue, Spanned, WS};

#[shard]
pub struct JSONObject {
    brace: x!["{"],
    ws: WS,
    content: Punctuated<Spanned<JSONObjectEntry>, Spanned<x![","]>>,
    brace: x!["}"],
}

#[shard]
pub struct JSONObjectEntry {
    name: JSONString,
    ws: WS,
    colon: x![":"],
    ws: WS,
    value: JSONValue,
}
