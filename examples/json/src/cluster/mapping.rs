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

pub struct JSON<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    ws: (
        ::xst::internal::ShardField<'i, WS<'static>>,
        ::xst::internal::ShardField<'i, WS<'static>>,
    ),
    value: ::xst::internal::ShardField<'i, JSONValue<'static>>,
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSON<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("JSON")
                .field("ws", &self.ws.0)
                .field("value", &self.value)
                .field("ws", &self.ws.1)
                .finish()
        }
    }

    impl ::xst::internal::StaticShard for JSON<'static> {}

    impl ::xst::internal::Shard for JSON<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSON<'i>;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::sequence(&[
            &::xst::internal::ShardData::reference::<WS<'static>>(),
            &::xst::internal::ShardData::reference::<JSONValue<'static>>(),
            &::xst::internal::ShardData::reference::<WS<'static>>(),
        ]);
    }
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

pub enum JSONValue<'i> {
    Object(::xst::internal::ShardField<'i, JSONObject<'static>>),
    Array(::xst::internal::ShardField<'i, JSONArray<'static>>),
    String(::xst::internal::ShardField<'i, JSONString<'static>>),
    Number(::xst::internal::ShardField<'i, JSONNumber<'static>>),
    Boolean(::xst::internal::ShardField<'i, JSONBoolean<'static>>),
    Null(::xst::internal::ShardField<'i, JSONNull<'static>>),
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSONValue<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            match self {
                Self::Object(value) => f.debug_tuple("Object").field(value).finish(),
                Self::Array(value) => f.debug_tuple("Array").field(value).finish(),
                Self::String(value) => f.debug_tuple("String").field(value).finish(),
                Self::Number(value) => f.debug_tuple("Number").field(value).finish(),
                Self::Boolean(value) => f.debug_tuple("Boolean").field(value).finish(),
                Self::Null(value) => f.debug_tuple("Null").field(value).finish(),
            }
        }
    }

    impl ::xst::internal::StaticShard for JSONValue<'static> {}

    impl ::xst::internal::Shard for JSONValue<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONValue<'i>;
        const DATA: &'static ::xst::internal::ShardData =
            &::xst::internal::ShardData::alternative(&[
                &::xst::internal::ShardData::reference::<JSONObject<'static>>(),
                &::xst::internal::ShardData::reference::<JSONArray<'static>>(),
                &::xst::internal::ShardData::reference::<JSONString<'static>>(),
                &::xst::internal::ShardData::reference::<JSONNumber<'static>>(),
                &::xst::internal::ShardData::reference::<JSONBoolean<'static>>(),
                &::xst::internal::ShardData::reference::<JSONNull<'static>>(),
            ]);
    }
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

pub struct JSONObject<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    brace: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    ws: ::xst::internal::ShardField<'i, WS<'static>>,
    content: ::xst::internal::ShardField<
        'i,
        Punctuated<
            'static,
            Spanned<'static, JSONObjectEntry<'static>>,
            Spanned<'static, ::xst::internal::ShardClosure<JSONObject<'static>, 0>>,
        >,
    >,
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSONObject<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("JSONObject")
                .field("brace", &self.brace.0)
                .field("ws", &self.ws)
                .field("content", &self.content)
                .field("brace", &self.brace.1)
                .finish()
        }
    }

    impl ::xst::internal::StaticShard for JSONObject<'static> {}

    impl ::xst::internal::Shard for JSONObject<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONObject<'i>;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::sequence(&[
            &::xst::internal::ShardData::literal("{"),
            &::xst::internal::ShardData::reference::<WS<'static>>(),
            &::xst::internal::ShardData::reference::<
                Punctuated<
                    'static,
                    Spanned<'static, JSONObjectEntry<'static>>,
                    Spanned<'static, __xst_shard_closure_0>,
                >,
            >(),
            &::xst::internal::ShardData::literal("}"),
        ]);
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub struct __xst_shard_closure_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::Shard for __xst_shard_closure_0 {
        type Core = Self;
    }

    impl ::xst::internal::ShardCore for __xst_shard_closure_0 {
        type Output<'i> = &'i ::xst::internal::str;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::literal(",");
    }

    impl ::xst::internal::ShardClosureForward<0> for __xst_shard_core_0 {
        type Closure = __xst_shard_closure_0;
    }
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

