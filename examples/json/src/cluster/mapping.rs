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

#[derive(::xst::internal::Debug)]
pub struct JSON<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    ws: (
        ::xst::internal::ShardField<'i, WS<'static>>,
        ::xst::internal::ShardField<'i, WS<'static>>,
    ),
    value: ::xst::internal::ShardField<'i, WS<'static>>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSON<'static> {}

    impl ::xst::internal::Shard for JSON<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
pub enum JSONValue<'i> {
    Object(::xst::internal::ShardField<'i, JSONObject<'static>>),
    Array(::xst::internal::ShardField<'i, JSONArray<'static>>),
    String(::xst::internal::ShardField<'i, JSONString<'static>>),
    Number(::xst::internal::ShardField<'i, JSONNumber<'static>>),
    Boolean(::xst::internal::ShardField<'i, JSONBoolean<'static>>),
    Null(::xst::internal::ShardField<'i, JSONNull<'static>>),
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONValue<'static> {}

    impl ::xst::internal::Shard for JSONValue<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
pub struct JSONObject<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    brace: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    ws: ::xst::internal::ShardField<'i, WS<'static>>,
    content: ::xst::internal::ShardField<
        'i,
        Punctuated<
            'static,
            Spanned<'static, JSONObjectEntry<'static>>,
            Spanned<'static, WS<'static> /* TODO */>,
        >,
    >,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONObject<'static> {}

    impl ::xst::internal::Shard for JSONObject<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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
                    Spanned<'static, WS<'static> /* TODO */>,
                >,
            >(),
            &::xst::internal::ShardData::literal("}"),
        ]);
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

#[derive(::xst::internal::Debug)]
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
    impl ::xst::internal::StaticShard for JSONObjectEntry<'static> {}

    impl ::xst::internal::Shard for JSONObjectEntry<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
pub struct JSONArray<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    bracket: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    ws: ::xst::internal::ShardField<'i, WS<'static>>,
    entries: ::xst::internal::ShardField<
        'i,
        Punctuated<
            'static,
            Spanned<'static, JSONValue<'static>>,
            Spanned<'static, WS<'static> /* TODO */>,
        >,
    >,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONArray<'static> {}

    impl ::xst::internal::Shard for JSONArray<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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
                    Spanned<'static, WS<'static> /* TODO */>,
                >,
            >(),
            &::xst::internal::ShardData::literal("]"),
        ]);
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

#[derive(::xst::internal::Debug)]
pub struct JSONString<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    quote: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    content: ::xst::internal::ShardField<'i, Content<'static>>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONString<'static> {}

    impl ::xst::internal::Shard for JSONString<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
struct Content<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::StaticShard for Content<'static> {}

    impl ::xst::internal::Shard for Content<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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
            0,
        );
    }
};

// original reference: crate::cluster::string::Escape
#[shard]
type Escape = x! {[
    | {'"' '\\' '/' 'b' 'f' 'n' 'r' 't'}
    | "u" {'0'..'9' 'A'..'F' 'a'..'f'}![4]
]};

#[derive(::xst::internal::Debug)]
struct Escape<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::StaticShard for Escape<'static> {}

    impl ::xst::internal::Shard for Escape<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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
                        4,
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

#[derive(::xst::internal::Debug)]
pub struct JSONNumber<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    sign: ::xst::internal::Option<&'i ::xst::internal::str>,
    integer: &'i ::xst::internal::str,
    fraction: ::xst::internal::Option<::xst::internal::ShardField<'i, JSONFraction<'static>>>,
    exponent: ::xst::internal::Option<::xst::internal::ShardField<'i, JSONExponent<'static>>>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONNumber<'static> {}

    impl ::xst::internal::Shard for JSONNumber<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
pub struct JSONFraction<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    point: &'i ::xst::internal::str,
    digits: ::xst::internal::ShardField<'i, Digits<'static>>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONFraction<'static> {}

    impl ::xst::internal::Shard for JSONFraction<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
pub struct JSONExponent<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    e: &'i ::xst::internal::str,
    sign: ::xst::internal::Option<&'i ::xst::internal::str>,
    digits: ::xst::internal::ShardField<'i, Digits<'static>>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONExponent<'static> {}

    impl ::xst::internal::Shard for JSONExponent<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
struct Digits<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::StaticShard for Digits<'static> {}

    impl ::xst::internal::Shard for Digits<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = &'i ::xst::internal::str;
        const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::vec(
            &::xst::internal::ShardData::reference::<Digit<'static>>(),
            1,
            0,
        );
    }
};

// original reference: crate::cluster::number::Digit
#[shard]
type Digit = x! {[
    | "0"
    | One2Nine
]};

#[derive(::xst::internal::Debug)]
struct Digit<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::StaticShard for Digit<'static> {}

    impl ::xst::internal::Shard for Digit<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
struct One2Nine<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::StaticShard for One2Nine<'static> {}

    impl ::xst::internal::Shard for One2Nine<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
pub enum JSONBoolean<'i> {
    True(::xst::internal::ShardField<'i, JSONBoolean<'static>>),
    False(::xst::internal::ShardField<'i, JSONBoolean<'static>>),
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONBoolean<'static> {}

    impl ::xst::internal::Shard for JSONBoolean<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
pub struct JSONTrue<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    text: &'i ::xst::internal::str,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONTrue<'static> {}

    impl ::xst::internal::Shard for JSONTrue<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
pub struct JSONFalse<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    text: &'i ::xst::internal::str,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONFalse<'static> {}

    impl ::xst::internal::Shard for JSONFalse<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
pub struct JSONNull<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    text: &'i ::xst::internal::str,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONNull<'static> {}

    impl ::xst::internal::Shard for JSONNull<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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

#[derive(::xst::internal::Debug)]
pub struct WS<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    space: &'i ::xst::internal::str,
}

const _: () = {
    impl ::xst::internal::StaticShard for WS<'static> {}

    impl ::xst::internal::Shard for WS<'static> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
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
            0,
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

#[derive(::xst::internal::Debug)]
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
    impl<T: ::xst::internal::Shard, P: ::xst::internal::Shard> ::xst::internal::Shard
        for Punctuated<'static, T, P>
    {
        type Core = __xst_shard_core_0<T, P>;
    }

    #[allow(dead_code)]
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
                    0,
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
pub struct Spanned {
    inner: T,
    ws: WS,
}

#[derive(::xst::internal::Debug)]
pub struct Spanned<'i, T: ::xst::internal::Shard> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    inner: ::xst::internal::ShardField<'i, T>,
    ws: ::xst::internal::ShardField<'i, WS<'static>>,
}

const _: () = {
    impl<T: ::xst::internal::Shard> ::xst::internal::Shard for Spanned<'static, T> {
        type Core = __xst_shard_core_0<T>;
    }

    #[allow(dead_code)]
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
