use aeris::ui::shard;

// use crate::cluster::{JSONString, JSONValue, WS};

#[shard]
pub mod JSONObject {
    #[derive(Debug)]
    struct Shard {
        brace: x!["{"],
        content: JSONObjectContent,
        brace: x!["}"],
    }
}

#[shard]
pub mod JSONObjectContent {
    #[derive(Debug)]
    enum Shard {
        None(WS),
        Some {
            first: JSONObjectEntry,
            rest: Vec<JSONObjectRestEntry>,
        },
    }
}

#[shard]
pub mod JSONObjectRestEntry {
    #[derive(Debug)]
    struct Shard {
        rest: x![","],
        entry: JSONObjectEntry,
    }
}

#[shard]
pub mod JSONObjectEntry {
    #[derive(Debug)]
    struct Shard {
        ws: WS,
        name: JSONString,
        ws: WS,
        colon: x![":"],
        ws: WS,
        value: JSONValue,
        ws: WS,
    }
}
