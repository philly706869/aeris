//! #[shard] macro code generation example

use xst::shard;

macro_rules! shard_impl {
    ($name:ident, $output:ty, $data:expr) => {
        impl<'i> ::xst::internal::Shard for $name<'i> {
            type Core = $name<'static>;
        }
        impl ::xst::internal::ShardCore for $name<'static> {
            type Output<'i> = $output;
            const DATA: &'static ::xst::internal::ShardData = &$data;
        }
    };
}

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
        ::xst::internal::ShardField<'i, WS<'static>>,
        ::xst::internal::ShardField<'i, WS<'static>>,
    ),
    value: ::xst::internal::ShardField<'i, JSONValue<'static>>,
}
impl<'i> ::xst::internal::StaticShard for JSON<'i> {}
shard_impl!(
    JSON,
    JSON<'i>,
    ::xst::internal::ShardData::sequence(&[
        &::xst::internal::ShardData::reference::<WS<'static>>(),
        &::xst::internal::ShardData::reference::<JSONValue<'static>>(),
        &::xst::internal::ShardData::reference::<WS<'static>>(),
    ])
);

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
    Object(::xst::internal::ShardField<'i, JSONObject<'static>>),
    Array(::xst::internal::ShardField<'i, JSONArray<'static>>),
    String(::xst::internal::ShardField<'i, JSONString<'static>>),
    Number(::xst::internal::ShardField<'i, JSONNumber<'static>>),
    Boolean(::xst::internal::ShardField<'i, JSONBoolean<'static>>),
    Null(::xst::internal::ShardField<'i, JSONNull<'static>>),
}
impl<'i> ::xst::internal::StaticShard for JSONValue<'i> {}
shard_impl!(
    JSONValue,
    JSONValue<'i>,
    ::xst::internal::ShardData::alternative(&[
        &::xst::internal::ShardData::reference::<JSONObject<'static>>(),
        &::xst::internal::ShardData::reference::<JSONArray<'static>>(),
        &::xst::internal::ShardData::reference::<JSONString<'static>>(),
        &::xst::internal::ShardData::reference::<JSONNumber<'static>>(),
        &::xst::internal::ShardData::reference::<JSONBoolean<'static>>(),
        &::xst::internal::ShardData::reference::<JSONNull<'static>>(),
    ])
);

#[derive(Debug)]
struct Comma<'i>(::xst::internal::PhantomData<&'i ()>);
shard_impl!(Comma, &'i str, ::xst::internal::ShardData::literal(","));

#[shard]
pub struct JSONObject {
    brace: x! {"{"},
    ws: WS,
    content: Punctuated<Spanned<JSONObjectEntry>, Spanned<Comma>>,
    brace: x! {"}"},
}
#[derive(Debug)]
pub struct JSONObject<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    brace: (&'i str, &'i str),
    ws: ::xst::internal::ShardField<'i, WS<'static>>,
    content: ::xst::internal::ShardField<
        'i,
        Punctuated<
            'static,
            Spanned<'static, JSONObjectEntry<'static>>,
            Spanned<'static, Comma<'static>>,
        >,
    >,
}
impl<'i> ::xst::internal::StaticShard for JSONObject<'i> {}
shard_impl!(
    JSONObject,
    JSONObject<'i>,
    ::xst::internal::ShardData::sequence(&[
        &::xst::internal::ShardData::literal("{"),
        &::xst::internal::ShardData::reference::<WS<'static>>(),
        &::xst::internal::ShardData::reference::<
            Punctuated<
                'static,
                Spanned<'static, JSONObjectEntry<'static>>,
                Spanned<'static, Comma<'static>>,
            >,
        >(),
        &::xst::internal::ShardData::literal("}"),
    ])
);

