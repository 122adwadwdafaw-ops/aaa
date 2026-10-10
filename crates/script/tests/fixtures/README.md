# Original ScriptUI visual fixture

`button-states.jsx` is original work by @cdinhoo, licensed MIT OR Apache-2.0.
It opens an isolated palette without changing the project or preferences. Run the
same source in After Effects and EffectCraft for local visual observation.

`button-states.jsxbin` was generated from that exact source using Adobe's public
ExtendScript Debugger 2.1.0 `exportToJSXBin.js` CLI as a black box (ES 2.0 header).
The compiler is not included. The UI regression exercises both forms with real
pointer events and records normal, hover, and held-press states when
`EC_SNAPSHOT_DIR` is set. No Adobe screenshots or artwork are included.
