use aeris::ui::shard;

// use crate::cluster::{JSONValue, WS};

#[shard]
pub mod JSONArray {
    #[derive(Debug)]
    struct Shard {
        bracket: x!["["],
        ws: WS,
        entries: Punctuated<Spanned<JSONValue>, Spanned<x![","]>>,
        bracket: x!["["],
    }
}
