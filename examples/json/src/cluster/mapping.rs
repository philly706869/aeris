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
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
    ws: (
        ::xst::internal::FieldOutput<'i, JSON<'static>, 0>,
        ::xst::internal::FieldOutput<'i, JSON<'static>, 2>,
    ),
    value: ::xst::internal::FieldOutput<'i, JSON<'static>, 1>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSON<'static> {}

    impl ::xst::internal::Shard for JSON<'static> {
        type Output<'i> = JSON<'i>;
        type Data = ::xst::internal::Alt<(
            ::xst::internal::Ext<WS<'static>>,
            ::xst::internal::Ext<JSONValue<'static>>,
            ::xst::internal::Ext<WS<'static>>,
        )>;
    }

    impl ::xst::internal::ShardField<0> for JSON<'static> {
        type Output<'i> = ::xst::internal::Output<'i, WS<'static>>;
    }

    impl ::xst::internal::ShardField<1> for JSON<'static> {
        type Output<'i> = ::xst::internal::Output<'i, JSONValue<'static>>;
    }

    impl ::xst::internal::ShardField<2> for JSON<'static> {
        type Output<'i> = ::xst::internal::Output<'i, WS<'static>>;
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
enum JSONValue<'i> {
    Object(::xst::internal::FieldOutput<'i, JSONValue<'static>, 0>),
    Array(::xst::internal::FieldOutput<'i, JSONValue<'static>, 1>),
    String(::xst::internal::FieldOutput<'i, JSONValue<'static>, 2>),
    Number(::xst::internal::FieldOutput<'i, JSONValue<'static>, 3>),
    Boolean(::xst::internal::FieldOutput<'i, JSONValue<'static>, 4>),
    Null(::xst::internal::FieldOutput<'i, JSONValue<'static>, 5>),
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONValue<'static> {}

    impl ::xst::internal::Shard for JSONValue<'static> {
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

    impl ::xst::internal::ShardField<0> for JSONValue<'static> {
        type Output<'i> = ::xst::internal::Output<'i, JSONObject<'static>>;
    }

    impl ::xst::internal::ShardField<1> for JSONValue<'static> {
        type Output<'i> = ::xst::internal::Output<'i, JSONArray<'static>>;
    }

    impl ::xst::internal::ShardField<2> for JSONValue<'static> {
        type Output<'i> = ::xst::internal::Output<'i, JSONString<'static>>;
    }

    impl ::xst::internal::ShardField<3> for JSONValue<'static> {
        type Output<'i> = ::xst::internal::Output<'i, JSONNumber<'static>>;
    }

    impl ::xst::internal::ShardField<4> for JSONValue<'static> {
        type Output<'i> = ::xst::internal::Output<'i, JSONBoolean<'static>>;
    }

    impl ::xst::internal::ShardField<5> for JSONValue<'static> {
        type Output<'i> = ::xst::internal::Output<'i, JSONNull<'static>>;
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
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
    brace: (
        ::xst::internal::FieldOutput<'i, JSONObject<'static>, 0>,
        ::xst::internal::FieldOutput<'i, JSONObject<'static>, 3>,
    ),
    ws: ::xst::internal::FieldOutput<'i, JSONObject<'static>, 1>,
    content: ::xst::internal::FieldOutput<'i, JSONObject<'static>, 2>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONObject<'static> {}

    impl ::xst::internal::Shard for JSONObject<'static> {
        type Output<'i> = JSONObject<'i>;
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

    impl ::xst::internal::ShardField<0> for JSONObject<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    impl ::xst::internal::ShardField<1> for JSONObject<'static> {
        type Output<'i> = ::xst::internal::Output<'i, WS<'static>>;
    }

    impl ::xst::internal::ShardField<2> for JSONObject<'static> {
        type Output<'i> = ::xst::internal::Output<
            'i,
            Punctuated<
                'static,
                ::xst::internal::Ext<
                    Spanned<'static, ::xst::internal::Ext<JSONObjectEntry<'static>>>,
                >,
                ::xst::internal::Ext<Spanned<'static, ::xst::internal::Lit<Literal1>>>,
            >,
        >;
    }

    impl ::xst::internal::ShardField<3> for JSONObject<'static> {
        type Output<'i> = &'i ::xst::internal::str;
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
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
    name: ::xst::internal::FieldOutput<'i, JSONObjectEntry<'static>, 0>,
    ws: (
        ::xst::internal::FieldOutput<'i, JSONObjectEntry<'static>, 1>,
        ::xst::internal::FieldOutput<'i, JSONObjectEntry<'static>, 3>,
    ),
    colon: ::xst::internal::FieldOutput<'i, JSONObjectEntry<'static>, 2>,
    value: ::xst::internal::FieldOutput<'i, JSONObjectEntry<'static>, 4>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONObjectEntry<'static> {}

    impl ::xst::internal::Shard for JSONObjectEntry<'static> {
        type Output<'i> = JSONObjectEntry<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Ext<JSONString<'static>>,
            ::xst::internal::Ext<WS<'static>>,
            ::xst::internal::Lit<Literal0>,
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
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
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
    bracket: x! { "[" },
}

