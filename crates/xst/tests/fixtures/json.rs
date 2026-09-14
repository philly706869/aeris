// Grammar fixtures from examples/json/src/cluster/mapping.rs.
use xst::shard;

#[shard]
pub struct JSON {
    ws: WS,
    value: JSONValue,
    ws: WS,
}

#[shard]
pub enum JSONValue {
    Object(JSONObject),
    Array(JSONArray),
    String(JSONString),
    Number(JSONNumber),
    Boolean(JSONBoolean),
    Null(JSONNull),
}

#[shard]
pub struct JSONObject {
    brace: x! { "{" },
    ws: WS,
    content: Punctuated<Spanned<JSONObjectEntry>, Spanned<x! { "," }>>,
    brace: x! { "}" },
}

#[shard]
pub struct JSONObjectEntry {
    name: JSONString,
    ws: WS,
    colon: x! { ":" },
    ws: WS,
    value: JSONValue,
}

#[shard]
pub struct JSONArray {
    bracket: x! { "[" },
    ws: WS,
    entries: Punctuated<Spanned<JSONValue>, Spanned<x! { "," }>>,
    bracket: x! { "]" },
}

#[shard]
pub struct JSONString {
    quote: x! { "\"" },
    content: Content,
    quote: x! { "\"" },
}

#[shard]
type Content = x! {
    [
        | {! '"' '\\' '\u{0000}'..'\u{001F}'}
        | "\\" Escape
    ]*
};

#[shard]
type Escape = x! {[
    | {'"' '\\' '/' 'b' 'f' 'n' 'r' 't'}
    | "u" {'0'..'9' 'A'..'F' 'a'..'f'}^[4]
]};

#[shard]
pub struct JSONNumber {
    sign: xopt![x! { "-" }],
    integer: x! {[
        | Digit
        | One2Nine Digits
    ]},
    fraction: xopt![JSONFraction],
    exponent: xopt![JSONExponent],
}

#[shard]
pub struct JSONFraction {
    point: x! { "." },
    digits: Digits,
}

#[shard]
pub struct JSONExponent {
    e: x! { {'E' 'e'} },
    sign: xopt![x! { {'+' '-'} }],
    digits: Digits,
}

#[shard]
type Digits = x! { Digit+ };

#[shard]
type Digit = x! {[
    | "0"
    | One2Nine
]};

#[shard]
type One2Nine = x! { {'1'..'9'} };

#[shard]
pub enum JSONBoolean {
    True(JSONTrue),
    False(JSONFalse),
}

#[shard]
pub struct JSONTrue {
    text: x! { "true" },
}

#[shard]
pub struct JSONFalse {
    text: x! { "false" },
}

#[shard]
pub struct JSONNull {
    text: x! { "null" },
}

#[shard]
pub struct WS {
    space: x! { {' ' '\t' '\n' '\r'}* },
}

#[shard]
pub struct Punctuated<T, P> {
    inner: xopt![(xbox![T], xvec![(P, T), ..])],
}

#[shard]
pub struct Spanned<T> {
    inner: T,
    ws: WS,
}
