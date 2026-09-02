// ////////////////
// JSON
// ////////////////

#[cfg(false)]
#[shard]
pub struct JSON {
    ws: WS,
    value: JSONValue,
    ws: WS,
}

#[derive(Debug)]
pub struct JSON<'i> {
    _i: ::std::marker::PhantomData<&'i ()>,
    ws: (WS<'i>, WS<'i>),
    value: JSONValue<'i>,
}

const _: () = {
    impl<'i> ::aeris::ui::internal::Shard for JSON<'i> {
        type Static = Static;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static {}

    impl ::aeris::ui::internal::ShardStatic for Static {
        type Data = ::aeris::ui::internal::Alternative<(
            ::aeris::ui::internal::Extern<
                <self::WS<'static> as ::aeris::ui::internal::Shard>::Static,
            >,
            ::aeris::ui::internal::Extern<
                <self::JSONValue<'static> as ::aeris::ui::internal::Shard>::Static,
            >,
            ::aeris::ui::internal::Extern<
                <self::WS<'static> as ::aeris::ui::internal::Shard>::Static,
            >,
        )>;
    }

    impl<'i> ::aeris::ui::internal::StaticShard for JSON<'i> {}
};

// ////////////////
// JSON
// ////////////////

// ////////////////
// JSONValue
// ////////////////

#[cfg(false)]
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
    Array(JSONArray<'i>),
    Null(JSONNull<'i>),
}

const _: () = {
    impl<'i> ::aeris::ui::internal::Shard for JSONValue<'i> {
        type Static = Static;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static {}

    impl ::aeris::ui::internal::ShardStatic for Static {
        type Data = ::aeris::ui::internal::Alternative<(
            ::aeris::ui::internal::Extern<
                <self::JSONArray<'static> as ::aeris::ui::internal::Shard>::Static,
            >,
            ::aeris::ui::internal::Extern<
                <self::JSONNull<'static> as ::aeris::ui::internal::Shard>::Static,
            >,
        )>;
    }

    impl<'i> ::aeris::ui::internal::StaticShard for JSONValue<'i> {}
};

// ////////////////
// JSONValue
// ////////////////

// ////////////////
// JSONArray
// ////////////////

#[cfg(false)]
#[shard]
pub struct JSONArray {
    bracket: x!["["],
    ws: WS,
    entries: Punctuated<Spanned<JSONValue!(box)>, Spanned<x![","]>>,
    bracket: x!["["],
}

#[derive(Debug)]
pub struct JSONArray<'i> {
    _i: ::std::marker::PhantomData<&'i ()>,
    bracket: (
        &'i ::aeris::ui::internal::Str,
        &'i ::aeris::ui::internal::Str,
    ),
    ws: WS<'i>,
    entries: Punctuated<
        'i,
        Spanned<'i, Box<JSONValue<'i>>>,
        Spanned<'i, &'i ::aeris::ui::internal::Str>,
    >,
}

const _: () = {
    impl<'i> ::aeris::ui::internal::Shard for JSONArray<'i> {
        type Static = Static;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static {}

    impl ::aeris::ui::internal::ShardStatic for Static {
        type Data = ::aeris::ui::internal::Sequence<(
            ::aeris::ui::internal::Literal<Literal0>,
            ::aeris::ui::internal::Extern<
                <self::WS<'static> as ::aeris::ui::internal::Shard>::Static,
            >,
            ::aeris::ui::internal::Extern<
                <self::Punctuated<
                    'static,
                    self::Spanned<'static, self::JSONValue<'static>>,
                    self::Spanned<'static, ::aeris::ui::internal::Literal<Literal1>>,
                > as ::aeris::ui::internal::Shard>::Static,
            >,
            ::aeris::ui::internal::Literal<Literal2>,
        )>;
    }

    #[allow(dead_code)]
    struct Literal0;

    impl ::aeris::ui::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::aeris::ui::internal::Str = "[";
    }

    #[allow(dead_code)]
    struct Literal1;

    impl ::aeris::ui::internal::ShardLiteral for Literal1 {
        const LITERAL: &'static ::aeris::ui::internal::Str = ",";
    }

    #[allow(dead_code)]
    struct Literal2;

    impl ::aeris::ui::internal::ShardLiteral for Literal2 {
        const LITERAL: &'static ::aeris::ui::internal::Str = "]";
    }

    impl<'i> ::aeris::ui::internal::StaticShard for JSONArray<'i> {}
};

// ////////////////
// JSONArray
// ////////////////

// ////////////////
// JSONNull
// ////////////////

#[cfg(false)]
#[shard]
pub struct JSONNull {
    text: x!["null"],
}

#[derive(Debug)]
pub struct JSONNull<'i> {
    _i: ::std::marker::PhantomData<&'i ()>,
    text: &'i ::aeris::ui::internal::Str,
}

const _: () = {
    impl<'i> ::aeris::ui::internal::Shard for JSONNull<'i> {
        type Static = Static;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static {}

    impl ::aeris::ui::internal::ShardStatic for Static {
        type Data = ::aeris::ui::internal::Literal<Literal0>;
    }

    struct Literal0;

    impl ::aeris::ui::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::aeris::ui::internal::Str = "null";
    }

    impl<'i> ::aeris::ui::internal::StaticShard for JSONNull<'i> {}
};

// ////////////////
// JSONNull
// ////////////////

// ////////////////
// WS
// ////////////////

#[cfg(false)]
#[shard]
pub struct WS {
    space: x![{' ' '\t' '\n' '\r'}*],
}

#[derive(Debug)]
pub struct WS<'i> {
    _i: ::std::marker::PhantomData<&'i ()>,
    space: &'i ::aeris::ui::internal::Str,
}

