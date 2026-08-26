use aeris::ui::shard;

// use crate::cluster::{JSONArray, JSONBoolean, JSONNull, JSONNumber, JSONObject, JSONString};

#[shard]
pub enum JSONValue {
    Object(JSONObject),
    Array(JSONArray),
    String(JSONString),
    Number(JSONNumber),
    Boolean(JSONBoolean),
    Null(JSONNull),
}
