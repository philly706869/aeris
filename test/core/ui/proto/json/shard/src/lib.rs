use aeris::ui::cluster;

cluster! {
    #[derive(Debug)]
    pub struct JSON
    trait JSONImpl {
        (ws): WS
        value: JSONValue
        (ws): WS
    }
}

impl JSONImpl for JSON {}

cluster! {
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

cluster! {
    pub struct JSONString
    trait JSONStringImpl {
        (quote): '"'
        content: Content
        (quote): '"'
    }
    Content (
        (
            | {! '"' '\\' '\u{0000}'..'\u{001F}'}
            | '\\' Escape
        )*
    )
    Escape (
        | {'"' '\\' '/' 'b' 'f' 'n' 'r' 't'}
        | 'u' {'0'..'9' 'A'..'F' 'a'..'f'}[4]
    )
}

impl JSONStringImpl for JSONString {}

cluster! {
    pub struct JSONNumber
    trait JSONNumberImpl {
        sign: '-'?
        integer: (Digit | One2Nine Digits)
        fraction: JSONFraction?
        exponent: JSONExponent?
    }

    pub struct JSONFraction
    trait JSONFractionImpl {
        point: '.'
        digits: Digits
    }

    pub struct JSONExponent
    trait JSONExponentImpl {
        e: {'E' 'e'}
        sign: JSONSign?
        digits: Digits
    }

    pub enum JSONSign
    trait JSONSignImpl {
        Plus ( '+' )
        Minus ( '-' )
    }

    Digits ( Digit+ )
    Digit ( '0' | One2Nine )
    One2Nine ( {'1'..'9'} )
}

impl JSONNumberImpl for JSONNumber {}
impl JSONFractionImpl for JSONFraction {}
impl JSONExponentImpl for JSONExponent {}
impl JSONSignImpl for JSONSign {}

cluster! {
    pub enum JSONBoolean
    trait JSONBooleanImpl {
        True ( "true" )
        False ( "false" )
    }
}

impl JSONBooleanImpl for JSONBoolean {}

cluster! {
    pub struct JSONNull
    trait JSONNullImpl ( "null" )
}

impl JSONNullImpl for JSONNull {}

cluster! {
    pub struct WS
    trait WSImpl ( ( {' ' '\t' '\n' '\r'}* ) )
}

impl WSImpl for WS {}