const _: () = {
    impl<'i> ::aeris::ui::internal::Shard for WS<'i> {
        type Static = Static;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static {}

    impl ::aeris::ui::internal::ShardStatic for Static {
        type Data = ::aeris::ui::internal::Vec<::aeris::ui::internal::Set<Set0, false>, 0, 0>;
    }

    struct Set0;

    impl ::aeris::ui::internal::ShardSet for Set0 {
        const SET: &'static [::std::ops::RangeInclusive<char>] =
            &[' '..=' ', '\t'..='\t', '\n'..='\n', '\r'..='\r'];
    }

    impl<'i> ::aeris::ui::internal::StaticShard for WS<'i> {}
};

// ////////////////
// WS
// ////////////////

// ////////////////
// Punctuated
// ////////////////

#[cfg(false)]
#[shard]
pub struct Punctuated<T, P> {
    inner: Option<(Box<T>, Vec<(P, T)>)>,
}

#[derive(Debug)]
pub struct Punctuated<'i, T, P> {
    _i: ::std::marker::PhantomData<&'i ()>,
    inner: Option<(Box<T>, Vec<(P, T)>)>,
}

const _: () = {
    impl<'i, T, P> ::aeris::ui::internal::Shard for Punctuated<'i, T, P>
    where
        T: ::aeris::ui::internal::Shard + 'static,
        P: ::aeris::ui::internal::Shard + 'static,
    {
        type Static = Static<
            <T as ::aeris::ui::internal::Shard>::Static,
            <P as ::aeris::ui::internal::Shard>::Static,
        >;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static<T, P> {
        T: ::std::marker::PhantomData<T>,
        P: ::std::marker::PhantomData<P>,
    }

    impl<T, P> ::aeris::ui::internal::ShardStatic for Static<T, P>
    where
        T: ::aeris::ui::internal::ShardStatic,
        P: ::aeris::ui::internal::ShardStatic,
    {
        type Data = ::aeris::ui::internal::Option<
            ::aeris::ui::internal::Sequence<(
                ::aeris::ui::internal::Extern<T>,
                ::aeris::ui::internal::Vec<
                    ::aeris::ui::internal::Sequence<(
                        ::aeris::ui::internal::Extern<P>,
                        ::aeris::ui::internal::Extern<T>,
                    )>,
                    0,
                    0,
                >,
            )>,
        >;
    }
};

// ////////////////
// Punctuated
// ////////////////

// ////////////////
// Spanned
// ////////////////

#[cfg(false)]
#[shard]
pub struct Spanned {
    inner: T,
    ws: WS,
}

#[derive(Debug)]
pub struct Spanned<'i, T> {
    _i: ::std::marker::PhantomData<&'i ()>,
    inner: T,
    ws: WS<'i>,
}

const _: () = {
    impl<'i, T> ::aeris::ui::internal::Shard for Spanned<'i, T>
    where
        T: ::aeris::ui::internal::Shard + 'static,
    {
        type Static = Static<<T as ::aeris::ui::internal::Shard>::Static>;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static<T> {
        T: ::std::marker::PhantomData<T>,
    }

    impl<T> ::aeris::ui::internal::ShardStatic for Static<T>
    where
        T: ::aeris::ui::internal::ShardStatic,
    {
        type Data = ::aeris::ui::internal::Sequence<(
            ::aeris::ui::internal::Extern<T>,
            ::aeris::ui::internal::Extern<
                <self::WS<'static> as ::aeris::ui::internal::Shard>::Static,
            >,
        )>;
    }
};

// ////////////////
// Spanned
// ////////////////
