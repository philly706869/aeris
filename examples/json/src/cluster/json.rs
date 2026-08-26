use aeris::ui::shard;

// use crate::cluster::{JSONValue, WS};

#[shard]
pub struct JSON {
    ws: WS,
    value: JSONValue,
    ws: WS,
}