#[shard]
pub struct JSONObjectEntry {
    name: JSONString,
    ws: WS,
    colon: x! {":"},
    ws: WS,
    value: JSONValue,
}
#[derive(Debug)]
pub struct JSONObjectEntry<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    name: ::xst::internal::ShardField<'i, JSONString<'static>>,
    ws: (
        ::xst::internal::ShardField<'i, WS<'static>>,
        ::xst::internal::ShardField<'i, WS<'static>>,
    ),
    colon: &'i str,
    value: ::xst::internal::ShardField<'i, JSONValue<'static>>,
}
impl<'i> ::xst::internal::StaticShard for JSONObjectEntry<'i> {}
shard_impl!(
    JSONObjectEntry,
    JSONObjectEntry<'i>,
    ::xst::internal::ShardData::sequence(&[
        &::xst::internal::ShardData::reference::<JSONString<'static>>(),
        &::xst::internal::ShardData::reference::<WS<'static>>(),
        &::xst::internal::ShardData::literal(":"),
        &::xst::internal::ShardData::reference::<WS<'static>>(),
        &::xst::internal::ShardData::reference::<JSONValue<'static>>(),
    ])
);

#[shard]
pub struct JSONArray {
    bracket: x! {"["},
    ws: WS,
    entries: Punctuated<Spanned<JSONValue>, Spanned<Comma>>,
    bracket: x! {"]"},
}
#[derive(Debug)]
pub struct JSONArray<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    bracket: (&'i str, &'i str),
    ws: ::xst::internal::ShardField<'i, WS<'static>>,
    entries: ::xst::internal::ShardField<
        'i,
        Punctuated<'static, Spanned<'static, JSONValue<'static>>, Spanned<'static, Comma<'static>>>,
    >,
}
impl<'i> ::xst::internal::StaticShard for JSONArray<'i> {}
shard_impl!(
    JSONArray,
    JSONArray<'i>,
    ::xst::internal::ShardData::sequence(&[
        &::xst::internal::ShardData::literal("["),
        &::xst::internal::ShardData::reference::<WS<'static>>(),
        &::xst::internal::ShardData::reference::<
            Punctuated<
                'static,
                Spanned<'static, JSONValue<'static>>,
                Spanned<'static, Comma<'static>>,
            >,
        >(),
        &::xst::internal::ShardData::literal("]"),
    ])
);

#[shard]
pub struct JSONString {
    quote: x! {"\""},
    content: Content,
    quote: x! {"\""},
}
#[derive(Debug)]
pub struct JSONString<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    quote: (&'i str, &'i str),
    content: ::xst::internal::ShardField<'i, Content<'static>>,
}
impl<'i> ::xst::internal::StaticShard for JSONString<'i> {}
shard_impl!(
    JSONString,
    JSONString<'i>,
    ::xst::internal::ShardData::sequence(&[
        &::xst::internal::ShardData::literal("\""),
        &::xst::internal::ShardData::reference::<Content<'static>>(),
        &::xst::internal::ShardData::literal("\""),
    ])
);

#[derive(Debug)]
struct Content<'i>(::xst::internal::PhantomData<&'i ()>);
shard_impl!(
    Content,
    &'i str,
    ::xst::internal::ShardData::vec(
        &::xst::internal::ShardData::alternative(&[
            &::xst::internal::ShardData::set(
                true,
                &['"'..='"', '\\'..='\\', '\u{0000}'..='\u{001F}']
            ),
            &::xst::internal::ShardData::sequence(&[
                &::xst::internal::ShardData::literal("\\"),
                &::xst::internal::ShardData::reference::<Escape<'static>>()
            ]),
        ]),
        0,
        usize::MAX
    )
);
#[derive(Debug)]
struct Escape<'i>(::xst::internal::PhantomData<&'i ()>);
shard_impl!(
    Escape,
    &'i str,
    ::xst::internal::ShardData::alternative(&[
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
                't'..='t'
            ]
        ),
        &::xst::internal::ShardData::sequence(&[
            &::xst::internal::ShardData::literal("u"),
            &::xst::internal::ShardData::vec(
                &::xst::internal::ShardData::set(false, &['0'..='9', 'A'..='F', 'a'..='f']),
                4,
                4
            )
        ]),
    ])
);

