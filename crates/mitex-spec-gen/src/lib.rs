//! Provides embedded command specifications for MiTeX.

use mitex_spec::{CommandSpec, JsonCommandSpec};

/// The default command specification.
///
/// See [Reproducing Default Command Specification][repro-default] for more
/// information.
///
/// [repro-default]: https://github.com/mitex-rs/artifacts/blob/main/README.md#default-command-specification-since-v011
pub static DEFAULT_SPEC: once_cell::sync::Lazy<CommandSpec> = once_cell::sync::Lazy::new(|| {
    let json: JsonCommandSpec = serde_json::from_slice(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/mitex-artifacts/spec/default.json"
    )))
    .expect("the default spec is valid JSON");
    json.into()
});

/// The Typst definitions converted math calls into, by path under the
/// package's `specs/`; `mod.typ` exports them as `mitex-scope`.
///
/// Kept under `assets/specs/` (a copy of `packages/mitex/specs/`) so this crate
/// remains self-contained when vendored or packaged in isolation.
pub const TYPST_SPECS: &[(&str, &str)] = &[
    ("mod.typ", include_str!("../assets/specs/mod.typ")),
    ("prelude.typ", include_str!("../assets/specs/prelude.typ")),
    (
        "latex/standard.typ",
        include_str!("../assets/specs/latex/standard.typ"),
    ),
];

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::TYPST_SPECS;

    #[test]
    fn vendored_typst_specs_match_package() {
        let package_specs =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packages/mitex/specs");
        if !package_specs.exists() {
            return;
        }
        for (rel_path, embedded) in TYPST_SPECS {
            let source_path = package_specs.join(rel_path);
            let source = std::fs::read_to_string(&source_path)
                .unwrap_or_else(|err| panic!("failed to read {}: {err}", source_path.display()));
            assert_eq!(
                *embedded, source,
                "crates/mitex-spec-gen/assets/specs/{rel_path} is out of sync with packages/mitex/specs/{rel_path}"
            );
        }
    }
}
