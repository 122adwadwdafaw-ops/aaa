mod fixtures;
use effectcraft_jsxbin::{Limits, decode, decode_with_limits};

#[test]
fn original_compiler_corpus_decodes() {
    for (name, _, encoded) in fixtures::CASES {
        let d = decode(encoded).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(d.version, "2.0");
    }
}

#[test]
fn every_truncated_fixture_is_an_error() {
    for (name, _, encoded) in fixtures::CASES {
        for end in 0..encoded.trim_end().len() {
            assert!(decode(&encoded[..end]).is_err(), "{name}: truncated at {end}");
        }
    }
}

#[test]
fn malformed_mutations_never_panic() {
    for (_, _, encoded) in fixtures::CASES {
        for pos in (15..encoded.len()).step_by(7) {
            for replacement in [0, b'@', b'?', b'0', b'z', 255] {
                let mut bytes = encoded.as_bytes().to_vec();
                bytes[pos] = replacement;
                if let Ok(input) = std::str::from_utf8(&bytes) {
                    let _ = decode(input);
                }
            }
        }
    }
}

#[test]
fn limits_and_format_errors_are_actionable() {
    let src = fixtures::CASES[0].2;
    for limits in [
        Limits { input_bytes: 1, ..Limits::default() },
        Limits { nodes: 1, ..Limits::default() },
        Limits { depth: 1, ..Limits::default() },
        Limits { depth: usize::MAX, ..Limits::default() },
        Limits { output_bytes: 1, ..Limits::default() },
    ] {
        assert!(decode_with_limits(src, limits).is_err());
    }
    for src in ["", "var x=1", "@JSXBIN@ES@3.0@", "@JSXBIN@ES@2.0@n", "@JSXBIN@ES@2.0@?"] {
        assert!(decode(src).unwrap_err().to_string().contains("JSXBIN at byte"));
    }
    assert!(decode(&(src.to_string() + "garbage")).is_err());
    let d = decode(&format!("\u{feff}\n{src}")).unwrap();
    assert_eq!(d.source, decode(src).unwrap().source);
    let wrapped = src.as_bytes().chunks(24).map(|b| std::str::from_utf8(b).unwrap()).collect::<Vec<_>>().join("\\\n");
    // A line continuation may wrap the body, but not split the fixed format header.
    assert_eq!(decode(&wrapped).unwrap().source, decode(src).unwrap().source);
}

#[test]
fn compiler_internal_identifiers_are_bound_consistently() {
    let (_, _, source) = fixtures::CASES.iter().find(|c| c.0 == "extended_const_decl").unwrap();
    // Original fixture's first symbol `a` becomes an internal `|a|` name.
    // Encoded characters: a = jB, | = jc, string length 3 = D.
    let source = source.replacen("zBjB", "zDjcjBjc", 1);
    let decoded = decode(&source).unwrap().source;
    assert!(decoded.contains("const __jsxbin_internal_0 = 3"));
    assert!(decoded.contains("__jsxbin_internal_0;"));
}

#[test]
fn version_dispatch_retains_the_declared_version() {
    let (_, _, source) = fixtures::CASES.iter().find(|c| c.0 == "original-fixtures_arithmetic").unwrap();
    // These are deliberately header-modified grammar probes, not Adobe-authored
    // 1.0 / 2.1 conformance fixtures. Real 2.1 is probed locally through nested eval.
    for version in ["1.0", "2.0", "2.1"] {
        let encoded = source.replacen("@2.0@", &format!("@{version}@"), 1);
        let result = decode(&encoded).unwrap();
        assert_eq!(result.version, version);
        assert_eq!(result.source, decode(source).unwrap().source);
    }
}
