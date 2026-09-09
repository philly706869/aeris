//! #[shard] macro code generation example

use xst::shard;

// ////////////////
// JSON
// ////////////////

// original reference: crate::cluster::json::JSON
#[shard]
pub struct JSON {
    ws: WS,
    value: JSONValue,
    ws: WS,
}

#[derive(Debug)]
pub struct JSON<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    ws: (WS<'i>, WS<'i>),
    value: JSONValue<'i>,
}

const _: () = {
    impl ::xst::internal::Shard for JSON<'static> {
        type Data = ::xst::internal::Alt<(
            ::xst::internal::Ext<WS<'static>>,
            ::xst::internal::Ext<JSONValue<'static>>,
            ::xst::internal::Ext<WS<'static>>,
        )>;
    }

    impl ::xst::internal::StaticShard for JSON<'static> {}
};

// ////////////////
// JSON
// ////////////////

// ////////////////
// JSONValue
// ////////////////

// original reference: crate::cluster::value::JSONValue
#[shard]
pub enum JSONValue {
    Object(JSONObject),
    Array(JSONArray),
    String(JSONString),
    Number(JSONNumber),
    Boolean(JSONBoolean),
    Null(JSONNull),
}

#[derive(Debug)]
pub enum JSONValue<'i> {
    Object(JSONObject<'i>),
    Array(JSONArray<'i>),
    String(JSONString<'i>),
    Number(JSONNumber<'i>),
    Boolean(JSONBoolean<'i>),
    Null(JSONNull<'i>),
}

const _: () = {
    impl ::xst::internal::Shard for JSONValue<'static> {
        type Data = ::xst::internal::Alt<(
            ::xst::internal::Ext<JSONObject<'static>>,
            ::xst::internal::Ext<JSONArray<'static>>,
            ::xst::internal::Ext<JSONString<'static>>,
            ::xst::internal::Ext<JSONNumber<'static>>,
            ::xst::internal::Ext<JSONBoolean<'static>>,
            ::xst::internal::Ext<JSONNull<'static>>,
        )>;
    }

    impl ::xst::internal::StaticShard for JSONValue<'static> {}
};

// ////////////////
// JSONValue
// ////////////////

// ////////////////
// JSONObject
// ////////////////

// original reference: crate::cluster::object::JSONObject
#[shard]
pub struct JSONObject {
    brace: x! { "{" },
    ws: WS,
    content: Punctuated<Spanned<JSONObjectEntry>, Spanned<x! { "," }>>,
    brace: x! { "}" },
}

#[derive(Debug)]
pub struct JSONObject<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    brace: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    ws: WS<'i>,
    content:
        Punctuated<'i, Spanned<'i, JSONObjectEntry<'i>>, Spanned<'i, &'i ::xst::internal::str>>,
}

const _: () = {
    impl ::xst::internal::Shard for JSONObject<'static> {
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Lit<Literal0>,
            ::xst::internal::Ext<WS<'static>>,
            ::xst::internal::Ext<
                Punctuated<
                    'static,
                    ::xst::internal::Ext<
                        Spanned<'static, ::xst::internal::Ext<JSONObjectEntry<'static>>>,
                    >,
                    ::xst::internal::Ext<Spanned<'static, ::xst::internal::Lit<Literal1>>>,
                >,
            >,
            ::xst::internal::Lit<Literal2>,
        )>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::str = "{";
    }

    #[allow(dead_code)]
    pub struct Literal1;

    impl ::xst::internal::ShardLiteral for Literal1 {
        const LITERAL: &'static ::xst::internal::str = ",";
    }

    #[allow(dead_code)]
    pub struct Literal2;

    impl ::xst::internal::ShardLiteral for Literal2 {
        const LITERAL: &'static ::xst::internal::str = "}";
    }

    impl ::xst::internal::StaticShard for JSONObject<'static> {}
};

// original reference: crate::cluster::object::JSONObjectEntry
#[shard]
pub struct JSONObjectEntry {
    name: JSONString,
    ws: WS,
    colon: x! { ":" },
    ws: WS,
    value: JSONValue,
}

#[derive(Debug)]
pub struct JSONObjectEntry<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    name: JSONString<'i>,
    ws: (WS<'i>, WS<'i>),
    colon: &'i ::xst::internal::str,
    value: JSONValue<'i>,
}

