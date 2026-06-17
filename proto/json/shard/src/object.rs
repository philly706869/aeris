use aeris::ui::cluster;

cluster! {
    pub struct JSONObject
    trait JSONObjectImpl {
        (brace): '{'
        content: JSONObjectContent
        (brace): '}'
    }

    pub enum JSONObjectContent
    trait JSONObjectContentImpl {
        None ( WS )
        Some {
            first: JSONObjectEntry
            rest: JSONObjectRestEntry*
        }
    }

    pub struct JSONObjectRestEntry
    trait JSONObjectRestEntryImpl {
        rest: ','
        entry: JSONObjectEntry
    }

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
