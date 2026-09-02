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

#[allow(unused_imports)]
const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardLiteral;
    use ::aeris::ui::ShardSet;
    use ::aeris::ui::ShardStatic;
    use ::aeris::ui::StaticShard;
    use ::aeris::ui::shards;
    use ::std::marker::PhantomData;
    use ::std::ops::RangeInclusive;

    impl<'i> Shard for JSON<'i> {
        type Static = Static;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static {}

    impl ShardStatic for Static {
        type Data = shards::Alternative<(
            shards::Extern<<WS<'static> as Shard>::Static>,
            shards::Extern<<JSONValue<'static> as Shard>::Static>,
            shards::Extern<<WS<'static> as Shard>::Static>,
        )>;
    }

    impl<'i> StaticShard for JSON<'i> {}
};

#[allow(non_snake_case)]
mod ___private_shard_JSON {
    #[warn(non_snake_case)]
    mod internal {}
}

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

#[allow(unused_imports)]
const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardLiteral;
    use ::aeris::ui::ShardSet;
    use ::aeris::ui::ShardStatic;
    use ::aeris::ui::StaticShard;
    use ::aeris::ui::shards;
    use ::std::marker::PhantomData;
    use ::std::ops::RangeInclusive;

    impl<'i> Shard for JSONValue<'i> {
        type Static = Static;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static {}

    impl ShardStatic for Static {
        type Data = shards::Alternative<(
            shards::Extern<<JSONArray<'static> as Shard>::Static>,
            shards::Extern<<JSONNull<'static> as Shard>::Static>,
        )>;
    }

    impl<'i> StaticShard for JSONValue<'i> {}
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
    _i: ::std::marker::PhantomData<&'i ()>,
    bracket: (&'i str, &'i str),
    ws: WS<'i>,
    entries: Punctuated<'i, Spanned<'i, Box<JSONValue<'i>>>, Spanned<'i, &'i str>>,
}

#[allow(unused_imports)]
const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardLiteral;
    use ::aeris::ui::ShardSet;
    use ::aeris::ui::ShardStatic;
    use ::aeris::ui::StaticShard;
    use ::aeris::ui::shards;
    use ::std::marker::PhantomData;
    use ::std::ops::RangeInclusive;

    impl<'i> Shard for JSONArray<'i> {
        type Static = Static;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static {}

    impl ShardStatic for Static {
        type Data = shards::Sequence<(
            shards::Literal<ShardLiteral0>,
            shards::Extern<<WS<'static> as Shard>::Static>,
            shards::Extern<
                <Punctuated<
                    'static,
                    Spanned<'static, JSONValue<'static>>,
                    Spanned<'static, ShardLiteral1>,
                > as Shard>::Static,
            >,
        )>;
    }

    #[allow(dead_code)]
    struct ShardLiteral0;

    impl ShardLiteral for ShardLiteral0 {
        const LITERAL: &'static str = "[";
    }

    #[allow(dead_code)]
    struct ShardLiteral1;

    impl ShardLiteral for ShardLiteral1 {
        const LITERAL: &'static str = ",";
    }

    #[allow(dead_code)]
    struct ShardLiteral2;

    impl ShardLiteral for ShardLiteral2 {
        const LITERAL: &'static str = "]";
    }

    impl<'i> StaticShard for JSONArray<'i> {}
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
    text: &'i str,
}

#[allow(unused_imports)]
const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardLiteral;
    use ::aeris::ui::ShardSet;
    use ::aeris::ui::ShardStatic;
    use ::aeris::ui::StaticShard;
    use ::aeris::ui::shards;
    use ::std::marker::PhantomData;
    use ::std::ops::RangeInclusive;

    impl<'i> Shard for JSONNull<'i> {
        type Static = Static;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static {}

    impl ShardStatic for Static {
        type Data = shards::Literal<ShardLiteral0>;
    }

    struct ShardLiteral0;

    impl ShardLiteral for ShardLiteral0 {
        const LITERAL: &'static str = "null";
    }

    impl<'i> StaticShard for JSONNull<'i> {}
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
    space: &'i str,
}

#[allow(unused_imports)]
const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardLiteral;
    use ::aeris::ui::ShardSet;
    use ::aeris::ui::ShardStatic;
    use ::aeris::ui::StaticShard;
    use ::aeris::ui::shards;
    use ::std::marker::PhantomData;
    use ::std::ops::RangeInclusive;

    impl<'i> Shard for WS<'i> {
        type Static = Static;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static {}

    impl ShardStatic for Static {
        type Data = shards::Vec<shards::Set<ShardSet0, false>, 0, 0>;
    }

    struct ShardSet0;

    impl ShardSet for ShardSet0 {
        const SET: &'static [RangeInclusive<char>] =
            &[' '..=' ', '\t'..='\t', '\n'..='\n', '\r'..='\r'];
    }

    impl<'i> StaticShard for WS<'i> {}
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

#[allow(unused_imports)]
const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardLiteral;
    use ::aeris::ui::ShardSet;
    use ::aeris::ui::ShardStatic;
    use ::aeris::ui::StaticShard;
    use ::aeris::ui::shards;
    use ::std::marker::PhantomData;
    use ::std::ops::RangeInclusive;

    impl<'i, T, P> Shard for Punctuated<'i, T, P>
    where
        T: Shard + 'static,
        P: Shard + 'static,
    {
        type Static = Static<<T as Shard>::Static, <P as Shard>::Static>;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static<T, P> {
        T: PhantomData<T>,
        P: PhantomData<P>,
    }

    impl<T, P> ShardStatic for Static<T, P>
    where
        T: ShardStatic,
        P: ShardStatic,
    {
        type Data = shards::Option<
            shards::Sequence<(
                shards::Extern<T>,
                shards::Vec<shards::Sequence<(shards::Extern<P>, shards::Extern<T>)>, 0, 0>,
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

#[allow(unused_imports)]
const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardLiteral;
    use ::aeris::ui::ShardSet;
    use ::aeris::ui::ShardStatic;
    use ::aeris::ui::StaticShard;
    use ::aeris::ui::shards;
    use ::std::marker::PhantomData;
    use ::std::ops::RangeInclusive;

    impl<'i, T> Shard for Spanned<'i, T>
    where
        T: Shard + 'static,
    {
        type Static = Static<<T as Shard>::Static>;
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct Static<T> {
        T: PhantomData<T>,
    }

    impl<T> ShardStatic for Static<T>
    where
        T: ShardStatic,
    {
        type Data = shards::Sequence<(
            shards::Extern<T>,
            shards::Extern<<WS<'static> as Shard>::Static>,
        )>;
    }
};

// ////////////////
// Spanned
// ////////////////
