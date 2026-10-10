# JSXBIN loader

This crate decodes ASCII JSXBIN into JavaScript for EffectCraft's scripting host.
It is L0, pure Rust, dependency-free, and performs no file access or execution.
`decode` returns source and the declared format version, or an error with the byte
offset in the original encoded input. Decoded code uses host helpers supplied by
`crates/script/src/xml.js`; it is not intended as a standalone source decompiler.

## Provenance

The wire grammar was adapted from autoboosh's MIT-licensed C# converter,
[`codecopy/jsxbin-to-jsx-converter`](https://github.com/codecopy/jsxbin-to-jsx-converter),
pinned to `a7a225f676f4cb1f9bbe9b737dba5b717fc37145`. Copyright (c) 2015 autoboosh.
The full notice is in [LICENSE-reference-MIT](LICENSE-reference-MIT).

The contributor explicitly authorized MIT decoder references for this local fork
on 2026-10-10, overriding AGENTS.md's reference-decoder restriction for this work.
This is not a claim of compliance with upstream's unchanged clean-room policy or
of maintainer approval. No copyleft decoder implementation was adapted.

The bounded Rust reader/emitter, validation, behavioral corrections, host loading
path and regression scripts are original work. The format remains undocumented
by Adobe; no official specification edition or complete conformance suite exists
in this work. Original scripts were compiled through the public ExtendScript
Debugger 2.1.0 compiler CLI as a black box; the compiler is not bundled.

## Coverage and boundaries

* ES 2.0: an original corpus covering literals (including UTF-16, signed numbers,
  negative zero), sparse arrays, properties, regular expressions, assignment,
  operator precedence, evaluation order, declarations, closures, hoisting,
  loops, labels, switch fallthrough, exceptions, guarded catch, and `with`.
* ES 2.1: the v2 node layouts, exercised with real nested compiled payloads in
  the user's local script collection. The public test corpus is predominantly
  genuine ES 2.0; header-modified version-dispatch tests are labeled as such.
* ES 1.0: version-specific references and for-in layouts from the MIT reference;
  no Adobe-generated 1.0 conformance corpus has been obtained.
* Known XML node layouts emit host-adapter calls; the host covers common XML/XMLList,
  Namespace/QName, literal interpolation, attributes, children, descendants,
  filters, namespace selection and common mutation operations. It is **not a full
  ECMA-357 / ExtendScript runtime**. Unknown node/operator variants return errors.
* Compiler temporary identifiers beginning with `|` are mapped consistently to
  legal identifiers; a collision with a later source identifier is rejected.
* Legacy block function declarations are initialized at block entry, before the
  block's other statements. Plain source still uses Boa's language semantics.

The host supplies shared panel globals, permission-aware binary File I/O, special
folders, preference persistence, localization, legacy string/toSource methods,
XML attribute expressions in eval, and ScriptUI capture/bubble listeners. Known AE
menu-command IDs map explicitly to engine commands; unknown IDs fail. BridgeTalk
currently supplies host identification only, **not** interapplication messaging.

This loader does not implement binary plug-ins/ExternalObject libraries, `.ffx`
pseudo-effects, vendor licensing services, every ScriptUI control/drawing behavior,
source preprocessor `#include`/`#target`, or persistent named engines. Source E4X
adaptation recognizes only leading XML attribute-selector expressions (including
method calls); it is not a general E4X source parser. Compiled includes preserve
wire statement order even when source line numbers restart. A decoded script can
still fail on separate runtime or engine gaps.
XML edge cases (equality/coercion, unbound/global `typeof`, namespace mutation,
parent identity after copies, and all ECMA-357 methods) need further conformance
work. Do not advertise universal JSXBIN or After Effects compatibility.

## Limits and safety

Defaults: 16 MiB encoded input, 32 MiB reconstructed source, 500,000 nodes/list
entries, 96 recursive node levels, and 16 million decoded string units. Cumulative
intermediate output has an additional budget. Strings retain UTF-16 code units;
identifiers/operators cannot inject source text. Reads are bounds-checked, zero
runs/counts are validated, and trailing or truncated input is rejected. Host-side
parser nesting and execution limits still apply, including nested eval.

The host decodes before evaluation. Its local Boa 0.22.0 dependency patch binds a
docked panel as the script's top-level `this` without isolating global declarations.
It also adds a default-identity source-preparation hook inside intrinsic eval and
Function-body parsing. Direct eval keeps its caller's lexical environment; aliases
and indirect eval retain normal global behavior. `$.evalFile` runs in the current
host, honors the existing read permission gate, restores `$.fileName`, and caps
nested file calls at 32. It does not create a recursively executing engine session.

## Tests

`cargo test -p effectcraft-jsxbin -p effectcraft-script` includes original compiled
fixtures, truncation at every byte boundary, malformed mutations, resource limits,
source/compiled comparisons, legacy values, direct/indirect eval, file permissions,
console persistence, installed scripts, docked callbacks and panel restoration.
The CLI has a genuine compiled-input regression too.

`cargo run -p effectcraft-script --example compat_probe -- SCRIPT --fixture` runs
one input in a disposable session with writes denied by its Services adapter.
`--scratch DIR --write-root ROOT` enables writes with special folders redirected
inside DIR and file writes constrained to ROOT. Use copied script resources and an
operating-system sandbox that allows writes only inside that root, denies network
and subprocess access, and applies an external timeout. Folder operations still
use host filesystem APIs, so the operating-system boundary is required.

`EFFECTCRAFT_SCRIPT_TRACE=1` emits error stacks and missing app/$/ScriptUI members;
`EFFECTCRAFT_SCRIPT_TRACE_PROPERTIES=1` additionally logs undefined property names.
`EFFECTCRAFT_SCRIPT_TRACE_EXCEPTIONS=1` traces runtime exceptions before framework
catch handlers can swallow them. Caught exceptions are not automatically fatal.
These are opt-in local diagnostics, not a public telemetry or upload feature.
The local macOS batch report is kept outside the repository. No third-party scripts, decoded
third-party implementations, vendor assets or user projects are test fixtures here.