#[shard]
pub struct JSONNumber {
    sign: xopt![x! {"-"}],
    integer: x! {[| Digit | One2Nine Digits]},
    fraction: xopt![JSONFraction],
    exponent: xopt![JSONExponent],
}
#[derive(Debug)]
pub struct JSONNumber<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    sign: Option<&'i str>,
    integer: &'i str,
    fraction: Option<::xst::internal::ShardField<'i, JSONFraction<'static>>>,
    exponent: Option<::xst::internal::ShardField<'i, JSONExponent<'static>>>,
}
impl<'i> ::xst::internal::StaticShard for JSONNumber<'i> {}
shard_impl!(
    JSONNumber,
    JSONNumber<'i>,
    ::xst::internal::ShardData::sequence(&[
        &::xst::internal::ShardData::option(&::xst::internal::ShardData::literal("-")),
        &::xst::internal::ShardData::alternative(&[
            &::xst::internal::ShardData::reference::<Digit<'static>>(),
            &::xst::internal::ShardData::sequence(&[
                &::xst::internal::ShardData::reference::<One2Nine<'static>>(),
                &::xst::internal::ShardData::reference::<Digits<'static>>()
            ])
        ]),
        &::xst::internal::ShardData::option(&::xst::internal::ShardData::reference::<
            JSONFraction<'static>,
        >()),
        &::xst::internal::ShardData::option(&::xst::internal::ShardData::reference::<
            JSONExponent<'static>,
        >()),
    ])
);

#[derive(Debug)]
pub struct JSONFraction<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    point: &'i str,
    digits: ::xst::internal::ShardField<'i, Digits<'static>>,
}
impl<'i> ::xst::internal::StaticShard for JSONFraction<'i> {}
shard_impl!(
    JSONFraction,
    JSONFraction<'i>,
    ::xst::internal::ShardData::sequence(&[
        &::xst::internal::ShardData::literal("."),
        &::xst::internal::ShardData::reference::<Digits<'static>>()
    ])
);
#[derive(Debug)]
pub struct JSONExponent<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    e: &'i str,
    sign: Option<&'i str>,
    digits: ::xst::internal::ShardField<'i, Digits<'static>>,
}
impl<'i> ::xst::internal::StaticShard for JSONExponent<'i> {}
shard_impl!(
    JSONExponent,
    JSONExponent<'i>,
    ::xst::internal::ShardData::sequence(&[
        &::xst::internal::ShardData::set(false, &['E'..='E', 'e'..='e']),
        &::xst::internal::ShardData::option(&::xst::internal::ShardData::set(
            false,
            &['+'..='+', '-'..='-']
        )),
        &::xst::internal::ShardData::reference::<Digits<'static>>(),
    ])
);
#[derive(Debug)]
struct Digits<'i>(::xst::internal::PhantomData<&'i ()>);
shard_impl!(
    Digits,
    &'i str,
    ::xst::internal::ShardData::vec(
        &::xst::internal::ShardData::reference::<Digit<'static>>(),
        1,
        usize::MAX
    )
);
#[derive(Debug)]
struct Digit<'i>(::xst::internal::PhantomData<&'i ()>);
shard_impl!(
    Digit,
    &'i str,
    ::xst::internal::ShardData::alternative(&[
        &::xst::internal::ShardData::literal("0"),
        &::xst::internal::ShardData::reference::<One2Nine<'static>>()
    ])
);
#[derive(Debug)]
struct One2Nine<'i>(::xst::internal::PhantomData<&'i ()>);
shard_impl!(
    One2Nine,
    &'i str,
    ::xst::internal::ShardData::set(false, &['1'..='9'])
);

