//! Small E4X value adapter for decoded ExtendScript. Parsing uses the existing
//! permissively licensed XML dependency; no external entities or file/network IO.
use boa_engine::{Context, JsNativeError, JsResult, JsString, JsValue};
use serde_json::{Value, json};

pub(crate) const PRELUDE: &str = include_str!("xml.js");

pub(crate) fn parse(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let source = args.first().cloned().unwrap_or_default().to_string(ctx)?.to_std_string_escaped();
    let ns = args.get(1).cloned().unwrap_or_default().to_string(ctx)?.to_std_string_escaped();
    let error = |s: String| JsNativeError::syntax().with_message(s);
    if source.len() > 8 * 1024 * 1024 {
        return Err(error("XML input size limit exceeded".into()).into());
    }
    let source = source.trim();
    let source = source.strip_prefix("<>").and_then(|s| s.strip_suffix("</>")).unwrap_or(source);
    let ns = ns.replace('&', "&amp;").replace('"', "&quot;").replace('<', "&lt;");
    let wrapped = format!("<jsxbin-root xmlns=\"{ns}\">{source}</jsxbin-root>");
    let options = roxmltree::ParsingOptions { allow_dtd: false, nodes_limit: 100_000, ..Default::default() };
    let doc = roxmltree::Document::parse_with_options(&wrapped, options).map_err(|e| error(format!("XML: {e}")))?;
    let mut nodes = Vec::new();
    for node in doc.root_element().children() {
        nodes.push(value(node, 0).map_err(error)?);
    }
    Ok(JsValue::from(JsString::from(json!(nodes).to_string())))
}
fn value(n: roxmltree::Node<'_, '_>, depth: usize) -> Result<Value, String> {
    if depth > 96 {
        return Err("XML nesting limit exceeded".into());
    }
    if n.is_element() {
        let tag = n.tag_name();
        let attrs: Vec<Value> = n.attributes().map(|a| json!({"kind":"attribute", "name":a.name(),"ns":a.namespace().unwrap_or(""),"prefix": a.namespace().and_then(|ns| n.lookup_prefix(ns)).unwrap_or(""),"value":a.value()})).collect();
        let mut children = Vec::new();
        for c in n.children() {
            children.push(value(c, depth + 1)?);
        }
        let namespaces: Vec<Value> = n.namespaces().map(|ns| json!({"prefix":ns.name().unwrap_or(""),"uri":ns.uri()})).collect();
        Ok(
            json!({"kind":"element", "name":tag.name(), "ns":tag.namespace().unwrap_or(""), "prefix":tag.namespace().and_then(|ns| n.lookup_prefix(ns)).unwrap_or(""),"attributes":attrs,"children":children,"namespaces":namespaces}),
        )
    } else if n.is_text() {
        Ok(json!({"kind":"text","value":n.text().unwrap_or("")}))
    } else if n.is_comment() {
        Ok(json!({"kind":"comment","value":n.text().unwrap_or("")}))
    } else if let Some(pi) = n.pi() {
        Ok(json!({"kind":"processing-instruction","name":pi.target,"value":pi.value.unwrap_or("")}))
    } else {
        Err("unsupported XML node".into())
    }
}
