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
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    ws: (
        ::xst::internal::Output<'i, WS<'static>>,
        ::xst::internal::Output<'i, WS<'static>>,
    ),
    value: ::xst::internal::Output<'i, JSONValue<'static>>,
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSON<'i> {}

    impl<'i> ::xst::internal::Shard for JSON<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSON<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Ext<WS<'static>>,
            ::xst::internal::Ext<JSONValue<'static>>,
            ::xst::internal::Ext<WS<'static>>,
        )>;
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

#[derive(Debug)]
pub enum JSONValue<'i> {
    Object(::xst::internal::Output<'i, JSONObject<'static>>),
    Array(::xst::internal::Output<'i, JSONArray<'static>>),
    String(::xst::internal::Output<'i, JSONString<'static>>),
    Number(::xst::internal::Output<'i, JSONNumber<'static>>),
    Boolean(::xst::internal::Output<'i, JSONBoolean<'static>>),
    Null(::xst::internal::Output<'i, JSONNull<'static>>),
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSONValue<'i> {}

    impl<'i> ::xst::internal::Shard for JSONValue<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONValue<'i>;
        type Data = ::xst::internal::Alt<(
            ::xst::internal::Ext<JSONObject<'static>>,
            ::xst::internal::Ext<JSONArray<'static>>,
            ::xst::internal::Ext<JSONString<'static>>,
            ::xst::internal::Ext<JSONNumber<'static>>,
            ::xst::internal::Ext<JSONBoolean<'static>>,
            ::xst::internal::Ext<JSONNull<'static>>,
        )>;
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

#[derive(Debug)]
pub struct JSONObject<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    brace: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    ws: ::xst::internal::Output<'i, WS<'static>>,
    content: ::xst::internal::Output<
        'i,
        Punctuated<
            'static,
            ::xst::internal::Ext<Spanned<'static, ::xst::internal::Ext<JSONObjectEntry<'static>>>>,
            ::xst::internal::Ext<Spanned<'static, ::xst::internal::Lit<__xst_shard_literal_1>>>,
        >,
    >,
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSONObject<'i> {}

    impl<'i> ::xst::internal::Shard for JSONObject<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONObject<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Lit<__xst_shard_literal_0>,
            ::xst::internal::Ext<WS<'static>>,
            ::xst::internal::Ext<
                Punctuated<
                    'static,
                    ::xst::internal::Ext<
                        Spanned<'static, ::xst::internal::Ext<JSONObjectEntry<'static>>>,
                    >,
                    ::xst::internal::Ext<
                        Spanned<'static, ::xst::internal::Lit<__xst_shard_literal_1>>,
                    >,
                >,
            >,
            ::xst::internal::Lit<__xst_shard_literal_2>,
        )>;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_0;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_0 {
        const LITERAL: &'static ::xst::internal::str = "{";
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_1;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_1 {
        const LITERAL: &'static ::xst::internal::str = ",";
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_2;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_2 {
        const LITERAL: &'static ::xst::internal::str = "}";
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

#[derive(Debug)]
pub struct JSONObjectEntry<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    name: ::xst::internal::FieldOutput<'i, JSONObjectEntry<'static>, 0>,
    ws: (
        ::xst::internal::FieldOutput<'i, JSONObjectEntry<'static>, 1>,
        ::xst::internal::FieldOutput<'i, JSONObjectEntry<'static>, 3>,
    ),
    colon: ::xst::internal::FieldOutput<'i, JSONObjectEntry<'static>, 2>,
    value: ::xst::internal::FieldOutput<'i, JSONObjectEntry<'static>, 4>,
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSONObjectEntry<'i> {}

    impl<'i> ::xst::internal::Shard for JSONObjectEntry<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONObjectEntry<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Ext<JSONString<'static>>,
            ::xst::internal::Ext<WS<'static>>,
            ::xst::internal::Lit<__xst_shard_literal_0>,
            ::xst::internal::Ext<WS<'static>>,
            ::xst::internal::Ext<JSONValue<'static>>,
        )>;
    }

    impl ::xst::internal::ShardField<0> for JSONObjectEntry<'static> {
        type Output<'i> = ::xst::internal::Output<'i, JSONString<'static>>;
    }

    impl ::xst::internal::ShardField<1> for JSONObjectEntry<'static> {
        type Output<'i> = ::xst::internal::Output<'i, WS<'static>>;
    }

    impl ::xst::internal::ShardField<2> for JSONObjectEntry<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    impl ::xst::internal::ShardField<3> for JSONObjectEntry<'static> {
        type Output<'i> = ::xst::internal::Output<'i, WS<'static>>;
    }

    impl ::xst::internal::ShardField<4> for JSONObjectEntry<'static> {
        type Output<'i> = ::xst::internal::Output<'i, JSONValue<'static>>;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_0;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_0 {
        const LITERAL: &'static ::xst::internal::str = ":";
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

