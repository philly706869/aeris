use aeris::ui::cluster;

cluster! {
    #[derive(Debug)]
    pub enum JSONValue
    trait JSONValueImpl {
        Object ( JSONObject )
        Array ( JSONArray )
        String ( JSONString )
        Number ( JSONNumber )
        Boolean ( JSONBoolean )
        Null ( JSONNull )
    }
}

impl JSONValueImpl for JSONValue {}
