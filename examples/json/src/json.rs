use xst::shard;

use crate::{JSONValue, WS};

#[shard]
pub struct JSON {
    ws: WS,
    value: JSONValue,
    ws: WS,
}
