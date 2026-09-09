# Shard expansion rules

`expand_item` is not implemented yet. `examples/json/src/cluster/mapping.rs`
contains the source declarations followed by their intended expansions. These
rules describe that expansion contract; they do not imply runtime extraction
has been implemented.

## Names and scopes

Emit only the source-declared shard identity/binding at module scope. Put all
literal/set descriptors, grammar aliases, and trait implementations inside a
per-declaration `const _: () = { ... };`. Helpers may be `pub` inside this block
when required by associated-type visibility, but must not be nameable in the
surrounding module. Reuse local names such as `Literal0` across expansions;
never invent module-level names such as `ObjectComma` to expose a helper.

Forward shard names are source-declared identities, not generated helper names.
Their `Shard::Output<'i>` is `&'i str`. Binding outputs are their generated
structs/enums. Do not inspect another declaration to decide its output type.

## Two type expressions per field

Generate the grammar descriptor and the output type independently:

- A named shard reference uses `Ext<S>` for grammar and `Output<'i, S>` for output.
- An inline `x!` expression captures `&'i str`. In generic grammar arguments,
  wrap its complete descriptor in `Capture<Pattern>` so inner bindings do not
  affect that output.
- Generic parameters carry grammar and output through `ShardParam`.
- Option, vector, tuple, and box bindings compose the corresponding output
  containers. Grammar lowering does not depend on these output containers.
- A generic shard application computes its output through `Shard::Output`,
  rather than assuming that the referenced shard is a binding.

If a field's output expression needs a local grammar helper (including helpers
nested inside generic shard arguments), emit `FieldOutput<'i, Owner, FIELD,
VARIANT>` in the struct/enum. Define `ShardField<FIELD, VARIANT>` for the static
owner inside the anonymous const, where all grammar helpers are in scope. The
associated output normalizes to the borrowed/binding output and does not store
grammar descriptors. Reuse a local grammar alias when it avoids generating the
same expression twice.

FIELD is the zero-based source field index before grouping repeated names.
VARIANT is the zero-based source variant index, or zero for structs. Repeated
field names form tuples of their individual output expressions. Generic owners
carry the same parameter bounds on their Shard and ShardField implementations.

Simple fields can use direct output types/projections to avoid unnecessary
ShardField implementations. The field projection is a type-level indirection;
it adds no runtime allocation or lookup. Keep the existing typed grammar
representation unless another requirement needs a value-based representation.

## Verification

Compile expansions with multiple declarations and user types sharing generated
helper names. Exercise nested generic applications and borrowed output values.
`xst-core/tests/output.rs` covers these contracts without depending on the
unfinished procedural macro implementation. Type-check the mapping module as
well; checking only the proc-macro crate cannot validate expansion examples.