pub struct JSONObjectEntry<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    name: ::xst::internal::ShardField<'i, JSONString<'static>>,
    ws: (
        ::xst::internal::ShardField<'i, WS<'static>>,
        ::xst::internal::ShardField<'i, WS<'static>>,
    ),
    colon: &'i ::xst::internal::str,
    value: ::xst::internal::ShardField<'i, JSONValue<'static>>,
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSONObjectEntry<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("JSONObjectEntry")
                .field("name", &self.name)
                .field("ws", &self.ws.0)
                .field("colon", &self.colon)
                .field("ws", &self.ws.1)
                .field("value", &self.value)
                .finish()
        }
    }

    impl ::xst::internal::StaticShard for JSONObjectEntry<'static> {}

    impl ::xst::internal::Shard for JSONObjectEntry<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONObjectEntry<'i>;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::sequence(&[
            &::xst::internal::ShardData::reference::<JSONString<'static>>(),
            &::xst::internal::ShardData::reference::<WS<'static>>(),
            &::xst::internal::ShardData::literal(":"),
            &::xst::internal::ShardData::reference::<WS<'static>>(),
            &::xst::internal::ShardData::reference::<JSONValue<'static>>(),
        ]);
    }
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
    bracket: x! { "]" },
}

pub struct JSONArray<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    bracket: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    ws: ::xst::internal::ShardField<'i, WS<'static>>,
    entries: ::xst::internal::ShardField<
        'i,
        Punctuated<
            'static,
            Spanned<'static, JSONValue<'static>>,
            Spanned<'static, ::xst::internal::ShardClosure<JSONArray<'static>, 0>>,
        >,
    >,
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSONArray<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("JSONArray")
                .field("bracket", &self.bracket.0)
                .field("ws", &self.ws)
                .field("entries", &self.entries)
                .field("bracket", &self.bracket.1)
                .finish()
        }
    }

    impl ::xst::internal::StaticShard for JSONArray<'static> {}

    impl ::xst::internal::Shard for JSONArray<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONArray<'i>;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::sequence(&[
            &::xst::internal::ShardData::literal("["),
            &::xst::internal::ShardData::reference::<WS<'static>>(),
            &::xst::internal::ShardData::reference::<
                Punctuated<
                    'static,
                    Spanned<'static, JSONValue<'static>>,
                    Spanned<'static, __xst_shard_closure_0>,
                >,
            >(),
            &::xst::internal::ShardData::literal("]"),
        ]);
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_closure_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::Shard for __xst_shard_closure_0 {
        type Core = Self;
    }

    impl ::xst::internal::ShardCore for __xst_shard_closure_0 {
        type Output<'i> = &'i ::xst::internal::str;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::literal(",");
    }

    impl ::xst::internal::ShardClosureForward<0> for __xst_shard_core_0 {
        type Closure = __xst_shard_closure_0;
    }
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

