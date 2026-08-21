use aeris::ui::shard;

// use crate::cluster::{JSONString, JSONValue, Spanned, WS};

#[shard]
pub mod JSONObject {
    #[derive(Debug)]
    struct Shard {
        brace: x!["{"],
        ws: WS,
        content: Punctuated<Spanned<JSONObjectEntry>, Spanned<x![","]>>,
        brace: x!["}"],
    }
}

#[shard]
pub mod JSONObjectEntry {
    #[derive(Debug)]
    struct Shard {
        name: JSONString,
        ws: WS,
        colon: x![":"],
        ws: WS,
        value: JSONValue,
    }
}
