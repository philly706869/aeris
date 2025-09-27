use std::collections::HashMap;

pub enum JSONValue {
    Object(HashMap<String, JSONValue>),
    Array(Vec<JSONValue>),
    String(String),
    Number(String),
    Boolean(bool),
    Null,
}
