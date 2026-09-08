#[cfg(test)]
mod test {
    use xst::Cluster;

    use super::JSON;

    #[test]
    fn test() {
        let _: Cluster<JSON> = Cluster::build();
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
    _i: ::xst::internal::PhantomData<&'i ()>,
    ws: (WS<'i>, WS<'i>),
    value: JSONValue<'i>,
}

const _: () = {
    impl<'i> ::xst::internal::Shard for JSON<'i> {
        type Data = ::xst::internal::Alternative<(
            ::xst::internal::Extern<WS<'static>>,
            ::xst::internal::Extern<JSONValue<'static>>,
            ::xst::internal::Extern<WS<'static>>,
        )>;
    }

    impl<'i> ::xst::internal::StaticShard for JSON<'i> {}
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
    impl<'i> ::xst::internal::Shard for JSONValue<'i> {
        type Data = ::xst::internal::Alternative<(
            ::xst::internal::Extern<JSONArray<'static>>,
            ::xst::internal::Extern<JSONNull<'static>>,
        )>;
    }

    impl<'i> ::xst::internal::StaticShard for JSONValue<'i> {}
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
    _i: ::xst::internal::PhantomData<&'i ()>,
    bracket: (&'i ::xst::internal::Str, &'i ::xst::internal::Str),
    ws: WS<'i>,
    entries: Punctuated<'i, Spanned<'i, Box<JSONValue<'i>>>, Spanned<'i, &'i ::xst::internal::Str>>,
}

const _: () = {
    impl<'i> ::xst::internal::Shard for JSONArray<'i> {
        type Data = ::xst::internal::Sequence<(
            ::xst::internal::Literal<Literal0>,
            ::xst::internal::Extern<WS<'static>>,
            ::xst::internal::Extern<
                Punctuated<
                    'static,
                    ::xst::internal::Extern<
                        Spanned<'static, ::xst::internal::Extern<JSONValue<'static>>>,
                    >,
                    ::xst::internal::Extern<Spanned<'static, ::xst::internal::Literal<Literal1>>>,
                >,
            >,
            ::xst::internal::Literal<Literal2>,
        )>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::Str = "[";
    }

    #[allow(dead_code)]
    pub struct Literal1;

    impl ::xst::internal::ShardLiteral for Literal1 {
        const LITERAL: &'static ::xst::internal::Str = ",";
    }

    #[allow(dead_code)]
    pub struct Literal2;

    impl ::xst::internal::ShardLiteral for Literal2 {
        const LITERAL: &'static ::xst::internal::Str = "]";
    }

    impl<'i> ::xst::internal::StaticShard for JSONArray<'i> {}
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
    _i: ::xst::internal::PhantomData<&'i ()>,
    text: &'i ::xst::internal::Str,
}

const _: () = {
    impl<'i> ::xst::internal::Shard for JSONNull<'i> {
        type Data = ::xst::internal::Literal<Literal0>;
    }

    #[allow(dead_code)]
    pub struct Literal0;

    impl ::xst::internal::ShardLiteral for Literal0 {
        const LITERAL: &'static ::xst::internal::Str = "null";
    }

    impl<'i> ::xst::internal::StaticShard for JSONNull<'i> {}
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
    _i: ::xst::internal::PhantomData<&'i ()>,
    space: &'i ::xst::internal::Str,
}

const _: () = {
    impl<'i> ::xst::internal::Shard for WS<'i> {
        type Data = ::xst::internal::Vec<::xst::internal::Set<false, Set0>, 0, 0>;
    }

    #[allow(dead_code)]
    pub struct Set0;

    impl ::xst::internal::ShardSet for Set0 {
        const SET: &'static [::std::ops::RangeInclusive<char>] =
            &[' '..=' ', '\t'..='\t', '\n'..='\n', '\r'..='\r'];
    }

    impl<'i> ::xst::internal::StaticShard for WS<'i> {}
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
    _i: ::xst::internal::PhantomData<&'i ()>,
    inner: Option<(Box<T>, Vec<(P, T)>)>,
}

const _: () = {
    impl<'i, T, P> ::xst::internal::Shard for Punctuated<'i, T, P>
    where
        T: ::xst::internal::ShardParam,
        P: ::xst::internal::ShardParam,
    {
        type Data = ::xst::internal::Option<
            ::xst::internal::Sequence<(
                T,
                ::xst::internal::Vec<::xst::internal::Sequence<(P, T)>, 0, 0>,
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
    _i: ::xst::internal::PhantomData<&'i ()>,
    inner: T,
    ws: WS<'i>,
}

const _: () = {
    impl<'i, T> ::xst::internal::Shard for Spanned<'i, T>
    where
        T: ::xst::internal::ShardParam,
    {
        type Data = ::xst::internal::Sequence<(T, ::xst::internal::Extern<WS<'static>>)>;
    }
};

// ////////////////
// Spanned
// ////////////////
