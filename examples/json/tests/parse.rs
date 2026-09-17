use example_json::JSON;
use xst::Cluster;

#[test]
fn parses_json_values() {
    let cluster = Cluster::<JSON>::build();
    for input in [
        "null",
        "true",
        "false",
        "0",
        "-0",
        "123",
        "-12.5e+2",
        "0.1E-9",
        r#""hello""#,
        r#""한🦀""#,
        r#""\"\\\/\b\f\n\r\t\u0041""#,
        "[]",
        "{}",
        "[null, true, false, 12, \"text\"]",
        r#"{"a": 1, "b": [null, {"c": "한🦀"}]}"#,
        " \t\r\n [ 1 , 2 ] \n",
    ] {
        assert!(cluster.parse(input), "expected valid JSON: {input:?}");
    }
}

#[test]
fn rejects_invalid_json_values() {
    let cluster = Cluster::<JSON>::build();
    for input in [
        "",
        " ",
        "nul",
        "True",
        "null true",
        "01",
        "-01",
        "+1",
        "1.",
        ".1",
        "1e",
        "1e+",
        "1 2",
        r#""unterminated"#,
        "\"bad\nstring\"",
        r#""\q""#,
        r#""\u123""#,
        "[1,]",
        "[,1]",
        "[1 2]",
        "{\"a\" 1}",
        "{\"a\": 1,}",
        "{a: 1}",
        "{\"a\":}",
        "[1",
        "{\"a\": 1",
    ] {
        assert!(!cluster.parse(input), "expected invalid JSON: {input:?}");
    }
}
