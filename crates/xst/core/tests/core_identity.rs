use core::{any::TypeId, marker::PhantomData};
use xst_core::{Cluster, internal::*};

struct Borrowed<'a>(PhantomData<&'a str>);
struct Other<'a>(PhantomData<&'a str>);

const _: () = {
    pub struct Literal0;
    impl ShardLiteral for Literal0 {
        const LITERAL: &'static str = "a";
    }

    pub struct Core0(PhantomData<fn() -> ()>);
    pub struct Core1(PhantomData<fn() -> ()>);
    impl<'a> Shard for Borrowed<'a> {
        type Core = Core0;
    }
    impl<'a> StaticShard for Borrowed<'a> {}
    impl<'a> Shard for Other<'a> {
        type Core = Core1;
    }
    impl ShardCore for Core0 {
        type Data = Lit<Literal0>;
        type Output<'i> = &'i str;
    }
    impl ShardCore for Core1 {
        // Equal grammar does not imply equal core identity.
        type Data = Lit<Literal0>;
        type Output<'i> = &'i str;
    }
};

fn extern_id<T: ShardDataType>() -> TypeId {
    match T::DATA {
        ShardData::Extern(data) => data.id,
        _ => panic!("expected an external reference"),
    }
}

#[test]
fn borrowed_shards_resolve_to_static_cores() {
    fn check<'a>(input: &'a str) {
        assert_eq!(Cluster::<Borrowed<'a>>::build().parse(input), Ok(()));
        assert_eq!(
            TypeId::of::<Ext<Borrowed<'a>>>(),
            TypeId::of::<Ext<Borrowed<'static>>>()
        );
        assert_eq!(
            extern_id::<Ext<Borrowed<'a>>>(),
            TypeId::of::<<Borrowed<'a> as Shard>::Core>()
        );
        let output: ParamOutput<'a, Ext<Borrowed<'a>>> = input;
        assert_eq!(output.as_ptr(), input.as_ptr());
    }
    let input = String::from("a");
    check(&input);
}

#[test]
fn different_cores_with_identical_data_keep_distinct_ids() {
    assert_eq!(
        TypeId::of::<<<Borrowed<'static> as Shard>::Core as ShardCore>::Data>(),
        TypeId::of::<<<Other<'static> as Shard>::Core as ShardCore>::Data>(),
    );
    assert_ne!(
        extern_id::<Ext<Borrowed<'static>>>(),
        extern_id::<Ext<Other<'static>>>()
    );
}
