#[cfg(test)]
mod test {
    use xst::Cluster;

    use super::JSON;

    #[test]
    fn test() {
        let cluster: Cluster<JSON> = Cluster::build();
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
    impl ::xst::internal::Shard for JSON<'static> {
        type Data = ::xst::internal::Alternative<(
            ::xst::internal::Extern<WS<'static>>,
            ::xst::internal::Extern<JSONValue<'static>>,
            ::xst::internal::Extern<WS<'static>>,
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
    impl ::xst::internal::Shard for JSONValue<'static> {
        type Data = ::xst::internal::Alternative<(
            ::xst::internal::Extern<JSONArray<'static>>,
            ::xst::internal::Extern<JSONNull<'static>>,
        )>;
    }

    impl ::xst::internal::StaticShard for JSONValue<'static> {}
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
    entries: Punctuated<Spanned<Box<JSONValue>>, Spanned<x![","]>>,
    bracket: x!["["],
}

#[derive(Debug)]
pub struct JSONArray<'i> {
    _i: ::xst::internal::PhantomData<&'i ()>,
    bracket: (&'i ::xst::internal::str, &'i ::xst::internal::str),
    ws: WS<'i>,
    entries: Punctuated<'i, Spanned<'i, Box<JSONValue<'i>>>, Spanned<'i, &'i ::xst::internal::str>>,
}

const _: () = {
    impl ::xst::internal::Shard for JSONArray<'static> {
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
    text: &'i ::xst::internal::str,
}

const _: () = {
    impl ::xst::internal::Shard for JSONNull<'static> {
        type Data = ::xst::internal::Literal<Literal0>;
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

#[cfg(false)]
#[shard]
pub struct WS {
    space: x![{' ' '\t' '\n' '\r'}*],
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

#[cfg(false)]
#[shard]
pub struct Punctuated<T, P> {
    inner: Option<(Box<T>, vec![(P, T); ..])>,
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
    impl<T> ::xst::internal::Shard for Spanned<'static, T>
    where
        T: ::xst::internal::ShardParam,
    {
        type Data = ::xst::internal::Sequence<(T, ::xst::internal::Extern<WS<'static>>)>;
    }
};

// ////////////////
// Spanned
// ////////////////
