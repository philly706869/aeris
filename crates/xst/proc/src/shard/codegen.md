# Shard expansion rules

`expand_item` is not implemented yet. `examples/json/src/cluster/mapping.rs`
contains source declarations followed by their intended expansions. This file
describes that expansion contract against the current `xst-core` shard API; it
does not imply that parsing or output construction is implemented.

## Public shape and local helpers

A struct or enum shard becomes a binding with an input lifetime. Preserve its
visibility, name, fields or variants, and declared type parameters. Insert the
input lifetime before the declared type parameters and bind every declared type
parameter with `::xst::internal::Shard`.

A type shard becomes a same-named tuple marker carrying
`PhantomData<&'i ()>`. Preserve the alias visibility: a private alias produces a
private marker. Its `ShardCore::Output<'i>` is `&'i str`; struct and enum cores
output their generated binding.

Derive `::xst::internal::Debug` for each generated binding or marker. A struct
binding also contains a synthetic
`__xst_marker_0: PhantomData<&'i ()>` field. If that name is occupied, choose the
first unused `__xst_marker_N` in ascending order. The marker is not parsed and
does not appear in `ShardData`.

Put every implementation and generated helper in a per-declaration
`const _: () = { ... };`. Helpers that occur in public associated types must be
`pub` inside the block, but remain unnameable from the surrounding module. This
allows names such as `__xst_shard_core_0` and `__xst_shard_closure_0` to be
reused for every declaration. Generated identifiers and framework paths must
use macro hygiene.

Do not emit `#[allow(dead_code)]` for generated helpers. Emit
`#[allow(non_camel_case_types)]` for lower-case helper names.

## Shard identity and core

For a non-generic binding `S`, generate:

```rust
impl StaticShard for S<'static> {}

impl Shard for S<'static> {
    type Core = __xst_shard_core_0;
}
```

For a generic binding, implement `Shard` for
`S<'static, T, ...>` and use `__xst_shard_core_0<T, ...>` as its core. Repeat the
`T: Shard` bounds on the binding, the `Shard` implementation, the core helper,
and its `ShardCore` implementation. Generic shards are not `StaticShard` because
the parameters need not denote a single closed static root.

The core helper is a public, zero-sized tuple struct. Its marker is
`PhantomData<fn() -> ()>` without parameters and
`PhantomData<fn() -> (T, ...)>` with parameters. Implement `ShardCore` on it:

```rust
impl ShardCore for __xst_shard_core_0 {
    type Output<'i> = S<'i>;
    const DATA: &'static ShardData = /* lowered source grammar */;
}
```

`Shard` is the externally named grammar identity. `Shard::Core` is the hidden,
canonical identity used by `TypeId`, grammar traversal, output projection, and
field forwarding. Never inspect another declaration or shard kind while
generating a reference.

## Lowering grammar to `ShardData`

Lower the source grammar directly to nested const `ShardData` values:

- string literal: `ShardData::literal(text)`
- character set: `ShardData::set(negated, &[start..=end, ...])`
- optional pattern: `ShardData::option(&item)`
- repetition: `ShardData::vec(&item, min, max)`
- tuple or concatenation: `ShardData::sequence(&[...])`
- alternative: `ShardData::alternative(&[...])`
- named shard or generic parameter: `ShardData::reference::<S>()`

Use `::xst::internal::Option::{None, Some}` for repetition bounds and internal
re-exports for all other generated framework and standard-library paths.
Preserve source order, traversing depth-first and left-to-right. A single-item
struct or alias uses the item's data directly rather than wrapping it in a
one-element sequence. Repeated struct fields still contribute separate grammar
items in source order.

Every named reference must be a static shard identity. Apply `'static` to the
input-lifetime position of generated shard bindings, including nested generic
applications; leave a generic shard parameter such as `T` unchanged. For
example:

```rust
ShardData::reference::<WS<'static>>()
ShardData::reference::<Spanned<'static, JSONValue<'static>>>()
ShardData::reference::<T>()
```

`ShardData::reference` resolves the referenced `Shard::Core`, so recursive and
multiple source identities that share a core retain the core's canonical
identity.

## Binding field outputs

Compute binding output independently from `ShardData` lowering:

- an inline `x!` pattern captures the complete matched slice as `&'i str`
- a named shard `S` becomes `ShardField<'i, S>`
- a generic parameter `T` becomes `ShardField<'i, T>`
- `xopt!`, `xvec!`, `xbox!`, and tuple bindings recursively compose `Option`,
  `Vec`, `Box`, and tuples around their child outputs

Use the internal re-exports (`str`, `Option`, `Vec`, and `Box`) in emitted code.
An inline pattern always produces one string slice even when its grammar
contains alternatives, sequences, repetitions, or references.

For structs, group repeated source field names into one binding field whose
type is a tuple of the occurrence outputs in source order. Preserve the order of
the first occurrence when ordering generated fields.

Enum variants must have exactly one unnamed named-shard payload:
`Variant(ShardReference)`. Unit variants, named fields, multiple payloads, and
direct inline or wrapper patterns are invalid. Each generated variant stores
`ShardField<'i, ReferencedShard>`, while the enum grammar is an alternative of
the corresponding references in variant order.

## Private closure identities and field forwarding

An inline `x!` used as a shard-valued generic argument needs a stable shard
identity. Generate a public local helper named `__xst_shard_closure_N`, numbered
from zero in deterministic depth-first, left-to-right order for the declaration.
The helper:

- is a zero-sized `PhantomData<fn() -> ()>` tuple struct
- derives `Debug`
- implements `Shard<Core = Self>`
- implements `ShardCore<Output<'i> = &'i str>` with the inline pattern's data

The binding cannot name this block-local helper in its module-scope field type.
For every field whose fully composed output mentions such a helper, emit
`ShardFieldReference<'i, Owner<'static, ...>, INDEX>` instead. `INDEX` is a
zero-based counter over forwarded fields only, in source order. Then implement
the forwarding trait on the owner's core, not on the owner:

```rust
impl ShardFieldForward<INDEX> for __xst_shard_core_0<...> {
    type Field<'i> = /* complete output type, local helpers allowed */;
}
```

This matches the core API definition:

```rust
type ShardFieldReference<'i, T, const INDEX: usize> =
    <<T as Shard>::Core as ShardFieldForward<INDEX>>::Field<'i>;
```

Do not introduce forwarding for ordinary literals, references, generic
parameters, or composed outputs that can be named at module scope. The
projection is compile-time type indirection and adds no runtime allocation or
lookup.

## Determinism and verification

All generated numbering resets for each source declaration and must depend only
on source traversal, never on map iteration or allocation order. Preserve source
field names, variant names, generic parameter names, visibility, and grammar
order.

Use `examples/json/src/cluster/mapping.rs` as the code-generation golden model.
In particular, verify:

- ordinary, repeated, optional, tuple, vector, and boxed field outputs
- type-shard string output
- struct and enum bindings
- generic `Punctuated` and `Spanned` identities
- local comma closures used by `JSONObject` and `JSONArray`
- `ShardFieldForward` implementations on `__xst_shard_core_0`
- construction of a `Cluster<JSON>` from the complete mapping

Type-check the mapping and `xst-core` together. Checking only the proc-macro
crate cannot validate the generated API contract.