const _: () = {
    impl ::xst::internal::Shard for JSONObjectEntry<'static> {
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Ext<JSONString<'static>>,
            ::xst::internal::Ext<WS<'static>>,
            ::xst::internal::Lit<Literal0>,
            ::xst::internal::Ext<WS<'static>>,
            ::xst::internal::Ext<JSONValue<'static>>,
        )>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::str = ":";
    }

    impl ::xst::internal::StaticShard for JSONObjectEntry<'static> {}
};

// ////////////////
// JSONObject
// ////////////////

// ////////////////
// JSONArray
// ////////////////

// original reference: crate::cluster::array::JSONArray
#[shard]
pub struct JSONArray {
    bracket: x! { "[" },
    ws: WS,
    entries: Punctuated<Spanned<JSONValue>, Spanned<x! { "," }>>,
    bracket: x! { "[" },
}

#[derive(Debug)]
pub struct JSONArray<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    bracket: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    ws: WS<'i>,
    entries: Punctuated<'i, Spanned<'i, JSONValue<'i>>, Spanned<'i, &'i ::xst::internal::str>>,
}

const _: () = {
    impl ::xst::internal::Shard for JSONArray<'static> {
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Lit<Literal0>,
            ::xst::internal::Ext<WS<'static>>,
            ::xst::internal::Ext<
                Punctuated<
                    'static,
                    ::xst::internal::Ext<
                        Spanned<'static, ::xst::internal::Ext<JSONValue<'static>>>,
                    >,
                    ::xst::internal::Ext<Spanned<'static, ::xst::internal::Lit<Literal1>>>,
                >,
            >,
            ::xst::internal::Lit<Literal2>,
        )>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::str = "[";
    }

    #[allow(dead_code)]
    pub struct Literal1;

    impl ::xst::internal::ShardLiteral for Literal1 {
        const LITERAL: &'static ::xst::internal::str = ",";
    }

    #[allow(dead_code)]
    pub struct Literal2;

    impl ::xst::internal::ShardLiteral for Literal2 {
        const LITERAL: &'static ::xst::internal::str = "]";
    }

    impl ::xst::internal::StaticShard for JSONArray<'static> {}
};

// ////////////////
// JSONArray
// ////////////////

// ////////////////
// JSONString
// ////////////////

// original reference: crate::cluster::string::JSONString
#[shard]
pub struct JSONString {
    quote: x! { "\"" },
    content: Content,
    quote: x! { "\"" },
}

#[derive(Debug)]
pub struct JSONString<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    quote: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    content: Content<'i>, // TODO
}

const _: () = {
    impl ::xst::internal::Shard for JSONString<'static> {
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Lit<Literal0>,
            ::xst::internal::Ext<Content<'static>>,
            ::xst::internal::Lit<Literal1>,
        )>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::str = "\"";
    }

    #[allow(dead_code)]
    pub struct Literal1;

    impl ::xst::internal::ShardLiteral for Literal1 {
        const LITERAL: &'static ::xst::internal::str = "\"";
    }

    impl ::xst::internal::StaticShard for JSONString<'static> {}
};

// original reference: crate::cluster::string::Content
#[shard]
type Content = x! {
    [
        | {! '"' '\\' '\u{0000}'..'\u{001F}'}
        | "\\" Escape
    ]*
};

