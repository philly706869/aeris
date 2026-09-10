//! Visibility experiments for generated code. All generated helpers stay local.
use xst_core::{Cluster, internal::*};

#[derive(Debug)]
struct PrivateBinding<'i>(&'i str);

const _: () = {
    pub struct Literal0;
    impl ShardLiteral for Literal0 {
        const LITERAL: &'static str = "value";
    }
    impl Shard for PrivateBinding<'static> {
        type Core = PrivateBinding<'static>;
    }
    impl ShardCore for PrivateBinding<'static> {
        type Data = Lit<Literal0>;
        type Output<'i> = PrivateBinding<'i>;
    }
};

pub struct PublicBinding<'i> {
    value: FieldOutput<'i, PublicBinding<'static>, 0>,
}

const _: () = {
    // Forwarding a *value* hides the private grammar identity without changing
    // the grammar. The public associated type mentions only this local helper.
    pub struct Grammar0;
    impl ShardDataType for Grammar0 {
        const DATA: &'static xst_core::internal::ShardData =
            <<PrivateBinding<'static> as Shard>::Core as ShardCore>::Data::DATA;
    }

    // A new output type can contain a private binding. This is an opaque
    // wrapper, not a transparent associated-type alias to PrivateBinding.
    #[derive(Debug)]
    pub struct Output0<'i>(Output<'i, PrivateBinding<'static>>);

    impl Shard for PublicBinding<'static> {
        type Core = PublicBinding<'static>;
    }

    impl ShardCore for PublicBinding<'static> {
        type Data = Grammar0;
        type Output<'i> = PublicBinding<'i>;
    }
    impl StaticShard for PublicBinding<'static> {}
    impl ShardField<0> for PublicBinding<'static> {
        type Output<'i> = Output0<'i>;
    }

    impl<'i> PublicBinding<'i> {
        fn new(value: &'i str) -> Self {
            Self {
                value: Output0(PrivateBinding(value)),
            }
        }

        // Access can be generated here without exposing the private type in a
        // public associated type. Public operations must use public types.
        pub fn text(&self) -> &'i str {
            self.value.0.0
        }
    }
};

#[test]
fn local_public_adapters_hide_private_grammar_and_storage() {
    let input = String::from("value");
    let binding = PublicBinding::new(&input);
    assert_eq!(binding.text().as_ptr(), input.as_ptr());
    assert!(format!("{:?}", binding.value).contains("value"));
    let cluster = Cluster::<PublicBinding<'static>>::build();
    assert_eq!(cluster.parse(&input), Ok(()));
    assert!(cluster.parse("other").is_err());
}