pub struct JSONString<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    quote: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    content: ::xst::internal::ShardField<'i, Content<'static>>,
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSONString<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("JSONString")
                .field("quote", &self.quote.0)
                .field("content", &self.content)
                .field("quote", &self.quote.1)
                .finish()
        }
    }

    impl ::xst::internal::StaticShard for JSONString<'static> {}

    impl ::xst::internal::Shard for JSONString<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONString<'i>;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::sequence(&[
            &::xst::internal::ShardData::literal("\""),
            &::xst::internal::ShardData::reference::<Content<'static>>(),
            &::xst::internal::ShardData::literal("\""),
        ]);
    }
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
    impl ::xst::internal::StaticShard for Content<'static> {}

    impl ::xst::internal::Shard for Content<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = &'i ::xst::internal::str;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::vec(
            &::xst::internal::ShardData::alternative(&[
                &::xst::internal::ShardData::set(
                    true,
                    &['"'..='"', '\\'..='\\', '\u{0000}'..='\u{001F}'],
                ),
                &::xst::internal::ShardData::sequence(&[
                    &::xst::internal::ShardData::literal("\\"),
                    &::xst::internal::ShardData::reference::<Escape<'static>>(),
                ]),
            ]),
            0,
            ::xst::internal::Option::None,
        );
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
    impl ::xst::internal::StaticShard for Escape<'static> {}

    impl ::xst::internal::Shard for Escape<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = &'i ::xst::internal::str;
        const DATA: &'static ::xst::internal::ShardData =
            &::xst::internal::ShardData::alternative(&[
                &::xst::internal::ShardData::set(
                    false,
                    &[
                        '"'..='"',
                        '\\'..='\\',
                        '/'..='/',
                        'b'..='b',
                        'f'..='f',
                        'n'..='n',
                        'r'..='r',
                        't'..='t',
                    ],
                ),
                &::xst::internal::ShardData::sequence(&[
                    &::xst::internal::ShardData::literal("u"),
                    &::xst::internal::ShardData::vec(
                        &::xst::internal::ShardData::set(false, &['0'..='9', 'A'..='F', 'a'..='f']),
                        4,
                        ::xst::internal::Option::Some(4),
                    ),
                ]),
            ]);
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

pub struct JSONNumber<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    sign: ::xst::internal::Option<&'i ::xst::internal::str>,
    integer: &'i ::xst::internal::str,
    fraction: ::xst::internal::Option<::xst::internal::ShardField<'i, JSONFraction<'static>>>,
    exponent: ::xst::internal::Option<::xst::internal::ShardField<'i, JSONExponent<'static>>>,
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSONNumber<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("JSONNumber")
                .field("sign", &self.sign)
                .field("integer", &self.integer)
                .field("fraction", &self.fraction)
                .field("exponent", &self.exponent)
                .finish()
        }
    }

    impl ::xst::internal::StaticShard for JSONNumber<'static> {}

    impl ::xst::internal::Shard for JSONNumber<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONNumber<'i>;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::sequence(&[
            &::xst::internal::ShardData::option(&::xst::internal::ShardData::literal("-")),
            &::xst::internal::ShardData::alternative(&[
                &::xst::internal::ShardData::reference::<Digit<'static>>(),
                &::xst::internal::ShardData::sequence(&[
                    &::xst::internal::ShardData::reference::<One2Nine<'static>>(),
                    &::xst::internal::ShardData::reference::<Digits<'static>>(),
                ]),
            ]),
            &::xst::internal::ShardData::option(&::xst::internal::ShardData::reference::<
                JSONFraction<'static>,
            >()),
            &::xst::internal::ShardData::option(&::xst::internal::ShardData::reference::<
                JSONExponent<'static>,
            >()),
        ]);
    }
};

// original reference: crate::cluster::number::JSONFraction
#[shard]
pub struct JSONFraction {
    point: x! { "." },
    digits: Digits,
}

pub struct JSONFraction<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    point: &'i ::xst::internal::str,
    digits: ::xst::internal::ShardField<'i, Digits<'static>>,
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSONFraction<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("JSONFraction")
                .field("point", &self.point)
                .field("digits", &self.digits)
                .finish()
        }
    }

    impl ::xst::internal::StaticShard for JSONFraction<'static> {}

    impl ::xst::internal::Shard for JSONFraction<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONFraction<'i>;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::sequence(&[
            &::xst::internal::ShardData::literal("."),
            &::xst::internal::ShardData::reference::<Digits<'static>>(),
        ]);
    }
};

// original reference: crate::cluster::number::JSONExponent
#[shard]
pub struct JSONExponent {
    e: x! { {'E' 'e'} },
    sign: xopt![x! { {'+' '-'} }],
    digits: Digits,
}

