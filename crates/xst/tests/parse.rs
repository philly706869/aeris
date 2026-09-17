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
    assert!(cluster.recognizes("한🦀"));
    for input in ["", "한", "🦀", "한🦀x", "x한🦀"] {
        assert!(!cluster.recognizes(input), "{input:?}");
    }
    assert!(cluster.recognizes("한🦀"));
}

#[test]
fn explores_all_matching_terminal_predicates() {
    let cluster = Cluster::<Overlap>::build();
    for input in ["ax", "ay", "az", "bx", "🦀z"] {
        assert!(cluster.recognizes(input), "{input:?}");
    }
    for input in ["", "a", "by", "0z", "axy"] {
        assert!(!cluster.recognizes(input), "{input:?}");
    }
}

#[test]
fn handles_bounded_optional_and_lazy_repetition() {
    let cluster = Cluster::<Bounded>::build();
    for n in 0..7 {
        for suffix in ["", "b", "c", "bccc", "bb", "cb"] {
            let input = format!("{}{suffix}", "a".repeat(n));
            assert_eq!(
                cluster.recognizes(&input),
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
        assert!(cluster.recognizes(&"a".repeat(n)));
    }
    assert!(!cluster.recognizes(""));
    assert!(!cluster.recognizes("aaab"));
    let conflict = Cluster::<ReduceConflict>::build();
    assert!(conflict.recognizes("ax"));
    assert!(conflict.recognizes("ay"));
    assert!(!conflict.recognizes("az"));
}

#[test]
fn nullable_and_mutual_cycles_terminate() {
    let nullable = Cluster::<Nullable>::build();
    for input in ["", "a", "aa", "aaaa"] {
        assert!(nullable.recognizes(input), "{input:?}");
    }
    assert!(!nullable.recognizes("aab"));
    let empty = Cluster::<EmptyRepeat>::build();
    assert!(empty.recognizes(""));
    assert!(!empty.recognizes("a"));
    let mutual = Cluster::<MutualA>::build();
    assert!(mutual.recognizes("a"));
    assert!(!mutual.recognizes(""));
    assert!(!mutual.recognizes("aa"));
    let unproductive = Cluster::<Unproductive>::build();
    assert!(!unproductive.recognizes(""));
    assert!(!unproductive.recognizes("a"));
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
        assert!(cluster.recognizes(input), "{input:?}");
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
        assert!(!cluster.recognizes(input), "{input:?}");
    }
}
