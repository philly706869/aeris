#![allow(dead_code)]

use xst::{Cluster, shard};

mod json {
    include!("fixtures/json.rs");
}

#[shard]
type Unicode = x! { "한🦀" };

#[shard]
type Overlap = x! { [| {'a'..'z'} "x" | "a" "y" | {! '0'..'9'} "z"] };

#[shard]
struct OptionalB {
    value: xopt![x! { "b" }],
}

#[shard]
struct OptionalEmpty {
    value: xopt![x! { "" }],
}

#[shard]
type Bounded = x! { "a"^[2..4] OptionalB "c"*? };

#[shard]
type Ambiguous = x! { [| Ambiguous Ambiguous | "a"] };

#[shard]
type Nullable = x! { [| Nullable Nullable | "" | "a"] };

#[shard]
type EmptyRepeat = x! { ""* };

#[shard]
type MutualA = x! { [| MutualB | "a"] };

#[shard]
type MutualB = x! { MutualA };

#[shard]
type Unproductive = x! { Unproductive };

#[shard]
type ReduceConflict = x! { [| ("a" OptionalEmpty) "x" | ("a" ""*?) "y"] };

#[test]
fn consumes_the_entire_unicode_input() {
    let cluster = Cluster::<Unicode>::build();
    assert!(cluster.parse("한🦀"));
    for input in ["", "한", "🦀", "한🦀x", "x한🦀"] {
        assert!(!cluster.parse(input), "{input:?}");
    }
    assert!(cluster.parse("한🦀"));
}

#[test]
fn explores_all_matching_terminal_predicates() {
    let cluster = Cluster::<Overlap>::build();
    for input in ["ax", "ay", "az", "bx", "🦀z"] {
        assert!(cluster.parse(input), "{input:?}");
    }
    for input in ["", "a", "by", "0z", "axy"] {
        assert!(!cluster.parse(input), "{input:?}");
    }
}

#[test]
fn handles_bounded_optional_and_lazy_repetition() {
    let cluster = Cluster::<Bounded>::build();
    for n in 0..7 {
        for suffix in ["", "b", "c", "bccc", "bb", "cb"] {
            let input = format!("{}{suffix}", "a".repeat(n));
            assert_eq!(
                cluster.parse(&input),
                (2..=4).contains(&n) && !["bb", "cb"].contains(&suffix),
                "{input:?}"
            );
        }
    }
}

#[test]
fn shares_ambiguous_stacks_and_preserves_reduction_branches() {
    let cluster = Cluster::<Ambiguous>::build();
    for n in [1, 2, 3, 8, 32] {
        assert!(cluster.parse(&"a".repeat(n)));
    }
    assert!(!cluster.parse(""));
    assert!(!cluster.parse("aaab"));
    let conflict = Cluster::<ReduceConflict>::build();
    assert!(conflict.parse("ax"));
    assert!(conflict.parse("ay"));
    assert!(!conflict.parse("az"));
}

#[test]
fn nullable_and_mutual_cycles_terminate() {
    let nullable = Cluster::<Nullable>::build();
    for input in ["", "a", "aa", "aaaa"] {
        assert!(nullable.parse(input), "{input:?}");
    }
    assert!(!nullable.parse("aab"));
    let empty = Cluster::<EmptyRepeat>::build();
    assert!(empty.parse(""));
    assert!(!empty.parse("a"));
    let mutual = Cluster::<MutualA>::build();
    assert!(mutual.parse("a"));
    assert!(!mutual.parse(""));
    assert!(!mutual.parse("aa"));
    let unproductive = Cluster::<Unproductive>::build();
    assert!(!unproductive.parse(""));
    assert!(!unproductive.parse("a"));
}

#[test]
fn parses_recursive_json_without_extracting_values() {
    let cluster = Cluster::<json::JSON>::build();
    for input in [
        "null",
        " true \n",
        "-12.5e+2",
        "[]",
        "{}",
        r#"{"한🦀": [null, true, false, -12.5e+2, {"x": "a\n\u0041"}]}"#,
    ] {
        assert!(cluster.parse(input), "{input:?}");
    }
    for input in [
        "",
        "nul",
        "null true",
        "01",
        "1.",
        "[1,]",
        "{\"a\" 1}",
        "\"unterminated",
        "\"bad\nstring\"",
        "\"\\q\"",
    ] {
        assert!(!cluster.parse(input), "{input:?}");
    }
}
