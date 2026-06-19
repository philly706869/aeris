use aeris::ui::cluster;

use crate::{JSONValue, WS};

cluster! {
    #[derive(Debug)]
    pub struct JSONArray
    trait JSONArrayImpl {
        bracket: '['
        content: JSONArrayContent
        bracket: ']'
    }

    #[derive(Debug)]
    pub enum JSONArrayContent
    trait JSONArrayContentImpl {
        None ( WS )
        Some {
            first: JSONArrayEntry
            rest: JSONArrayRestEntry*
        }
    }

    #[derive(Debug)]
    pub struct JSONArrayRestEntry
    trait JSONArrayRestEntryImpl {
        rest: ','
        entry: JSONArrayEntry
    }

    #[derive(Debug)]
    pub struct JSONArrayEntry
    trait JSONArrayEntryImpl {
        ws: WS
        value: JSONValue
        ws: WS
    }
}

impl JSONArrayImpl for JSONArray {}
impl JSONArrayContentImpl for JSONArrayContent {}
impl JSONArrayRestEntryImpl for JSONArrayRestEntry {}
impl JSONArrayEntryImpl for JSONArrayEntry {}