pub struct JSONExponent<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    e: &'i ::xst::internal::str,
    sign: ::xst::internal::Option<&'i ::xst::internal::str>,
    digits: ::xst::internal::ShardField<'i, Digits<'static>>,
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSONExponent<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("JSONExponent")
                .field("e", &self.e)
                .field("sign", &self.sign)
                .field("digits", &self.digits)
                .finish()
        }
    }

    impl ::xst::internal::StaticShard for JSONExponent<'static> {}

    impl ::xst::internal::Shard for JSONExponent<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONExponent<'i>;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::sequence(&[
            &::xst::internal::ShardData::set(false, &['E'..='E', 'e'..='e']),
            &::xst::internal::ShardData::option(&::xst::internal::ShardData::set(
                false,
                &['+'..='+', '-'..='-'],
            )),
            &::xst::internal::ShardData::reference::<Digits<'static>>(),
        ]);
    }
};

// original reference: crate::cluster::number::Digits
#[shard]
type Digits = x! { Digit+ };

struct Digits<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::StaticShard for Digits<'static> {}

    impl ::xst::internal::Shard for Digits<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = &'i ::xst::internal::str;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::vec(
            &::xst::internal::ShardData::reference::<Digit<'static>>(),
            1,
            ::xst::internal::Option::None,
        );
    }
};

// original reference: crate::cluster::number::Digit
#[shard]
type Digit = x! {[
    | "0"
    | One2Nine
]};

struct Digit<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::StaticShard for Digit<'static> {}

    impl ::xst::internal::Shard for Digit<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = &'i ::xst::internal::str;
        const DATA: &'static ::xst::internal::ShardData =
            &::xst::internal::ShardData::alternative(&[
                &::xst::internal::ShardData::literal("0"),
                &::xst::internal::ShardData::reference::<One2Nine<'static>>(),
            ]);
    }
};

// original reference: crate::cluster::number::One2Nine
#[shard]
type One2Nine = x! { {'1'..'9'} };

struct One2Nine<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::StaticShard for One2Nine<'static> {}

    impl ::xst::internal::Shard for One2Nine<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = &'i ::xst::internal::str;
        const DATA: &'static ::xst::internal::ShardData =
            &::xst::internal::ShardData::set(false, &['1'..='9']);
    }
};

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

pub enum JSONBoolean<'i> {
    True(::xst::internal::ShardField<'i, JSONTrue<'static>>),
    False(::xst::internal::ShardField<'i, JSONFalse<'static>>),
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSONBoolean<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            match self {
                Self::True(value) => f.debug_tuple("True").field(value).finish(),
                Self::False(value) => f.debug_tuple("False").field(value).finish(),
            }
        }
    }

    impl ::xst::internal::StaticShard for JSONBoolean<'static> {}

    impl ::xst::internal::Shard for JSONBoolean<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONBoolean<'i>;
        const DATA: &'static ::xst::internal::ShardData =
            &::xst::internal::ShardData::alternative(&[
                &::xst::internal::ShardData::reference::<JSONTrue<'static>>(),
                &::xst::internal::ShardData::reference::<JSONFalse<'static>>(),
            ]);
    }
};

// original reference: crate::cluster::boolean::JSONTrue
#[shard]
pub struct JSONTrue {
    text: x! { "true" },
}

pub struct JSONTrue<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    text: &'i ::xst::internal::str,
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSONTrue<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("JSONTrue")
                .field("text", &self.text)
                .finish()
        }
    }

    impl ::xst::internal::StaticShard for JSONTrue<'static> {}

    impl ::xst::internal::Shard for JSONTrue<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONTrue<'i>;
        const DATA: &'static ::xst::internal::ShardData =
            &::xst::internal::ShardData::literal("true");
    }
};

// original reference: crate::cluster::boolean::JSONFalse
#[shard]
pub struct JSONFalse {
    text: x! { "false" },
}

pub struct JSONFalse<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    text: &'i ::xst::internal::str,
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSONFalse<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("JSONFalse")
                .field("text", &self.text)
                .finish()
        }
    }

    impl ::xst::internal::StaticShard for JSONFalse<'static> {}

    impl ::xst::internal::Shard for JSONFalse<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONFalse<'i>;
        const DATA: &'static ::xst::internal::ShardData =
            &::xst::internal::ShardData::literal("false");
    }
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

