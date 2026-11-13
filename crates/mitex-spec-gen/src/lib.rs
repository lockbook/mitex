//! Provides embedded command specifications for MiTeX.

use mitex_spec::CommandSpec;

/// The default command specification.
///
/// See [Reproducing Default Command Specification][repro-default] for more
/// information.
///
/// [repro-default]: https://github.com/mitex-rs/artifacts/blob/main/README.md#default-command-specification-since-v011
pub static DEFAULT_SPEC: once_cell::sync::Lazy<CommandSpec> = once_cell::sync::Lazy::new(|| {
    CommandSpec::from_bytes(include_bytes!(concat!(
        env!("OUT_DIR"),
        "/mitex-artifacts/spec/default.rkyv"
    )))
});

/// The Typst definitions converted math calls into, by path under the
/// package's `specs/`; `mod.typ` exports them as `mitex-scope`.
pub const TYPST_SPECS: &[(&str, &str)] = &[
    (
        "mod.typ",
        include_str!("../../../packages/mitex/specs/mod.typ"),
    ),
    (
        "prelude.typ",
        include_str!("../../../packages/mitex/specs/prelude.typ"),
    ),
    (
        "latex/standard.typ",
        include_str!("../../../packages/mitex/specs/latex/standard.typ"),
    ),
];
