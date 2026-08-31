use aeris::ui::Cluster;

static JSON_CLUSTER: Cluster<JSON<'_>> = Cluster::build();

fn test<'i>(input: &'i str) -> JSON<'i> {
    JSON_CLUSTER.parse(input)
}

#[derive(Debug)]
pub struct JSON<'i> {
    ws: (WS<'i>, WS<'i>),
    value: JSONValue<'i>,
}

const _: () = {
    use ::aeris::ui::ShardData;
    use ::std::any::TypeId;
    #[allow(dead_code)]
    struct STATIC;
    impl<'i> ::aeris::ui::Shard for JSON<'i> {
        const TUID: TypeId = TypeId::of::<STATIC>();
    }
    impl<'i> ::aeris::ui::StaticShard<'i> for JSON<'i> {
        const DATA: &'static ShardData = &ShardData::Sequence(&[]);
    }
};

#[derive(Debug)]
pub enum JSONValue<'i> {
    Array(JSONArray<'i>),
    Null(JSONNull<'i>),
}

const _: () = {
    use ::aeris::ui::ShardData;
    use ::std::any::TypeId;
    #[allow(dead_code)]
    struct STATIC;
    impl<'i> ::aeris::ui::Shard for JSONValue<'i> {
        const TUID: TypeId = TypeId::of::<STATIC>();
    }
    impl<'i> ::aeris::ui::StaticShard<'i> for JSONValue<'i> {
        const DATA: &'static ShardData = &ShardData::Literal("");
    }
};

#[derive(Debug)]
pub struct JSONArray<'i> {
    _i: ::std::marker::PhantomData<&'i Self>,
    bracket: (&'i str, &'i str),
    ws: WS<'i>,
    entries: Punctuated<'i, Spanned<'i, &'i str /* JSONValue<'i> */>, Spanned<'i, &'i str>>,
}

const _: () = {
    use ::aeris::ui::ShardData;
    use ::std::any::TypeId;
    #[allow(dead_code)]
    struct STATIC;
    impl<'i> ::aeris::ui::Shard for JSONArray<'i> {
        const TUID: TypeId = TypeId::of::<STATIC>();
    }
    impl<'i> ::aeris::ui::StaticShard<'i> for JSONArray<'i> {
        const DATA: &'static ShardData = &ShardData::Literal("");
    }
};

#[derive(Debug)]
pub struct JSONNull<'i> {
    _i: ::std::marker::PhantomData<&'i Self>,
    text: &'i str,
}

const _: () = {
    use ::aeris::ui::ShardData;
    use ::std::any::TypeId;
    #[allow(dead_code)]
    struct STATIC;
    impl<'i> ::aeris::ui::Shard for JSONNull<'i> {
        const TUID: TypeId = TypeId::of::<STATIC>();
    }
    impl<'i> ::aeris::ui::StaticShard<'i> for JSONNull<'i> {
        const DATA: &'static ShardData = &ShardData::Literal("null");
    }
};

#[derive(Debug)]
pub struct WS<'i> {
    _i: ::std::marker::PhantomData<&'i Self>,
    space: &'i str,
}

const _: () = {
    use ::aeris::ui::ShardData;
    use ::std::any::TypeId;
    #[allow(dead_code)]
    struct STATIC;
    impl<'i> ::aeris::ui::Shard for WS<'i> {
        const TUID: TypeId = TypeId::of::<STATIC>();
    }
    impl<'i> ::aeris::ui::StaticShard<'i> for WS<'i> {
        const DATA: &'static ShardData = &ShardData::Vector(
            &ShardData::Set(false, &[' '..=' ', '\t'..='\t', '\n'..='\n', '\r'..='\r']),
            0,
            0,
        );
    }
};

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
    use ::aeris::ui::ShardEntry;
    use ::std::any::TypeId;
    #[allow(dead_code)]
    struct STATIC;
    impl<'i, T, P> ::aeris::ui::Shard for Punctuated<'i, T, P>
    where
        T: ShardEntry,
        P: ShardEntry,
    {
        const TUID: TypeId = TypeId::of::<STATIC>();
    }
    impl<'i, T, P> ::aeris::ui::DynamicShard for Punctuated<'i, T, P>
    where
        T: ShardEntry,
        P: ShardEntry,
    {
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
    use ::aeris::ui::ShardEntry;
    use ::std::any::TypeId;
    #[allow(dead_code)]
    struct STATIC;
    impl<'i, T> ::aeris::ui::Shard for Spanned<'i, T>
    where
        T: ShardEntry,
    {
        const TUID: TypeId = TypeId::of::<STATIC>();
    }
    impl<'i, T> ::aeris::ui::DynamicShard for Spanned<'i, T> where T: ShardEntry {}
};
