use super::prelude::*;

#[test]
fn test_fuzzing() {
    assert_snapshot!(convert_math(r#"\left\0"#).unwrap_err(), @r###"error: unknown command: \0"###);
    assert_snapshot!(convert_math(r#"\end{}"#).unwrap_err(), @r###"error: error unexpected: """###);
}

/// Runs `f` on a thread with a fixed stack, so overflowing it doesn't depend on
/// the test runner
fn on_fixed_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(8 << 20)
        .spawn(f)
        .unwrap()
        .join()
        .unwrap()
}

fn nest(open: &str, close: &str, depth: usize) -> String {
    format!("{}x{}", open.repeat(depth), close.repeat(depth))
}

#[test]
fn test_fuzzing_deep_nesting() {
    let shapes = [
        ("{", "}"),
        ("\\frac{", "}{y}"),
        ("\\left(", "\\right)"),
        ("\\begin{matrix}", "\\end{matrix}"),
        ("", "'"),
    ];
    let (deep, shallow, deep_scripts, deep_text) = on_fixed_stack(move || {
        let convert = |depth| shapes.map(|(open, close)| convert_math(&nest(open, close, depth)));
        (
            convert(20_000),
            convert(200),
            convert_math(&nest("x^{", "}", 20_000)),
            convert_text(&nest("\\textbf{", "}", 20_000)),
        )
    });
    let [braces, frac, left_right, env, primes] = deep.map(Result::unwrap_err);
    assert_snapshot!(braces, @r###"error: error unexpected: "{""###);
    assert_snapshot!(frac, @r###"error: error unexpected: "\\frac""###);
    assert_snapshot!(left_right, @r###"error: error unexpected: "\\left""###);
    assert_snapshot!(env, @r###"error: error unexpected: "matrix""###);
    assert_snapshot!(primes, @r###"error: error unexpected: "'""###);
    assert_snapshot!(deep_scripts.unwrap_err(), @r###"error: error unexpected: "x""###);
    assert_snapshot!(deep_text.unwrap_err(), @r###"error: error unexpected: "\\textbf""###);
    for converted in shallow {
        assert!(converted.is_ok(), "{converted:?}");
    }
}
