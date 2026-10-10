# Expressions and scripting parity

> **Last reviewed:** 2026-10-10 · **Last updated:** 2026-10-11 · **Change:** major (first expressions and scripting checklist against After Effects 2026 26.5) · **Target:** Adobe After Effects 2026

Expressions and ExtendScript are how After Effects work is automated and how much of it is
built: rigs, templates, studio pipelines. They are also what carries over from After Effects
today, while `.aep` projects can't. Architecture: [architecture.md](architecture.md) §5 and §5a;
extension points: [plugins.md](plugins.md); overall numbers:
[target-app-parity.md](target-app-parity.md).

**Breadth ≈ 93%, ready ≈ 65%** (estimated). The object models are written from Adobe's public
Expression Language Reference and Scripting Guide (behaviour only, AGENTS.md §2) and tested with
our own scripts. The local fork also probes the user’s installed scripts in disposable sessions; these are not paired AE/EffectCraft workflow-equivalence tests. The local JSXBIN fork adds MIT-referenced format code under the contributor’s explicit policy exception; see `crates/jsxbin/README.md`.

| Feature | After Effects 2026 | EffectCraft | Status | Evidence / gap |
|---|---|---|---|---|
| Expression language | JavaScript engine (default) and Legacy ExtendScript | JavaScript (boa) with legacy array maths (`value + [10, 0]`) | good | `crates/expr` tests; no separate Legacy ExtendScript engine mode |
| Expression object model | `thisComp`, `thisLayer`, `wiggle`, `loopOut`, `valueAtTime`, `sampleImage`, `footage()`, text style API… | Same members, from the reference | good | `crates/expr/src/tests.rs`; behaviour of edge cases unmeasured (G1) |
| Expression editing | Inline field, error bar, pick whip, Expression Language menu, external editors | Inline field that grows, error bar, pick whips (property and value), language menu | partial | External editor support (#364) |
| Expression performance | Cached, multi-frame | Per-thread boa contexts | unknown | No benchmark |
| Scripting object model | `app`, `Project`, items, layers, properties, keys, eases, render queue, `TextDocument`, `Shape`, markers, `File`/`Folder`, `Socket`, `app.settings` | Same classes, edits run through undoable engine commands; match names table | good | `crates/script` tests (≈ 10 test files) |
| ScriptUI | Dialogs, palettes, dockable panels, resource strings, `onDraw` / ScriptUIGraphics | Same, dockable panels reopen at launch | good | `ui_scriptui`, `tests_ui*` |
| Scripts menu, ScriptUI Panels folder | yes | yes | same | [plugins.md](plugins.md) |
| Startup / Shutdown folders | yes | not run | missing | plugins.md |
| `#include`, `#target` directives | yes | a script starting with one doesn't parse | missing | plugins.md; common in real scripts |
| `.jsxbin` | yes | bounded loader | partial | `crates/jsxbin/README.md`: bounded decoder, eval/file/panel integration; no claim of full ExtendScript or AE host compatibility |
| Keyboard shortcuts for scripts | yes | no | missing | plugins.md |
| Command-line rendering `aerender` | yes | `effectcraft-cli render`, `script`, `exec` | same purpose | `apps/effectcraft-cli` |
| Essential Graphics scripting hooks | `addToMotionGraphicsTemplate…` | yes (M13.7) | good | — |
| Agent control (MCP, control channel, every command) | none | yes | beyond | [agents.md](agents.md), [control-protocol.md](control-protocol.md); `list_effects` omits parameter types and ranges (#441) |
| Real-world script corpus | — | local initialization probes | partial | 172 installed scripts run in isolated macOS processes; diagnostics remain local. Not an AE differential or full workflow test. |

## Estimate

**≈ 20–40 h**: `#include` / `#target` and Startup / Shutdown folders 4–8 h, script shortcuts 2–4 h,
a corpus of public scripts and expressions with the fixes it finds 10–20 h, an expression
performance benchmark 4–8 h.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-11 | major | JSXBIN loader proposal and bounded runtime adapters; provenance needs maintainer policy review |
| 2026-10-10 | major | First expressions and scripting checklist |

## JSXBIN runtime compatibility (2026-10-11)

Traced runtime fixes include panel/global scope, compiled include statement order,
actual file-write preference reporting, byte-preserving BINARY I/O, platform folder
resolution, saved script preferences, localization, toSource, legacy strings,
explicit AE command IDs, and ScriptUI listener capture/bubbling from image/text
children. Duik Bassel.2 bootstraps and opens its rigging controls in the local
probe. Arm creation reaches `Layer.applyPreset` but stops at a binary `.ffx`
preset that the JSON preset loader cannot import, after creating a temporary
layer. This is not working end-to-end rigging.

A follow-up using actual egui pointer input in a disposable composition found
additional failures after initialization: custom tabs needed mouse-down/up
listeners, collection detection needed `OMCollection`, file type checks needed
File/Folder object tags, and tool callbacks needed callable layer/property-group
shorthand. These adapters now have original regression coverage. Native pointer transitions dispatch hover, primary press/release, motion, and focus
state before activation, with coordinates and modifiers. Custom `onDraw` receives
DrawState, state images switch live, and disabled containers disable descendants.
Middle/right-button and lock-key draw states are not yet modeled. Group callback
properties are not invoked a second time after their registered click listener.

The pointer probe selects a real Opacity property before applying tools. Wiggle,
Kleaner, Blink, Swing and Looper reach the binary `.ffx`/pseudo-effect dependency;
Motion Trail also requires marker `valueAtTime`. Navigation and absence of an
uncaught error must not be reported as successful application of those tools.

An original synthetic `.ecpreset` regression confirms that native presets can
carry custom effect-group controls, preserve their identity, and drive a real
expression (Amount 38 produces Opacity 76). This is a feasible target for local
conversion, not a completed `.ffx` converter. Missing pieces are conversion of
the control schema with stable names/order, reusable pseudo-effect registration
for `canAddProperty`/`addProperty`, and a mapping from script preset requests to
the native representation. Additional host/expression API gaps remain separate.

Remaining independently tracked areas include ScriptUI treeview, native external
libraries, cross-application BridgeTalk transport, system-command dependencies,
pseudo-effects, and full source E4X. Scripts that catch an error and show a panel
must not be counted as fully working. MoBar helper files require the real parent
framework; inventing its globals would hide rather than solve that dependency.
