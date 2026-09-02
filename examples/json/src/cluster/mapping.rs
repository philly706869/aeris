#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use aeris::ui::Cluster;

    use super::JSON;

    static CLUSTER: LazyLock<Cluster<JSON>> = LazyLock::new(|| Cluster::build());

    #[test]
    fn test() {
        CLUSTER.call();
    }
}

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
        type DATA = ::aeris::ui::internal::Alternative<(
            ::aeris::ui::internal::Extern<WS<'static>>,
            ::aeris::ui::internal::Extern<JSONValue<'static>>,
            ::aeris::ui::internal::Extern<WS<'static>>,
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
        type DATA = ::aeris::ui::internal::Alternative<(
            ::aeris::ui::internal::Extern<JSONArray<'static>>,
            ::aeris::ui::internal::Extern<JSONNull<'static>>,
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
    entries: Punctuated<Spanned<[JSONValue; boxed]>, Spanned<x![","]>>,
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
        type DATA = ::aeris::ui::internal::Sequence<(
            ::aeris::ui::internal::Literal<Literal0>,
            ::aeris::ui::internal::Extern<WS<'static>>,
            ::aeris::ui::internal::Extern<
                Punctuated<
                    'static,
                    ::aeris::ui::internal::Extern<
                        Spanned<'static, ::aeris::ui::internal::Extern<JSONValue<'static>>>,
                    >,
                    ::aeris::ui::internal::Extern<
                        Spanned<'static, ::aeris::ui::internal::Literal<Literal1>>,
                    >,
                >,
            >,
            ::aeris::ui::internal::Literal<Literal2>,
        )>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::aeris::ui::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::aeris::ui::internal::Str = "[";
    }

    #[allow(dead_code)]
    pub struct Literal1;

    impl ::aeris::ui::internal::ShardLiteral for Literal1 {
        const LITERAL: &'static ::aeris::ui::internal::Str = ",";
    }

    #[allow(dead_code)]
    pub struct Literal2;

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
        type DATA = ::aeris::ui::internal::Literal<Literal0>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

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
        type DATA = ::aeris::ui::internal::Vec<::aeris::ui::internal::Set<Set0, false>, 0, 0>;
    }

    #[allow(dead_code)]
    pub struct Set0;

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
    inner: [([T; boxed], [(P, T); ..]); option],
}

#[derive(Debug)]
pub struct Punctuated<'i, T, P> {
    _i: ::std::marker::PhantomData<&'i ()>,
    inner: Option<(Box<T>, Vec<(P, T)>)>,
}

const _: () = {
    impl<'i, T, P> ::aeris::ui::internal::Shard for Punctuated<'i, T, P>
    where
        T: ::aeris::ui::internal::ShardParam,
        P: ::aeris::ui::internal::ShardParam,
    {
        type DATA = ::aeris::ui::internal::Option<
            ::aeris::ui::internal::Sequence<(
                T,
                ::aeris::ui::internal::Vec<::aeris::ui::internal::Sequence<(P, T)>, 0, 0>,
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
        T: ::aeris::ui::internal::ShardParam,
    {
        type DATA =
            ::aeris::ui::internal::Sequence<(T, ::aeris::ui::internal::Extern<WS<'static>>)>;
    }
};

// ////////////////
// Spanned
// ////////////////