#[derive(Debug)]
pub struct JSONArray<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    bracket: (
        ::xst::internal::FieldOutput<'i, JSONArray<'static>, 0>,
        ::xst::internal::FieldOutput<'i, JSONArray<'static>, 3>,
    ),
    ws: ::xst::internal::FieldOutput<'i, JSONArray<'static>, 1>,
    entries: ::xst::internal::FieldOutput<'i, JSONArray<'static>, 2>,
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSONArray<'i> {}

    impl<'i> ::xst::internal::Shard for JSONArray<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONArray<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Lit<__xst_shard_literal_0>,
            ::xst::internal::Ext<WS<'static>>,
            ::xst::internal::Ext<
                Punctuated<
                    'static,
                    ::xst::internal::Ext<
                        Spanned<'static, ::xst::internal::Ext<JSONValue<'static>>>,
                    >,
                    ::xst::internal::Ext<
                        Spanned<'static, ::xst::internal::Lit<__xst_shard_literal_1>>,
                    >,
                >,
            >,
            ::xst::internal::Lit<__xst_shard_literal_2>,
        )>;
    }

    impl ::xst::internal::ShardField<0> for JSONArray<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    impl ::xst::internal::ShardField<1> for JSONArray<'static> {
        type Output<'i> = ::xst::internal::Output<'i, WS<'static>>;
    }

    impl ::xst::internal::ShardField<2> for JSONArray<'static> {
        type Output<'i> = ::xst::internal::Output<
            'i,
            Punctuated<
                'static,
                ::xst::internal::Ext<Spanned<'static, ::xst::internal::Ext<JSONValue<'static>>>>,
                ::xst::internal::Ext<Spanned<'static, ::xst::internal::Lit<__xst_shard_literal_1>>>,
            >,
        >;
    }

    impl ::xst::internal::ShardField<3> for JSONArray<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_0;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_0 {
        const LITERAL: &'static ::xst::internal::str = "[";
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_1;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_1 {
        const LITERAL: &'static ::xst::internal::str = ",";
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_2;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_2 {
        const LITERAL: &'static ::xst::internal::str = "]";
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

#[derive(Debug)]
pub struct JSONString<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    quote: (
        ::xst::internal::FieldOutput<'i, JSONString<'static>, 0>,
        ::xst::internal::FieldOutput<'i, JSONString<'static>, 2>,
    ),
    content: ::xst::internal::FieldOutput<'i, JSONString<'static>, 1>,
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSONString<'i> {}

    impl<'i> ::xst::internal::Shard for JSONString<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONString<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Lit<__xst_shard_literal_0>,
            ::xst::internal::Ext<Content<'static>>,
            ::xst::internal::Lit<__xst_shard_literal_1>,
        )>;
    }

    impl ::xst::internal::ShardField<0> for JSONString<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    impl ::xst::internal::ShardField<1> for JSONString<'static> {
        type Output<'i> = ::xst::internal::Output<'i, Content<'static>>;
    }

    impl ::xst::internal::ShardField<2> for JSONString<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_0;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_0 {
        const LITERAL: &'static ::xst::internal::str = "\"";
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_1;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_1 {
        const LITERAL: &'static ::xst::internal::str = "\"";
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

#[doc(hidden)]
pub struct Content<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl<'i> ::xst::internal::StaticShard for Content<'i> {}

    impl<'i> ::xst::internal::Shard for Content<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = &'i ::xst::internal::str;
        type Data = ::xst::internal::Vec<
            ::xst::internal::Alt<(
                ::xst::internal::Set<true, __xst_shard_set_0>,
                ::xst::internal::Seq<(
                    ::xst::internal::Lit<__xst_shard_literal_1>,
                    ::xst::internal::Ext<Escape<'static>>,
                )>,
            )>,
            0,
            { ::core::primitive::usize::MAX },
        >;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_set_0;

    impl ::xst::internal::ShardSet for __xst_shard_set_0 {
        const SET: &'static [::xst::internal::RangeInclusive<::xst::internal::char>] =
            &['"'..='"', '\\'..='\\', '\u{0000}'..='\u{001F}'];
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_1;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_1 {
        const LITERAL: &'static ::xst::internal::str = "\\";
    }
};

// original reference: crate::cluster::string::Escape
#[shard]
type Escape = x! {[
    | {'"' '\\' '/' 'b' 'f' 'n' 'r' 't'}
    | "u" {'0'..'9' 'A'..'F' 'a'..'f'}![4]
]};

#[doc(hidden)]
pub struct Escape<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl<'i> ::xst::internal::StaticShard for Escape<'i> {}

    impl<'i> ::xst::internal::Shard for Escape<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = &'i ::xst::internal::str;
        type Data = ::xst::internal::Alt<(
            ::xst::internal::Set<false, __xst_shard_set_0>,
            ::xst::internal::Seq<(
                ::xst::internal::Lit<__xst_shard_literal_1>,
                ::xst::internal::Vec<::xst::internal::Set<false, __xst_shard_set_2>, 4, 4>,
            )>,
        )>;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_set_0;

    impl ::xst::internal::ShardSet for __xst_shard_set_0 {
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
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_1;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_1 {
        const LITERAL: &'static ::xst::internal::str = "u";
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_set_2;

    impl ::xst::internal::ShardSet for __xst_shard_set_2 {
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

#[derive(Debug)]
pub struct JSONNumber<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    sign: ::xst::internal::FieldOutput<'i, JSONNumber<'static>, 0>,
    integer: ::xst::internal::FieldOutput<'i, JSONNumber<'static>, 1>,
    fraction: ::xst::internal::FieldOutput<'i, JSONNumber<'static>, 2>,
    exponent: ::xst::internal::FieldOutput<'i, JSONNumber<'static>, 3>,
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSONNumber<'i> {}

    impl<'i> ::xst::internal::Shard for JSONNumber<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONNumber<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Opt<::xst::internal::Lit<__xst_shard_literal_0>>,
            ::xst::internal::Alt<(
                ::xst::internal::Ext<Digit<'static>>,
                ::xst::internal::Seq<(
                    ::xst::internal::Ext<One2Nine<'static>>,
                    ::xst::internal::Ext<Digits<'static>>,
                )>,
            )>,
            ::xst::internal::Opt<::xst::internal::Ext<JSONFraction<'static>>>,
            ::xst::internal::Opt<::xst::internal::Ext<JSONExponent<'static>>>,
        )>;
    }

    impl ::xst::internal::ShardField<0> for JSONNumber<'static> {
        type Output<'i> = Option<&'i ::xst::internal::str>;
    }

    impl ::xst::internal::ShardField<1> for JSONNumber<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    impl ::xst::internal::ShardField<2> for JSONNumber<'static> {
        type Output<'i> = Option<::xst::internal::Output<'i, JSONFraction<'static>>>;
    }

    impl ::xst::internal::ShardField<3> for JSONNumber<'static> {
        type Output<'i> = Option<::xst::internal::Output<'i, JSONExponent<'static>>>;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_0;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_0 {
        const LITERAL: &'static ::xst::internal::str = "-";
    }
};

