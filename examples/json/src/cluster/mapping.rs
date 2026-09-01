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
    use ::aeris::ui::DynamicShard;
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::aeris::ui::StaticShard;
    use ::std::any::TypeId;
    use ::std::marker::PhantomData;

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct TUID {}

    impl<'i> Shard for JSON<'i> {
        type TUID = TUID;
    }

    #[allow(dead_code)]
    const fn tuid<S>() -> TypeId
    where
        S: Shard,
    {
        TypeId::of::<S::TUID>()
    }

    impl<'i> StaticShard for JSON<'i> {
        const DATA: &'static ShardData = &ShardData::Sequence(&[
            &ShardData::Extern(tuid::<WS>(), || WS::DATA),
            &ShardData::Extern(tuid::<JSONValue>(), || JSONValue::DATA),
            &ShardData::Extern(tuid::<WS>(), || WS::DATA),
        ]);
    }
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

#[allow(unused_imports)]
const _: () = {
    use ::aeris::ui::DynamicShard;
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::aeris::ui::StaticShard;
    use ::std::any::TypeId;
    use ::std::marker::PhantomData;

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct TUID {}

    impl<'i> Shard for JSONValue<'i> {
        type TUID = TUID;
    }

    #[allow(dead_code)]
    const fn tuid<S>() -> TypeId
    where
        S: Shard,
    {
        TypeId::of::<S::TUID>()
    }

    impl<'i> StaticShard for JSONValue<'i> {
        const DATA: &'static ShardData = &ShardData::Alternative(&[
            &ShardData::Extern(tuid::<JSONArray>(), || JSONArray::DATA),
            &ShardData::Extern(tuid::<JSONNull>(), || JSONNull::DATA),
        ]);
    }
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
    entries: Punctuated<'i, Spanned<'i, JSONValue<'i>>, Spanned<'i, &'i str>>,
}

#[allow(unused_imports)]
const _: () = {
    use ::aeris::ui::DynamicShard;
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::aeris::ui::StaticShard;
    use ::std::any::TypeId;
    use ::std::marker::PhantomData;

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct TUID {}

    impl<'i> Shard for JSONArray<'i> {
        type TUID = TUID;
    }

    #[allow(dead_code)]
    const fn tuid<S>() -> TypeId
    where
        S: Shard,
    {
        TypeId::of::<S::TUID>()
    }

    impl<'i> StaticShard for JSONArray<'i> {
        const DATA: &'static ShardData = &ShardData::Sequence(&[
            &ShardData::Literal("["),
            &ShardData::Extern(tuid::<WS>(), || WS::DATA),
            &ShardData::Extern(
                tuid::<Punctuated<Spanned<JSONValue>, Spanned<LambdaShard0>>>(),
                || todo!(), /* Punctuated::<Spanned<JSONValue>, Spanned<LambdaShard0>>::DATA */
            ),
            &ShardData::Extern(tuid::<WS>(), || WS::DATA),
            &ShardData::Literal("]"),
        ]);
    }

    struct LambdaShard0<'i>(PhantomData<&'i ()>);

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    struct LambdaShard0TUID {}

    impl<'i> Shard for LambdaShard0<'i> {
        type TUID = LambdaShard0TUID;
    }

    impl<'i> StaticShard for LambdaShard0<'i> {
        const DATA: &'static ShardData = &ShardData::Literal(",");
    }
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
    use ::aeris::ui::DynamicShard;
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::aeris::ui::StaticShard;
    use ::std::any::TypeId;
    use ::std::marker::PhantomData;

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct TUID {}

    impl<'i> Shard for JSONNull<'i> {
        type TUID = TUID;
    }

    #[allow(dead_code)]
    const fn tuid<S>() -> TypeId
    where
        S: Shard,
    {
        TypeId::of::<S::TUID>()
    }

    impl<'i> StaticShard for JSONNull<'i> {
        const DATA: &'static ShardData = &ShardData::Literal("null");
    }
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
    use ::aeris::ui::DynamicShard;
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::aeris::ui::StaticShard;
    use ::std::any::TypeId;
    use ::std::marker::PhantomData;

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct TUID {}

    impl<'i> Shard for WS<'i> {
        type TUID = TUID;
    }

    #[allow(dead_code)]
    const fn tuid<S>() -> TypeId
    where
        S: Shard,
    {
        TypeId::of::<S::TUID>()
    }

    impl<'i> StaticShard for WS<'i> {
        const DATA: &'static ShardData = &ShardData::Vector(
            &ShardData::Set(false, &[' '..=' ', '\t'..='\t', '\n'..='\n', '\r'..='\r']),
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
    use ::aeris::ui::DynamicShard;
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::aeris::ui::StaticShard;
    use ::std::any::TypeId;
    use ::std::marker::PhantomData;

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct TUID<T, P> {
        T: PhantomData<T>,
        P: PhantomData<P>,
    }

    impl<'i, T, P> Shard for Punctuated<'i, T, P>
    where
        T: Shard,
        P: Shard,
    {
        type TUID = TUID<T::TUID, P::TUID>;
    }

    #[allow(dead_code)]
    const fn tuid<S>() -> TypeId
    where
        S: Shard,
    {
        TypeId::of::<S::TUID>()
    }

    impl<'i, T, P> DynamicShard for Punctuated<'i, T, P>
    where
        T: Shard,
        P: Shard,
    {
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
    use ::aeris::ui::DynamicShard;
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::aeris::ui::StaticShard;
    use ::std::any::TypeId;
    use ::std::marker::PhantomData;

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub struct TUID<T> {
        T: PhantomData<T>,
    }

    impl<'i, T> Shard for Spanned<'i, T>
    where
        T: Shard,
    {
        type TUID = TUID<T::TUID>;
    }

    #[allow(dead_code)]
    const fn tuid<S>() -> TypeId
    where
        S: Shard,
    {
        TypeId::of::<S::TUID>()
    }

    impl<'i, T> ::aeris::ui::DynamicShard for Spanned<'i, T> where T: Shard {}
};

// ////////////////
// Spanned
// ////////////////