#[derive(Debug)]
pub struct JSONArray<'i> {
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
    bracket: (
        ::xst::internal::FieldOutput<'i, JSONArray<'static>, 0>,
        ::xst::internal::FieldOutput<'i, JSONArray<'static>, 3>,
    ),
    ws: ::xst::internal::FieldOutput<'i, JSONArray<'static>, 1>,
    entries: ::xst::internal::FieldOutput<'i, JSONArray<'static>, 2>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONArray<'static> {}

    impl ::xst::internal::Shard for JSONArray<'static> {
        type Output<'i> = JSONArray<'i>;
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
                ::xst::internal::Ext<Spanned<'static, ::xst::internal::Lit<Literal1>>>,
            >,
        >;
    }

    impl ::xst::internal::ShardField<3> for JSONArray<'static> {
        type Output<'i> = &'i ::xst::internal::str;
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
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
    quote: (
        ::xst::internal::FieldOutput<'i, JSONString<'static>, 0>,
        ::xst::internal::FieldOutput<'i, JSONString<'static>, 2>,
    ),
    content: ::xst::internal::FieldOutput<'i, JSONString<'static>, 1>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONString<'static> {}

    impl ::xst::internal::Shard for JSONString<'static> {
        type Output<'i> = JSONString<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Lit<Literal0>,
            ::xst::internal::Ext<Content<'static>>,
            ::xst::internal::Lit<Literal1>,
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
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::str = "\"";
    }

    #[allow(dead_code)]
    pub struct Literal1;

    impl ::xst::internal::ShardLiteral for Literal1 {
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

pub struct Content<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::Shard for Content<'static> {
        type Output<'i> = &'i ::xst::internal::str;
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

pub struct Escape<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::Shard for Escape<'static> {
        type Output<'i> = &'i ::xst::internal::str;
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

#[derive(Debug)]
pub struct JSONNumber<'i> {
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
    sign: ::xst::internal::FieldOutput<'i, JSONNumber<'static>, 0>,
    integer: ::xst::internal::FieldOutput<'i, JSONNumber<'static>, 1>,
    fraction: ::xst::internal::FieldOutput<'i, JSONNumber<'static>, 2>,
    exponent: ::xst::internal::FieldOutput<'i, JSONNumber<'static>, 3>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONNumber<'static> {}

    impl ::xst::internal::Shard for JSONNumber<'static> {
        type Output<'i> = JSONNumber<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Opt<::xst::internal::Lit<Literal0>>,
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
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
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
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
    point: ::xst::internal::FieldOutput<'i, JSONFraction<'static>, 0>,
    digits: ::xst::internal::FieldOutput<'i, JSONFraction<'static>, 1>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONFraction<'static> {}

    impl ::xst::internal::Shard for JSONFraction<'static> {
        type Output<'i> = JSONFraction<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Lit<Literal0>,
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
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
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
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
    e: ::xst::internal::FieldOutput<'i, JSONExponent<'static>, 0>,
    sign: ::xst::internal::FieldOutput<'i, JSONExponent<'static>, 1>,
    digits: ::xst::internal::FieldOutput<'i, JSONExponent<'static>, 2>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONExponent<'static> {}

    impl ::xst::internal::Shard for JSONExponent<'static> {
        type Output<'i> = JSONExponent<'i>;
        type Data = ::xst::internal::Seq<(
            ::xst::internal::Set<false, Set0>,
            ::xst::internal::Opt<::xst::internal::Set<false, Set1>>,
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
    pub struct Set0;

    impl ::xst::internal::ShardSet for Set0 {
        const SET: &'static [::xst::internal::RangeInclusive<::xst::internal::char>] =
            &['E'..='E', 'e'..='e'];
    }

    #[allow(dead_code)]
    pub struct Set1;

    impl ::xst::internal::ShardSet for Set1 {
        const SET: &'static [::xst::internal::RangeInclusive<::xst::internal::char>] =
            &['+'..='+', '-'..='-'];
    }
};

// original reference: crate::cluster::number::Digits
#[shard]
type Digits = x! { Digit+ };

struct Digits<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::Shard for Digits<'static> {
        type Output<'i> = &'i ::xst::internal::str;
        type Data = ::xst::internal::Vec<::xst::internal::Ext<Digit<'static>>, 1, 0>;
    }
};

