#[derive(Debug)]
pub struct Punctuated<'i, T, P>
where
    T: ::aeris::ui::ShardEntry,
    P: ::aeris::ui::ShardEntry,
{
    _i: ::std::marker::PhantomData<&'i Self>,
    inner: Option<(T, Vec<(P, T)>)>,
}

const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::aeris::ui::ShardEntry;
    use ::std::any::TypeId;
    struct STATIC;
    impl<'i, T, P> Shard for Punctuated<'i, T, P>
    where
        T: ShardEntry,
        P: ShardEntry,
    {
        const TUID: TypeId = TypeId::of::<STATIC>();
        const DATA: &'static ShardData<'static> = &ShardData::Literal("");
    }
};

#[derive(Debug)]
pub struct Spanned<'i, T>
where
    T: ::aeris::ui::ShardEntry,
{
    _i: ::std::marker::PhantomData<&'i Self>,
    inner: T,
    ws: WS<'i>,
}

const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::aeris::ui::ShardEntry;
    use ::std::any::TypeId;
    struct STATIC;
    impl<'i, T> Shard for Spanned<'i, T>
    where
        T: ShardEntry,
    {
        const TUID: TypeId = TypeId::of::<STATIC>();
        const DATA: &'static ShardData<'static> = &ShardData::Literal("");
    }
};

#[derive(Debug)]
pub struct WS<'i> {
    _i: ::std::marker::PhantomData<&'i Self>,
    space: &'i str,
}

const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::std::any::TypeId;
    struct STATIC;
    impl<'i> Shard for WS<'i> {
        const TUID: TypeId = TypeId::of::<STATIC>();
        const DATA: &'static ShardData<'static> = &ShardData::Vector(
            &ShardData::Set(false, &[' '..=' ', '\t'..='\t', '\n'..='\n', '\r'..='\r']),
            0,
            0,
        );
    }
};

#[derive(Debug)]
pub enum JSONValue<'i> {
    Array(JSONArray<'i>),
    Null(JSONNull<'i>),
}

const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::std::any::TypeId;
    struct STATIC;
    impl<'i> Shard for JSONValue<'i> {
        const TUID: TypeId = TypeId::of::<STATIC>();
        const DATA: &'static ShardData<'static> = &ShardData::Literal("");
    }
};

#[derive(Debug)]
pub struct JSONArray<'i> {
    _i: ::std::marker::PhantomData<&'i Self>,
    bracket: (&'i str, &'i str),
    ws: WS<'i>,
    entries: Punctuated<'i, Spanned<'i, Box<JSONValue<'i>>>, Spanned<'i, &'i str>>,
}

const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::std::any::TypeId;
    struct STATIC;
    impl<'i> Shard for JSONArray<'i> {
        const TUID: TypeId = TypeId::of::<STATIC>();
        const DATA: &'static ShardData<'static> = &ShardData::Literal("");
    }
};

#[derive(Debug)]
pub struct JSONNull<'i> {
    _i: ::std::marker::PhantomData<&'i Self>,
    text: &'i str,
}

const _: () = {
    use ::aeris::ui::Shard;
    use ::aeris::ui::ShardData;
    use ::std::any::TypeId;
    struct STATIC;
    impl<'i> Shard for JSONNull<'i> {
        const TUID: TypeId = TypeId::of::<STATIC>();
        const DATA: &'static ShardData<'static> = &ShardData::Literal("null");
    }
};
