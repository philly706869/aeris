use aeris::ui::shard;

// use crate::cluster::{JSONArray, JSONBoolean, JSONNull, JSONNumber, JSONObject, JSONString};

#[shard]
pub mod JSONValue {
    #[derive(Debug)]
    enum Shard {
        Object(JSONObject),
        Array(JSONArray),
        String(JSONString),
        Number(JSONNumber),
        Boolean(JSONBoolean),
        Null(JSONNull),
    }
}