struct Content<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::Shard for Content<'static> {
        type Data = ::xst::internal::Vec<
            ::xst::internal::Alt<(
                ::xst::internal::Set<true, Set0>,
                ::xst::internal::Seq<(
                    ::xst::internal::Lit<Literal1>,
                    ::xst::internal::Ext<Escape<'static>>,
                )>,
            )>,
            0,
            0,
        >;
    }

    #[allow(dead_code)]
    pub struct Set0;

    impl ::xst::internal::ShardSet for Set0 {
        const SET: &'static [::xst::internal::RangeInclusive<::xst::internal::char>] =
            &['"'..='"', '\\'..='\\', '\u{0000}'..='\u{001F}'];
    }

    #[allow(dead_code)]
    pub struct Literal1;

    impl ::xst::internal::ShardLiteral for Literal1 {
        const LITERAL: &'static ::xst::internal::str = "\\";
    }
};

// original reference: crate::cluster::string::Escape
#[shard]
type Escape = x! {[
    | {'"' '\\' '/' 'b' 'f' 'n' 'r' 't'}
    | "u" {'0'..'9' 'A'..'F' 'a'..'f'}![4]
]};

struct Escape<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::Shard for Escape<'static> {
        type Data = ::xst::internal::Alt<(
            ::xst::internal::Set<false, Set0>,
            ::xst::internal::Seq<(
                ::xst::internal::Lit<Literal1>,
                ::xst::internal::Set<false, Set2>, // TODO
            )>,
        )>;
    }

    #[allow(dead_code)]
    pub struct Set0;

    impl ::xst::internal::ShardSet for Set0 {
        const SET: &'static [::xst::internal::RangeInclusive<::xst::internal::char>] = &[
            '"'..='"',
            '\\'..='\\',
            '/'..='/',
            'b'..='b',
            'f'..='f',
            'n'..='n',
            'r'..='r',
            't'..='t',
        ];
    }

    #[allow(dead_code)]
    pub struct Literal1;

    impl ::xst::internal::ShardLiteral for Literal1 {
        const LITERAL: &'static ::xst::internal::str = "u";
    }

    #[allow(dead_code)]
    pub struct Set2;

    impl ::xst::internal::ShardSet for Set2 {
        const SET: &'static [::xst::internal::RangeInclusive<::xst::internal::char>] =
            &['0'..='9', 'A'..='F', 'a'..='f'];
    }
};

// ////////////////
// JSONString
// ////////////////

// ////////////////
// JSONNumber
// ////////////////

// original reference: crate::cluster::number::JSONNumber
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

// original reference: crate::cluster::number::JSONFraction
#[shard]
pub struct JSONFraction {
    point: x! { "." },
    digits: Digits,
}

// original reference: crate::cluster::number::JSONExponent
#[shard]
pub struct JSONExponent {
    e: x! { {'E' 'e'} },
    sign: xopt![x! { {'+' '-'} }],
    digits: Digits,
}

// original reference: crate::cluster::number::Digits
#[shard]
type Digits = x! { Digit+ };

// original reference: crate::cluster::number::Digit
#[shard]
type Digit = x! {[
    | '0'
    | One2Nine
]};

// original reference: crate::cluster::number::One2Nine
#[shard]
type One2Nine = x! { {'1'..'9'} };

// ////////////////
// JSONNumber
// ////////////////

// ////////////////
// JSONBoolean
// ////////////////

// original reference: crate::cluster::boolean::JSONBoolean
#[shard]
pub enum JSONBoolean {
    True(JSONTrue),
    False(JSONFalse),
}

#[derive(Debug)]
pub enum JSONBoolean<'i> {
    True(JSONTrue<'i>),
    False(JSONFalse<'i>),
}

const _: () = {
    impl ::xst::internal::Shard for JSONBoolean<'static> {
        type Data = ::xst::internal::Alt<(
            ::xst::internal::Ext<JSONTrue<'static>>,
            ::xst::internal::Ext<JSONFalse<'static>>,
        )>;
    }

    impl ::xst::internal::StaticShard for JSONBoolean<'static> {}
};

// original reference: crate::cluster::boolean::JSONTrue
#[shard]
pub struct JSONTrue {
    text: x! { "true" },
}

#[derive(Debug)]
pub struct JSONTrue<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    text: &'i ::xst::internal::str,
}