// original reference: crate::cluster::number::JSONFraction
#[shard]
pub struct JSONFraction {
    point: x! { "." },
    digits: Digits,
}

#[derive(Debug)]
pub struct JSONFraction<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    point: ::xst::internal::FieldOutput<'i, JSONFraction<'static>, 0>,
    digits: ::xst::internal::FieldOutput<'i, JSONFraction<'static>, 1>,
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSONFraction<'i> {}

    impl<'i> ::xst::internal::Shard for JSONFraction<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONFraction<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Lit<__xst_shard_literal_0>,
            ::xst::internal::Ext<Digits<'static>>,
        )>;
    }

    impl ::xst::internal::ShardField<0> for JSONFraction<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    impl ::xst::internal::ShardField<1> for JSONFraction<'static> {
        type Output<'i> = ::xst::internal::Output<'i, Digits<'static>>;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_0;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_0 {
        const LITERAL: &'static ::xst::internal::str = ".";
    }
};

// original reference: crate::cluster::number::JSONExponent
#[shard]
pub struct JSONExponent {
    e: x! { {'E' 'e'} },
    sign: xopt![x! { {'+' '-'} }],
    digits: Digits,
}

#[derive(Debug)]
pub struct JSONExponent<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    e: ::xst::internal::FieldOutput<'i, JSONExponent<'static>, 0>,
    sign: ::xst::internal::FieldOutput<'i, JSONExponent<'static>, 1>,
    digits: ::xst::internal::FieldOutput<'i, JSONExponent<'static>, 2>,
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSONExponent<'i> {}

    impl<'i> ::xst::internal::Shard for JSONExponent<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONExponent<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Set<false, __xst_shard_set_0>,
            ::xst::internal::Opt<::xst::internal::Set<false, __xst_shard_set_1>>,
            ::xst::internal::Ext<Digits<'static>>,
        )>;
    }

    impl ::xst::internal::ShardField<0> for JSONExponent<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    impl ::xst::internal::ShardField<1> for JSONExponent<'static> {
        type Output<'i> = Option<&'i ::xst::internal::str>;
    }

    impl ::xst::internal::ShardField<2> for JSONExponent<'static> {
        type Output<'i> = ::xst::internal::Output<'i, Digits<'static>>;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_set_0;

    impl ::xst::internal::ShardSet for __xst_shard_set_0 {
        const SET: &'static [::xst::internal::RangeInclusive<::xst::internal::char>] =
            &['E'..='E', 'e'..='e'];
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_set_1;

    impl ::xst::internal::ShardSet for __xst_shard_set_1 {
        const SET: &'static [::xst::internal::RangeInclusive<::xst::internal::char>] =
            &['+'..='+', '-'..='-'];
    }
};

