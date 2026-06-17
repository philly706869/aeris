use aeris::ui::cluster;

cluster! {
    pub struct JSONArray
    trait JSONArrayImpl {
        (bracket): '['
        content: JSONArrayContent
        (bracket): ']'
    }

    pub enum JSONArrayContent
    trait JSONArrayContentImpl {
        None ( WS )
        Some {
            first: JSONArrayEntry
            rest: JSONArrayRestEntry*
        }
    }

    pub struct JSONArrayRestEntry
    trait JSONArrayRestEntryImpl {
        rest: ','
        entry: JSONArrayEntry
    }

    pub struct JSONArrayEntry
    trait JSONArrayEntryImpl {
        (ws): WS
        value: JSONValue
        (ws): WS
    }
}

impl JSONArrayImpl for JSONArray {}
impl JSONArrayContentImpl for JSONArrayContent {}
impl JSONArrayRestEntryImpl for JSONArrayRestEntry {}
impl JSONArrayEntryImpl for JSONArrayEntry {}
