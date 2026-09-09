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
    ws: (
        ::xst::internal::Output<'i, WS<'static>>,
        ::xst::internal::Output<'i, WS<'static>>,
    ),
    value: ::xst::internal::Output<'i, JSONValue<'static>>,
}

const _: () = {
    impl ::xst::internal::Shard for JSON<'static> {
        type Output<'i> = JSON<'i>;
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
    Object(::xst::internal::Output<'i, JSONObject<'static>>),
    Array(::xst::internal::Output<'i, JSONArray<'static>>),
    String(::xst::internal::Output<'i, JSONString<'static>>),
    Number(::xst::internal::Output<'i, JSONNumber<'static>>),
    Boolean(::xst::internal::Output<'i, JSONBoolean<'static>>),
    Null(::xst::internal::Output<'i, JSONNull<'static>>),
}

const _: () = {
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

#[allow(dead_code)]
pub struct ObjectComma;

impl ::xst::internal::ShardLiteral for ObjectComma {
    const LITERAL: &'static ::xst::internal::str = ",";
}

#[derive(Debug)]
pub struct JSONObject<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    brace: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    ws: ::xst::internal::Output<'i, WS<'static>>,
    content: ::xst::internal::Output<
        'i,
        Punctuated<
            'static,
            ::xst::internal::Ext<Spanned<'static, ::xst::internal::Ext<JSONObjectEntry<'static>>>>,
            ::xst::internal::Ext<Spanned<'static, ::xst::internal::Lit<ObjectComma>>>,
        >,
    >,
}

const _: () = {
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
                    ::xst::internal::Ext<Spanned<'static, ::xst::internal::Lit<ObjectComma>>>,
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
    name: ::xst::internal::Output<'i, JSONString<'static>>,
    ws: (
        ::xst::internal::Output<'i, WS<'static>>,
        ::xst::internal::Output<'i, WS<'static>>,
    ),
    colon: &'i ::xst::internal::str,
    value: ::xst::internal::Output<'i, JSONValue<'static>>,
}

const _: () = {
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

#[allow(dead_code)]
pub struct ArrayComma;

impl ::xst::internal::ShardLiteral for ArrayComma {
    const LITERAL: &'static ::xst::internal::str = ",";
}

#[derive(Debug)]
pub struct JSONArray<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    bracket: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    ws: ::xst::internal::Output<'i, WS<'static>>,
    entries: ::xst::internal::Output<
        'i,
        Punctuated<
            'static,
            ::xst::internal::Ext<Spanned<'static, ::xst::internal::Ext<JSONValue<'static>>>>,
            ::xst::internal::Ext<Spanned<'static, ::xst::internal::Lit<ArrayComma>>>,
        >,
    >,
}

const _: () = {
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
                    ::xst::internal::Ext<Spanned<'static, ::xst::internal::Lit<ArrayComma>>>,
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
    content: ::xst::internal::Output<'i, Content<'static>>,
}

const _: () = {
    impl ::xst::internal::Shard for JSONString<'static> {
        type Output<'i> = JSONString<'i>;

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

#[doc(hidden)]
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

#[doc(hidden)]
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
    _i: ::xst::internal::PhantomData<&'i ()>,
    sign: Option<&'i ::xst::internal::str>,
    integer: &'i ::xst::internal::str,
    fraction: Option<::xst::internal::Output<'i, JSONFraction<'static>>>,
    exponent: Option<::xst::internal::Output<'i, JSONExponent<'static>>>,
}

const _: () = {
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

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::str = "-";
    }

    impl ::xst::internal::StaticShard for JSONNumber<'static> {}
};

// original reference: crate::cluster::number::JSONFraction
#[shard]
pub struct JSONFraction {
    point: x! { "." },
    digits: Digits,
}

#[derive(Debug)]
pub struct JSONFraction<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    point: &'i ::xst::internal::str,
    digits: ::xst::internal::Output<'i, Digits<'static>>,
}

const _: () = {
    impl ::xst::internal::Shard for JSONFraction<'static> {
        type Output<'i> = JSONFraction<'i>;

        type Data = ::xst::internal::Seq<(
            ::xst::internal::Lit<Literal0>,
            ::xst::internal::Ext<Digits<'static>>,
        )>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::str = ".";
    }

    impl ::xst::internal::StaticShard for JSONFraction<'static> {}
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
    _i: ::xst::internal::PhantomData<&'i ()>,
    e: &'i ::xst::internal::str,
    sign: Option<&'i ::xst::internal::str>,
    digits: ::xst::internal::Output<'i, Digits<'static>>,
}

const _: () = {
    impl ::xst::internal::Shard for JSONExponent<'static> {
        type Output<'i> = JSONExponent<'i>;

        type Data = ::xst::internal::Seq<(
            ::xst::internal::Set<false, Set0>,
            ::xst::internal::Opt<::xst::internal::Set<false, Set1>>,
            ::xst::internal::Ext<Digits<'static>>,
        )>;
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

    impl ::xst::internal::StaticShard for JSONExponent<'static> {}
};

// original reference: crate::cluster::number::Digits
#[shard]
type Digits = x! { Digit+ };

#[derive(Debug)]
#[doc(hidden)]
pub struct Digits<'i>(::xst::internal::PhantomData<&'i ()>);

const _: () = {
    impl ::xst::internal::Shard for Digits<'static> {
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

#[derive(Debug)]
#[doc(hidden)]
pub struct Digit<'i>(::xst::internal::PhantomData<&'i ()>);

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

#[derive(Debug)]
#[doc(hidden)]
pub struct One2Nine<'i>(::xst::internal::PhantomData<&'i ()>);

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
    True(::xst::internal::Output<'i, JSONTrue<'static>>),
    False(::xst::internal::Output<'i, JSONFalse<'static>>),
}

const _: () = {
    impl ::xst::internal::Shard for JSONBoolean<'static> {
        type Output<'i> = JSONBoolean<'i>;

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
        type Output<'i> = JSONTrue<'i>;

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
        type Output<'i> = JSONFalse<'i>;

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
        type Output<'i> = JSONNull<'i>;

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
        type Output<'i> = WS<'i>;

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
        type Output<'i> = Punctuated<
            'i,
            ::xst::internal::ParamOutput<'i, T>,
            ::xst::internal::ParamOutput<'i, P>,
        >;

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
    ws: ::xst::internal::Output<'i, WS<'static>>,
}

const _: () = {
    impl<T> ::xst::internal::Shard for Spanned<'static, T>
    where
        T: ::xst::internal::ShardParam,
    {
        type Output<'i> = Spanned<'i, ::xst::internal::ParamOutput<'i, T>>;

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
