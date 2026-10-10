# Boa compatibility patch

Vendored from crates.io `boa_engine` 0.22.0 (MIT OR Unlicense).
The original crate archive has SHA-256
`5d6aaf7fdc4299df68adb67e1a5440a51d34630a1f5271b9698fd74bc97b60d9`
and identifies upstream commit `337a3668a0dc86dd401ea20906e782249a64a228`
(`core/engine`). License texts are included from the upstream
[`v0.22` release](https://github.com/boa-dev/boa/tree/v0.22).
Local changes:

- `src/context/hooks.rs`, `src/builtins/eval/mod.rs`, and
  `src/builtins/function/mod.rs`: a default-identity `prepare_script_source` hook
  before intrinsic eval and Function-body parsing. EffectCraft decodes JSXBIN,
  adapts a narrow XML-selector expression subset, and applies parser-depth guards.
  Direct eval retains lexical scope; indirect eval remains global.
- `src/script.rs` and `src/builtins/eval/mod.rs`: an explicit script `this` value
  for docked panels without a function wrapper. Global declarations stay global,
  direct eval inherits panel `this`, ordinary function/indirect-eval calls do not.
- `src/vm/opcode/get/property.rs`: name the failed property on null/undefined
  receivers; optional `EFFECTCRAFT_SCRIPT_TRACE_PROPERTIES` logs missing names.

- `src/vm/mod.rs`: optional `EFFECTCRAFT_SCRIPT_TRACE_EXCEPTIONS` emits runtime
  exceptions before catch handlers, without evaluating JavaScript getters.

This is a local dependency patch, not upstream approval or universal ExtendScript
compatibility. Original EffectCraft tests cover scope, eval, nested files, errors,
and compiled/source equivalence.

For review, [`effectcraft.patch`](effectcraft.patch) contains the exact six-source-file
diff against the pinned crates.io archive. Apply it with `patch -p1` inside a fresh
copy of that archive. The patch is a review aid; Cargo builds the source in this
directory. Regenerate it when updating the dependency or its modifications.
