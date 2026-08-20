use aeris::ui::shard;

// use crate::cluster::{JSONValue, WS};

#[shard]
pub mod JSONArray {
    #[derive(Debug)]
    struct Shard {
        bracket: x!["["],
        ws: WS,
        entries: Punctuated<JSONArrayEntry, (x![","], WS)>,
        bracket: x!["["],
    }
}

#[shard]
pub mod JSONArrayEntry {
    #[derive(Debug)]
    struct Shard {
        value: JSONValue,
        ws: WS,
    }
}