// original reference: crate::cluster::number::Digits
#[shard]
type Digits = x! { Digit+ };

#[doc(hidden)]
pub struct Digits<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl<'i> ::xst::internal::StaticShard for Digits<'i> {}

    impl<'i> ::xst::internal::Shard for Digits<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = &'i ::xst::internal::str;
        type Data = ::xst::internal::Vec<
            ::xst::internal::Ext<Digit<'static>>,
            1,
            { ::core::primitive::usize::MAX },
        >;
    }
};

// original reference: crate::cluster::number::Digit
#[shard]
type Digit = x! {[
    | '0'
    | One2Nine
]};

#[doc(hidden)]
pub struct Digit<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl<'i> ::xst::internal::StaticShard for Digit<'i> {}

    impl<'i> ::xst::internal::Shard for Digit<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = &'i ::xst::internal::str;
        type Data = ::xst::internal::Alt<(
            ::xst::internal::Lit<__xst_shard_literal_0>,
            ::xst::internal::Ext<One2Nine<'static>>,
        )>;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_0;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_0 {
        const LITERAL: &'static ::xst::internal::str = "0";
    }
};

// original reference: crate::cluster::number::One2Nine
#[shard]
type One2Nine = x! { {'1'..'9'} };

#[doc(hidden)]
pub struct One2Nine<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl<'i> ::xst::internal::StaticShard for One2Nine<'i> {}

    impl<'i> ::xst::internal::Shard for One2Nine<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = &'i ::xst::internal::str;
        type Data = ::xst::internal::Set<false, __xst_shard_set_0>;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_set_0;

    impl ::xst::internal::ShardSet for __xst_shard_set_0 {
        const SET: &'static [::xst::internal::RangeInclusive<::xst::internal::char>] = &['1'..='9'];
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

#[derive(Debug)]
pub enum JSONBoolean<'i> {
    True(::xst::internal::FieldOutput<'i, JSONBoolean<'static>, 0>),
    False(::xst::internal::FieldOutput<'i, JSONBoolean<'static>, 1>),
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSONBoolean<'i> {}

    impl<'i> ::xst::internal::Shard for JSONBoolean<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONBoolean<'i>;
        type Data = ::xst::internal::Alt<(
            ::xst::internal::Ext<JSONTrue<'static>>,
            ::xst::internal::Ext<JSONFalse<'static>>,
        )>;
    }

    impl ::xst::internal::ShardField<0> for JSONBoolean<'static> {
        type Output<'i> = ::xst::internal::Output<'i, JSONTrue<'static>>;
    }

    impl ::xst::internal::ShardField<1> for JSONBoolean<'static> {
        type Output<'i> = ::xst::internal::Output<'i, JSONFalse<'static>>;
    }
};

// original reference: crate::cluster::boolean::JSONTrue
#[shard]
pub struct JSONTrue {
    text: x! { "true" },
}