pub struct JSONNull<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    text: &'i ::xst::internal::str,
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for JSONNull<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("JSONNull")
                .field("text", &self.text)
                .finish()
        }
    }

    impl ::xst::internal::StaticShard for JSONNull<'static> {}

    impl ::xst::internal::Shard for JSONNull<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONNull<'i>;
        const DATA: &'static ::xst::internal::ShardData =
            &::xst::internal::ShardData::literal("null");
    }
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

pub struct WS<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    space: &'i ::xst::internal::str,
}

const _: () = {
    impl<'i> ::xst::internal::fmt::Debug for WS<'i> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("WS").field("space", &self.space).finish()
        }
    }

    impl ::xst::internal::StaticShard for WS<'static> {}

    impl ::xst::internal::Shard for WS<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = WS<'i>;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::vec(
            &::xst::internal::ShardData::set(
                false,
                &[' '..=' ', '\t'..='\t', '\n'..='\n', '\r'..='\r'],
            ),
            0,
            ::xst::internal::Option::None,
        );
    }
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
    inner: xopt![(xbox![T], xvec![(P, T) | ..])],
}

pub struct Punctuated<'i, T: ::xst::internal::Shard, P: ::xst::internal::Shard> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    inner: ::xst::internal::Option<(
        ::xst::internal::Box<::xst::internal::ShardField<'i, T>>,
        ::xst::internal::Vec<(
            ::xst::internal::ShardField<'i, P>,
            ::xst::internal::ShardField<'i, T>,
        )>,
    )>,
}

const _: () = {
    impl<'i, T: ::xst::internal::Shard, P: ::xst::internal::Shard> ::xst::internal::fmt::Debug
        for Punctuated<'i, T, P>
    {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("Punctuated")
                .field("inner", &self.inner)
                .finish()
        }
    }

    impl<T: ::xst::internal::Shard, P: ::xst::internal::Shard> ::xst::internal::Shard
        for Punctuated<'static, T, P>
    {
        type Core = __xst_shard_core_0<T, P>;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0<T: ::xst::internal::Shard, P: ::xst::internal::Shard>(
        ::xst::internal::PhantomData<fn() -> (T, P)>,
    );

    impl<T: ::xst::internal::Shard, P: ::xst::internal::Shard> ::xst::internal::ShardCore
        for __xst_shard_core_0<T, P>
    {
        type Output<'i> = Punctuated<'i, T, P>;
        const DATA: &'static ::xst::internal::ShardData =
            &::xst::internal::ShardData::option(&::xst::internal::ShardData::sequence(&[
                &::xst::internal::ShardData::reference::<T>(),
                &::xst::internal::ShardData::vec(
                    &::xst::internal::ShardData::sequence(&[
                        &::xst::internal::ShardData::reference::<P>(),
                        &::xst::internal::ShardData::reference::<T>(),
                    ]),
                    0,
                    ::xst::internal::Option::None,
                ),
            ]));
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
pub struct Spanned<T> {
    inner: T,
    ws: WS,
}

pub struct Spanned<'i, T: ::xst::internal::Shard> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    inner: ::xst::internal::ShardField<'i, T>,
    ws: ::xst::internal::ShardField<'i, WS<'static>>,
}

const _: () = {
    impl<'i, T: ::xst::internal::Shard> ::xst::internal::fmt::Debug for Spanned<'i, T> {
        fn fmt(&self, f: &mut ::xst::internal::fmt::Formatter<'_>) -> ::xst::internal::fmt::Result {
            f.debug_struct("Spanned")
                .field("inner", &self.inner)
                .field("ws", &self.ws)
                .finish()
        }
    }

    impl<T: ::xst::internal::Shard> ::xst::internal::Shard for Spanned<'static, T> {
        type Core = __xst_shard_core_0<T>;
    }

    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0<T: ::xst::internal::Shard>(
        ::xst::internal::PhantomData<fn() -> (T,)>,
    );

    impl<T: ::xst::internal::Shard> ::xst::internal::ShardCore for __xst_shard_core_0<T> {
        type Output<'i> = Spanned<'i, T>;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::sequence(&[
            &::xst::internal::ShardData::reference::<T>(),
            &::xst::internal::ShardData::reference::<WS<'static>>(),
        ]);
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