// original reference: crate::cluster::number::Digit
#[shard]
type Digit = x! {[
    | '0'
    | One2Nine
]};

struct Digit<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::Shard for Digit<'static> {
        type Output<'i> = &'i ::xst::internal::str;
        type Data = ::xst::internal::Alt<(
            ::xst::internal::Lit<Literal0>,
            ::xst::internal::Ext<One2Nine<'static>>,
        )>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::str = "0";
    }
};

// original reference: crate::cluster::number::One2Nine
#[shard]
type One2Nine = x! { {'1'..'9'} };

struct One2Nine<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::Shard for One2Nine<'static> {
        type Output<'i> = &'i ::xst::internal::str;
        type Data = ::xst::internal::Set<false, Set0>;
    }

    #[allow(dead_code)]
    pub struct Set0;

    impl ::xst::internal::ShardSet for Set0 {
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
    impl ::xst::internal::StaticShard for JSONBoolean<'static> {}

    impl ::xst::internal::Shard for JSONBoolean<'static> {
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
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
    text: ::xst::internal::FieldOutput<'i, JSONTrue<'static>, 0>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONTrue<'static> {}

    impl ::xst::internal::Shard for JSONTrue<'static> {
        type Output<'i> = JSONTrue<'i>;
        type Data = ::xst::internal::Lit<Literal0>;
    }

    impl ::xst::internal::ShardField<0> for JSONTrue<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
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
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
    text: ::xst::internal::FieldOutput<'i, JSONFalse<'static>, 0>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONFalse<'static> {}

    impl ::xst::internal::Shard for JSONFalse<'static> {
        type Output<'i> = JSONFalse<'i>;
        type Data = ::xst::internal::Lit<Literal0>;
    }

    impl ::xst::internal::ShardField<0> for JSONFalse<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
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
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
    text: ::xst::internal::FieldOutput<'i, JSONNull<'static>, 0>,
}

const _: () = {
    impl ::xst::internal::StaticShard for JSONNull<'static> {}

    impl ::xst::internal::Shard for JSONNull<'static> {
        type Output<'i> = JSONNull<'i>;
        type Data = ::xst::internal::Lit<Literal0>;
    }

    impl ::xst::internal::ShardField<0> for JSONNull<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
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
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
    space: ::xst::internal::FieldOutput<'i, WS<'static>, 0>,
}

const _: () = {
    impl ::xst::internal::StaticShard for WS<'static> {}

    impl ::xst::internal::Shard for WS<'static> {
        type Output<'i> = WS<'i>;
        type Data = ::xst::internal::Vec<::xst::internal::Set<false, Set0>, 0, 0>;
    }

    impl ::xst::internal::ShardField<0> for WS<'static> {
        type Output<'i> = &'i ::xst::internal::str;
    }

    #[allow(dead_code)]
    pub struct Set0;

    impl ::xst::internal::ShardSet for Set0 {
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
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
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

    impl<T, P> ::xst::internal::Shard for Punctuated<'static, T, P>
    where
        T: ::xst::internal::ShardParam,
        P: ::xst::internal::ShardParam,
    {
        type Output<'i> = Punctuated<'i, T, P>;
        type Data = ::xst::internal::Opt<
            ::xst::internal::Seq<(T, ::xst::internal::Vec<::xst::internal::Seq<(P, T)>, 0, 0>)>,
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
    __xst_marker0: ::xst::internal::PhantomData<&'i ()>,
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

    impl<T> ::xst::internal::Shard for Spanned<'static, T>
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
    }

    #[test]
    fn field_outputs_cover_borrowing_grouping_and_variants() {
        use super::*;
        let input = String::from("hello");
        let string = JSONString {
            __xst_marker0: ::core::marker::PhantomData,
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
            __xst_marker0: ::core::marker::PhantomData,
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
        struct Literal0;
        struct Literal1;
        let _ = (Literal0, Literal1);
        let object = JSONObject {
            __xst_marker0: ::core::marker::PhantomData,
            brace: ("{", "}"),
            ws: WS {
                __xst_marker0: ::core::marker::PhantomData,
                space: "",
            },
            content: Punctuated {
                __xst_marker0: ::core::marker::PhantomData,
                inner: None,
            },
        };
        let array = JSONArray {
            __xst_marker0: ::core::marker::PhantomData,
            bracket: ("[", "]"),
            ws: WS {
                __xst_marker0: ::core::marker::PhantomData,
                space: "",
            },
            entries: Punctuated {
                __xst_marker0: ::core::marker::PhantomData,
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
