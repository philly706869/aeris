use aeris::ui::shard;

// use crate::cluster::{JSONValue, WS};

#[shard]
pub mod JSONArray {
    #[derive(Debug)]
    struct Shard {
        bracket: x!["["],
        content: JSONArrayContent,
        bracket: x!["["],
    }
}

#[shard]
pub mod JSONArrayContent {
    #[derive(Debug)]
    enum Shard {
        None(WS),
        Some {
            first: JSONArrayEntry,
            rest: Vec<JSONArrayRestEntry>,
        },
    }
}

#[shard]
pub mod JSONArrayRestEntry {
    #[derive(Debug)]
    struct Shard {
        rest: x![","],
        entry: JSONArrayEntry,
    }
}

#[shard]
pub mod JSONArrayEntry {
    #[derive(Debug)]
    struct Shard {
        ws: WS,
        value: JSONValue,
        ws: WS,
    }
}
