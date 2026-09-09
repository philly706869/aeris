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

## Deterministic identifiers

Traverse each source declaration in source order, depth-first and left-to-right
through field patterns and generic arguments. Assign a single zero-based leaf
counter to literal and character-set occurrences: `Literal0`, `Set1`, etc.
Reset it for each declaration. Count repeated occurrences separately; share the
assigned identifier between grammar and output generation rather than allocating
it again. Numbering must not depend on HashMap iteration, other declarations,
shard kinds, or allocation order. Source field/variant names and declared generic
parameter names are preserved from the source.

The lifetime marker uses `__xst_marker0`; if a source field already owns that
name, choose the first unused `__xst_markerN` in ascending order. It is a
synthetic field, not a parsed field, and does not get a ShardField index.
Forward shard marker storage is likewise not a binding field. Generated helper
identifiers must use macro hygiene so a same-spelled source path still resolves
to the source item. If an additional local grammar alias is needed, use
`GrammarN` with its source field index rather than a semantic name.

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

For every source binding field, unconditionally emit `FieldOutput<'i, Owner, FIELD>` in the struct/enum. Define `ShardField<FIELD>` for the static
owner inside the anonymous const, where all grammar helpers are in scope. The
associated output normalizes to the borrowed/binding output and does not store
grammar descriptors. Reuse a local grammar alias when it avoids generating the
same expression twice.

FIELD is a single zero-based index: the source field index before grouping
repeated names for structs, or the source variant index for enums. Repeated
struct field names form tuples of their individual output expressions.

Enum variants must be `Variant(ShardReference)`: exactly one named struct,
enum, or forward shard reference. Named-field variants, unit variants, empty
payloads, multiple payloads, and direct inline patterns/wrapper expressions are
not allowed. Generic arguments on the referenced shard follow the normal shard
reference rules. No separate variant index or nested enum field index exists.
The emitter need not know which kind of shard the reference denotes. Generic owners
carry the same ShardParam bounds on the binding type, Shard implementation,
and ShardField implementations. Preserve their grammar parameters in
Shard::Output: `Spanned<'i, T>`, not `Spanned<'i, T::Output<'i>>`. The input
lifetime is applied by each field projection. This ensures static grammar
identities never contain borrowed output parameters. For generated Debug impls,
bound the field projections instead of grammar parameters; place these impls
in the anonymous const as well.

Do not special-case literals, simple references, or generic parameters in the
struct/enum field declaration. Direct types and Output/ParamOutput projections
belong only in the corresponding ShardField implementation. The field
projection is a type-level indirection;
it adds no runtime allocation or lookup. Keep the existing typed grammar
representation unless another requirement needs a value-based representation.

## Verification

Compile expansions with multiple declarations and user types sharing generated
helper names. Exercise nested generic applications and borrowed output values.
`xst-core/tests/output.rs` covers these contracts without depending on the
unfinished procedural macro implementation. Type-check the mapping module as
well; checking only the proc-macro crate cannot validate expansion examples.

## Debug contract

`ShardField::Output<'i>` and generic `ShardParam::Output<'i>` implement Debug.
`Ext<S>` is a ShardParam when every `S::Output<'i>` implements Debug; grammar-only
references remain unrestricted. Sequence arguments require Debug on the complete
output tuple (the standard library implements tuple Debug only up to its supported
arity). Generated Debug implementations need no additional field-output bounds
and must not require Debug on grammar descriptor types themselves.