const _: () = {
    impl ::xst::internal::Shard for JSONTrue<'static> {
        type Data = ::xst::internal::Lit<Literal0>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::str = "true";
    }

    impl ::xst::internal::StaticShard for JSONTrue<'static> {}
};

// original reference: crate::cluster::boolean::JSONFalse
#[shard]
pub struct JSONFalse {
    text: x! { "false" },
}

#[derive(Debug)]
pub struct JSONFalse<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    text: &'i ::xst::internal::str,
}

const _: () = {
    impl ::xst::internal::Shard for JSONFalse<'static> {
        type Data = ::xst::internal::Lit<Literal0>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::str = "false";
    }

    impl ::xst::internal::StaticShard for JSONFalse<'static> {}
};

// ////////////////
// JSONBoolean
// ////////////////

// ////////////////
// JSONNull
// ////////////////

// original reference: crate::cluster::null::JSONNull
#[shard]
pub struct JSONNull {
    text: x! { "null" },
}

#[derive(Debug)]
pub struct JSONNull<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    text: &'i ::xst::internal::str,
}

const _: () = {
    impl ::xst::internal::Shard for JSONNull<'static> {
        type Data = ::xst::internal::Lit<Literal0>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::str = "null";
    }

    impl ::xst::internal::StaticShard for JSONNull<'static> {}
};

// ////////////////
// JSONNull
// ////////////////

// ////////////////
// WS
// ////////////////

// original reference: crate::cluster::ws::WS
#[shard]
pub struct WS {
    space: x! { {' ' '\t' '\n' '\r'}* },
}

#[derive(Debug)]
pub struct WS<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    space: &'i ::xst::internal::str,
}

const _: () = {
    impl ::xst::internal::Shard for WS<'static> {
        type Data = ::xst::internal::Vec<::xst::internal::Set<false, Set0>, 0, 0>;
    }

    #[allow(dead_code)]
    pub struct Set0;

    impl ::xst::internal::ShardSet for Set0 {
        const SET: &'static [::xst::internal::RangeInclusive<::xst::internal::char>] =
            &[' '..=' ', '\t'..='\t', '\n'..='\n', '\r'..='\r'];
    }

    impl ::xst::internal::StaticShard for WS<'static> {}
};

// ////////////////
// WS
// ////////////////

// ////////////////
// Punctuated
// ////////////////

// original reference: crate::cluster::punctuated::Punctuated
#[shard]
pub struct Punctuated<T, P> {
    inner: xopt![(xbox![T], xvec![(P, T); ..])],
}

#[derive(Debug)]
pub struct Punctuated<'i, T, P> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    inner: Option<(Box<T>, Vec<(P, T)>)>,
}

const _: () = {
    impl<T, P> ::xst::internal::Shard for Punctuated<'static, T, P>
    where
        T: ::xst::internal::ShardParam,
        P: ::xst::internal::ShardParam,
    {
        type Data = ::xst::internal::Opt<
            ::xst::internal::Seq<(T, ::xst::internal::Vec<::xst::internal::Seq<(P, T)>, 0, 0>)>,
        >;
    }
};

// ////////////////
// Punctuated
// ////////////////

// ////////////////
// Spanned
// ////////////////

// original reference: crate::cluster::spanned::Spanned
#[shard]
pub struct Spanned {
    inner: T,
    ws: WS,
}

#[derive(Debug)]
pub struct Spanned<'i, T> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    inner: T,
    ws: WS<'i>,
}

const _: () = {
    impl<T> ::xst::internal::Shard for Spanned<'static, T>
    where
        T: ::xst::internal::ShardParam,
    {
        type Data = ::xst::internal::Seq<(T, ::xst::internal::Ext<WS<'static>>)>;
    }
};

// ////////////////
// Spanned
// ////////////////

#[cfg(test)]
mod test {
    use xst::Cluster;

    use super::JSON;

    #[test]
    fn test() {
        let cluster: Cluster<JSON> = Cluster::build();
    }
}
