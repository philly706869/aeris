use aeris::ui::cluster;

use crate::{JSONValue, WS};

cluster! {
    #[derive(Debug)]
    pub struct JSONObject
    trait JSONObjectImpl {
        (brace): '{'
        content: JSONObjectContent
        (brace): '}'
    }

    #[derive(Debug)]
    pub enum JSONObjectContent
    trait JSONObjectContentImpl {
        None ( WS )
        Some {
            first: JSONObjectEntry
            rest: JSONObjectRestEntry*
        }
    }

    #[derive(Debug)]
    pub struct JSONObjectRestEntry
    trait JSONObjectRestEntryImpl {
        rest: ','
        entry: JSONObjectEntry
    }

    #[derive(Debug)]
    pub struct JSONObjectEntry
    trait JSONObjectEntryImpl {
        (ws): WS
        name: JSONString
        (ws): WS
        colon: ':'
        (ws): WS
        value: JSONValue
        (ws): WS
    }
}

impl JSONObjectImpl for JSONObject {}
impl JSONObjectContentImpl for JSONObjectContent {}
impl JSONObjectRestEntryImpl for JSONObjectRestEntry {}
impl JSONObjectEntryImpl for JSONObjectEntry {}
