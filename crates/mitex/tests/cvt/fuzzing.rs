use super::prelude::*;

#[test]
fn test_fuzzing() {
    assert_snapshot!(convert_math(r#"\left\0"#).unwrap_err(), @r###"error: unknown command: \0"###);
    assert_snapshot!(convert_math(r#"\end{}"#).unwrap_err(), @r###"error: error unexpected: """###);
}

#[test]
fn unbraced_tabular_argument() {
    assert_snapshot!(convert_math(r"\begin{tabular} a \end{tabular}").unwrap_err(), @"error: tabular column spec must be braced");
}

#[test]
fn newcommand_naming_an_environment() {
    assert!(convert_math(r"\newcommand{\begin{matrix}_").is_ok());
}

#[test]
fn macro_ending_in_hash() {
    assert!(convert_math(r"\newcommand{\x}{a#}\x").is_ok());
}

#[test]
fn self_referencing_macros() {
    let limit = r#"error: error unexpected: "macro expansion limit""#;
    for input in [
        r"\newcommand{\x}{\x}\x",
        r"\newcommand{\x}{\y}\newcommand{\y}{\x}\x",
        r"\newcommand{\x}[1]{\x{#1#1}}\x{a}",
        r"\newenvironment{e}{\begin{e}}{\end{e}}\begin{e}\end{e}",
    ] {
        assert_eq!(convert_math(input).unwrap_err(), limit, "{input}");
    }
}

#[test]
fn long_documents_with_macros() {
    let uses = r"\R + ".repeat(600);
    let args = "x+".repeat(3000);
    for input in [
        format!(r"\newcommand{{\R}}{{\mathbb{{R}}}}{uses}"),
        format!(
            r"\newcommand{{\R}}{{\mathbb{{R}}}}\newenvironment{{thm}}{{}}{{}}\begin{{thm}}{uses}\end{{thm}}"
        ),
        format!(r"\newcommand{{\abs}}[1]{{|#1|}}\abs{{{args}}}"),
    ] {
        assert!(convert_math(&input).is_ok(), "{input}");
    }
    let text = format!(
        r"\newcommand{{\R}}{{\mathbb{{R}}}}\newenvironment{{thm}}{{}}{{}}\begin{{thm}}{}\end{{thm}}",
        r"Let $x \in \R$. ".repeat(600)
    );
    assert!(convert_text(&text).is_ok());
}