#[derive(Debug)]
pub struct JSONTrue<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    text: ::xst::internal::FieldOutput<'i, JSONTrue<'static>, 0>,
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSONTrue<'i> {}

    impl<'i> ::xst::internal::Shard for JSONTrue<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONTrue<'i>;
        type Data = ::xst::internal::Lit<__xst_shard_literal_0>;
    }

    impl ::xst::internal::ShardField<0> for JSONTrue<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_0;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_0 {
        const LITERAL: &'static ::xst::internal::str = "true";
    }
};

// original reference: crate::cluster::boolean::JSONFalse
#[shard]
pub struct JSONFalse {
    text: x! { "false" },
}

#[derive(Debug)]
pub struct JSONFalse<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    text: ::xst::internal::FieldOutput<'i, JSONFalse<'static>, 0>,
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSONFalse<'i> {}

    impl<'i> ::xst::internal::Shard for JSONFalse<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONFalse<'i>;
        type Data = ::xst::internal::Lit<__xst_shard_literal_0>;
    }

    impl ::xst::internal::ShardField<0> for JSONFalse<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_0;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_0 {
        const LITERAL: &'static ::xst::internal::str = "false";
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

#[derive(Debug)]
pub struct JSONNull<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    text: ::xst::internal::FieldOutput<'i, JSONNull<'static>, 0>,
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for JSONNull<'i> {}

    impl<'i> ::xst::internal::Shard for JSONNull<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = JSONNull<'i>;
        type Data = ::xst::internal::Lit<__xst_shard_literal_0>;
    }

    impl ::xst::internal::ShardField<0> for JSONNull<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_literal_0;

    impl ::xst::internal::ShardLiteral for __xst_shard_literal_0 {
        const LITERAL: &'static ::xst::internal::str = "null";
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

#[derive(Debug)]
pub struct WS<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    space: ::xst::internal::FieldOutput<'i, WS<'static>, 0>,
}

const _: () = {
    impl<'i> ::xst::internal::StaticShard for WS<'i> {}

    impl<'i> ::xst::internal::Shard for WS<'i> {
        type Core = __xst_shard_core_0;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0(::xst::internal::PhantomData<fn() -> ()>);

    impl ::xst::internal::ShardCore for __xst_shard_core_0 {
        type Output<'i> = WS<'i>;
        type Data = ::xst::internal::Vec<
            ::xst::internal::Set<false, __xst_shard_set_0>,
            0,
            { ::core::primitive::usize::MAX },
        >;
    }

    impl ::xst::internal::ShardField<0> for WS<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_set_0;

    impl ::xst::internal::ShardSet for __xst_shard_set_0 {
        const SET: &'static [::xst::internal::RangeInclusive<::xst::internal::char>] =
            &[' '..=' ', '\t'..='\t', '\n'..='\n', '\r'..='\r'];
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
    inner: xopt![(xbox![T], xvec![(P, T); ..])],
}

pub struct Punctuated<'i, T: ::xst::internal::ShardParam, P: ::xst::internal::ShardParam> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    inner: ::xst::internal::FieldOutput<'i, Punctuated<'static, T, P>, 0>,
}

const _: () = {
    impl<'i, T: ::xst::internal::ShardParam, P: ::xst::internal::ShardParam> ::core::fmt::Debug
        for Punctuated<'i, T, P>
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("Punctuated")
                .field("inner", &self.inner)
                .finish()
        }
    }

    impl<'i, T, P> ::xst::internal::Shard for Punctuated<'i, T, P>
    where
        T: ::xst::internal::ShardParam,
        P: ::xst::internal::ShardParam,
    {
        type Core = __xst_shard_core_0<T, P>;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0<T, P>(::xst::internal::PhantomData<fn() -> (T, P)>);

    impl<T, P> ::xst::internal::ShardCore for __xst_shard_core_0<T, P>
    where
        T: ::xst::internal::ShardParam,
        P: ::xst::internal::ShardParam,
    {
        type Output<'i> = Punctuated<'i, T, P>;
        type Data = ::xst::internal::Opt<
            ::xst::internal::Seq<(
                T,
                ::xst::internal::Vec<
                    ::xst::internal::Seq<(P, T)>,
                    0,
                    { ::core::primitive::usize::MAX },
                >,
            )>,
        >;
    }

    impl<T, P> ::xst::internal::ShardField<0> for Punctuated<'static, T, P>
    where
        T: ::xst::internal::ShardParam,
        P: ::xst::internal::ShardParam,
    {
        type Output<'i> = Option<(
            Box<::xst::internal::ParamOutput<'i, T>>,
            Vec<(
                ::xst::internal::ParamOutput<'i, P>,
                ::xst::internal::ParamOutput<'i, T>,
            )>,
        )>;
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

pub struct Spanned<'i, T: ::xst::internal::ShardParam> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    inner: ::xst::internal::FieldOutput<'i, Spanned<'static, T>, 0>,
    ws: ::xst::internal::FieldOutput<'i, Spanned<'static, T>, 1>,
}

