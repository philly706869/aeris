use aeris::ui::shard;

// use crate::cluster::{JSONValue, WS};

#[shard]
pub mod JSON {
    #[derive(Debug)]
    struct Shard {
        ws: WS,
        value: JSONValue,
        ws: WS,
    }
}