#[shard]
pub enum JSONBoolean {
    True(JSONTrue),
    False(JSONFalse),
}
#[derive(Debug)]
pub enum JSONBoolean<'i> {
    True(::xst::internal::ShardField<'i, JSONTrue<'static>>),
    False(::xst::internal::ShardField<'i, JSONFalse<'static>>),
}
impl<'i> ::xst::internal::StaticShard for JSONBoolean<'i> {}
shard_impl!(
    JSONBoolean,
    JSONBoolean<'i>,
    ::xst::internal::ShardData::alternative(&[
        &::xst::internal::ShardData::reference::<JSONTrue<'static>>(),
        &::xst::internal::ShardData::reference::<JSONFalse<'static>>()
    ])
);

macro_rules! keyword_shard {
    ($name:ident, $text:literal) => {
        #[derive(Debug)]
        pub struct $name<'i> {
            __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
            text: &'i str,
        }
        impl<'i> ::xst::internal::StaticShard for $name<'i> {}
        shard_impl!($name, $name<'i>, ::xst::internal::ShardData::literal($text));
    };
}
keyword_shard!(JSONTrue, "true");
keyword_shard!(JSONFalse, "false");
keyword_shard!(JSONNull, "null");

#[derive(Debug)]
pub struct WS<'i> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    space: &'i str,
}
impl<'i> ::xst::internal::StaticShard for WS<'i> {}
shard_impl!(
    WS,
    WS<'i>,
    ::xst::internal::ShardData::vec(
        &::xst::internal::ShardData::set(
            false,
            &[' '..=' ', '\t'..='\t', '\n'..='\n', '\r'..='\r']
        ),
        0,
        usize::MAX
    )
);

pub struct Punctuated<'i, T: ::xst::internal::Shard, P: ::xst::internal::Shard> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    inner: Option<(
        Box<::xst::internal::ShardField<'i, T>>,
        Vec<(
            ::xst::internal::ShardField<'i, P>,
            ::xst::internal::ShardField<'i, T>,
        )>,
    )>,
}
impl<'i, T: ::xst::internal::Shard, P: ::xst::internal::Shard> ::core::fmt::Debug
    for Punctuated<'i, T, P>
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Punctuated")
            .field("inner", &self.inner)
            .finish()
    }
}
impl<'i, T: ::xst::internal::Shard + 'static, P: ::xst::internal::Shard + 'static>
    ::xst::internal::Shard for Punctuated<'i, T, P>
where
    T::Core: 'static,
    P::Core: 'static,
{
    type Core = Punctuated<'static, T, P>;
}
impl<T: ::xst::internal::Shard + 'static, P: ::xst::internal::Shard + 'static>
    ::xst::internal::ShardCore for Punctuated<'static, T, P>
where
    T::Core: 'static,
    P::Core: 'static,
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
                usize::MAX,
            ),
        ]));
}

pub struct Spanned<'i, T: ::xst::internal::Shard> {
    __xst_marker_0: ::xst::internal::PhantomData<&'i ()>,
    inner: ::xst::internal::ShardField<'i, T>,
    ws: ::xst::internal::ShardField<'i, WS<'static>>,
}
impl<'i, T: ::xst::internal::Shard> ::core::fmt::Debug for Spanned<'i, T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Spanned")
            .field("inner", &self.inner)
            .field("ws", &self.ws)
            .finish()
    }
}
impl<'i, T: ::xst::internal::Shard + 'static> ::xst::internal::Shard for Spanned<'i, T>
where
    T::Core: 'static,
{
    type Core = Spanned<'static, T>;
}
impl<T: ::xst::internal::Shard + 'static> ::xst::internal::ShardCore for Spanned<'static, T>
where
    T::Core: 'static,
{
    type Output<'i> = Spanned<'i, T>;
    const DATA: &'static ::xst::internal::ShardData = &::xst::internal::ShardData::sequence(&[
        &::xst::internal::ShardData::reference::<T>(),
        &::xst::internal::ShardData::reference::<WS<'static>>(),
    ]);
}

#[cfg(test)]
mod test {
    use super::JSON;
    use xst::Cluster;
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
}