const _: () = {
    impl<'i, T: ::xst::internal::ShardParam> ::core::fmt::Debug for Spanned<'i, T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_struct("Spanned")
                .field("inner", &self.inner)
                .field("ws", &self.ws)
                .finish()
        }
    }

    impl<'i, T> ::xst::internal::Shard for Spanned<'i, T>
    where
        T: ::xst::internal::ShardParam,
    {
        type Core = __xst_shard_core_0<T>;
    }

    #[allow(dead_code)]
    #[allow(non_camel_case_types)]
    pub struct __xst_shard_core_0<T>(::xst::internal::PhantomData<fn() -> (T,)>);

    impl<T> ::xst::internal::ShardCore for __xst_shard_core_0<T>
    where
        T: ::xst::internal::ShardParam,
    {
        type Output<'i> = Spanned<'i, T>;
        type Data = ::xst::internal::Seq<(T, ::xst::internal::Ext<WS<'static>>)>;
    }

    impl<T> ::xst::internal::ShardField<0> for Spanned<'static, T>
    where
        T: ::xst::internal::ShardParam,
    {
        type Output<'i> = ::xst::internal::ParamOutput<'i, T>;
    }

    impl<T> ::xst::internal::ShardField<1> for Spanned<'static, T>
    where
        T: ::xst::internal::ShardParam,
    {
        type Output<'i> = ::xst::internal::Output<'i, WS<'static>>;
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
        for input in [
            "null",
            " true ",
            "-12.5e+2",
            "[]",
            "{}",
            r#"{"a":[1,false,null,"\u0041"]}"#,
        ] {
            assert_eq!(cluster.parse(input), Ok(()), "{input:?}");
        }
        for input in ["", "01", "1.", "[1,]", r#""\u00""#, "true false"] {
            assert!(cluster.parse(input).is_err(), "{input:?}");
        }
    }

    #[test]
    fn field_outputs_cover_borrowing_grouping_and_variants() {
        use super::*;
        let input = String::from("hello");
        let string = JSONString {
            __xst_marker_0: ::core::marker::PhantomData,
            quote: ("\"", "\""),
            content: &input,
        };
        assert_eq!(string.content.as_ptr(), input.as_ptr());
        assert_eq!(string.quote, ("\"", "\""));
        let value = JSONValue::String(string);
        let JSONValue::String(string) = value else {
            panic!("wrong variant")
        };
        assert_eq!(string.content, "hello");

        let number = JSONNumber {
            __xst_marker_0: ::core::marker::PhantomData,
            sign: Some("-"),
            integer: "1",
            fraction: None,
            exponent: None,
        };
        assert_eq!(number.sign, Some("-"));
        assert_eq!(number.integer, "1");
    }

    #[test]
    fn generic_fields_keep_helpers_local() {
        use super::*;
        // These names belong to the test's scope, not either expansion.
        #[allow(non_camel_case_types)]
        struct __xst_shard_literal_0;
        #[allow(non_camel_case_types)]
        struct __xst_shard_literal_1;
        let _ = (__xst_shard_literal_0, __xst_shard_literal_1);
        let object = JSONObject {
            __xst_marker_0: ::core::marker::PhantomData,
            brace: ("{", "}"),
            ws: WS {
                __xst_marker_0: ::core::marker::PhantomData,
                space: "",
            },
            content: Punctuated {
                __xst_marker_0: ::core::marker::PhantomData,
                inner: None,
            },
        };
        let array = JSONArray {
            __xst_marker_0: ::core::marker::PhantomData,
            bracket: ("[", "]"),
            ws: WS {
                __xst_marker_0: ::core::marker::PhantomData,
                space: "",
            },
            entries: Punctuated {
                __xst_marker_0: ::core::marker::PhantomData,
                inner: None,
            },
        };
        assert!(object.content.inner.is_none());
        assert!(array.entries.inner.is_none());
        // Debug requires only output fields, not local grammar descriptors.
        assert!(format!("{object:?}").contains("JSONObject"));
        assert!(format!("{array:?}").contains("JSONArray"));
    }
}
