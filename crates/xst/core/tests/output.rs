//! Hand-written expansions: field generation never inspects the referenced shard.
use xst_core::{Cluster, internal::*};

struct Text;
impl ShardLiteral for Text {
    const LITERAL: &'static str = "hello";
}

// #[shard] type Foo = x! { "hello" };
struct Foo;
impl Shard for Foo {
    type Data = Lit<Text>;
    type Output<'i> = &'i str;
}
impl StaticShard for Foo {}

// #[shard] struct Bar { foo: Foo }
#[derive(Debug)]
struct Bar<'i> {
    foo: Output<'i, Foo>,
}
impl Shard for Bar<'static> {
    type Data = Ext<Foo>;
    type Output<'i> = Bar<'i>;
}
impl StaticShard for Bar<'static> {}

#[derive(Debug)]
enum Binding<'i> {
    Forward(Output<'i, Foo>),
    Struct(Output<'i, Bar<'static>>),
}
impl Shard for Binding<'static> {
    type Data = Alt<(Ext<Foo>, Ext<Bar<'static>>)>;
    type Output<'i> = Binding<'i>;
}
impl StaticShard for Binding<'static> {}

// Generic bindings substitute *output types* into the output struct, while
// their static grammar identity continues to use grammar descriptors.
struct Wrapper<T>(T);
impl<T: ShardParam> Shard for Wrapper<T> {
    type Data = T;
    type Output<'i> = Wrapper<ParamOutput<'i, T>>;
}

fn wrap<'i, T: ShardParam>(value: ParamOutput<'i, T>) -> Output<'i, Wrapper<T>> {
    Wrapper(value)
}

#[test]
fn forward_and_binding_outputs_borrow_the_input() {
    let input = String::from("hello");
    let forward: Output<'_, Foo> = input.as_str();
    let binding: Output<'_, Bar<'static>> = Bar { foo: forward };
    assert_eq!(binding.foo.as_ptr(), input.as_ptr());
    let variants: [Output<'_, Binding<'static>>; 2] =
        [Binding::Forward(forward), Binding::Struct(binding)];
    for variant in variants {
        let text = match variant {
            Binding::Forward(text) => text,
            Binding::Struct(bar) => bar.foo,
        };
        assert_eq!(text, input);
    }
    assert_eq!(Cluster::<Foo>::build().parse(&input), Ok(()));
    assert_eq!(Cluster::<Bar<'static>>::build().parse(&input), Ok(()));
    assert_eq!(Cluster::<Binding<'static>>::build().parse(&input), Ok(()));
}

#[test]
fn generic_arguments_preserve_their_output_types() {
    let input = String::from("hello");
    let forward = wrap::<Ext<Foo>>(&input);
    let binding = wrap::<Ext<Bar<'static>>>(Bar { foo: &input });
    let literal = wrap::<Lit<Text>>(&input);
    let nested = wrap::<Ext<Wrapper<Ext<Foo>>>>(forward);
    assert_eq!(nested.0.0, input);
    assert_eq!(binding.0.foo, input);
    assert_eq!(literal.0, input);

    type Args = Seq<(Opt<Ext<Foo>>, Vec<Ext<Bar<'static>>, 0, 2>)>;
    let composite = wrap::<Args>((Some(&input), vec![Bar { foo: &input }]));
    assert_eq!(composite.0.0, Some(input.as_str()));
    assert_eq!(composite.0.1[0].foo, input);
}

#[test]
fn inline_pattern_capture_ignores_internal_bindings() {
    type Pattern = Seq<(Ext<Foo>, Opt<Ext<Bar<'static>>>)>;
    let input = String::from("hellohello");
    let value = wrap::<Capture<Pattern>>(&input);
    assert_eq!(value.0, input);
    // Capturing only changes the output contract, not the grammar.
    assert!(core::ptr::eq(
        <Capture<Pattern> as xst_core::internal::ShardDataType>::DATA,
        <Pattern as xst_core::internal::ShardDataType>::DATA,
    ));
}
