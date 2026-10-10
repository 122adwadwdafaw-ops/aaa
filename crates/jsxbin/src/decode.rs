// JSXBIN wire-node layouts adapted from autoboosh (MIT, 2015).
// The validated Rust reader/emitter is maintained by this fork.
use crate::reader::{Reader, identifier, quote};
use crate::{Decoded, Limits, Result};

#[derive(Default)]
struct Node {
    text: String,
    line: i64,
    kind: u8,
    list: Vec<Option<Node>>,
    binary: Option<(String, String)>,
    declaration: Option<String>,
    member: Option<(String, String)>,
}
impl Node {
    fn stmt(&self) -> String {
        if let Some(name) = &self.declaration {
            return format!("var {name} = {};\n", self.text);
        }
        if matches!(self.kind, b'J' | b'H' | b'f' | b'I') { format!("{};\n", self.text) } else { format!("{}\n", self.text) }
    }
}
struct Line {
    line: i64,
    body: String,
    labels: String,
}
struct Decoder<'a> {
    r: Reader<'a>,
    nodes: usize,
    depth: usize,
    emitted: usize,
    constants: Vec<String>,
    path: String,
    internal_names: std::collections::BTreeMap<Vec<u16>, String>,
    seen_names: std::collections::BTreeSet<String>,
}

pub(crate) fn decode(source: &str, limits: Limits) -> Result<Decoded> {
    let r = Reader::new(source, limits)?;
    let version = r.version;
    let mut d = Decoder {
        r,
        nodes: 0,
        depth: 0,
        emitted: 0,
        constants: Vec::new(),
        path: String::new(),
        internal_names: Default::default(),
        seen_names: Default::default(),
    };
    let root = d.required()?;
    if root.kind != b'M' {
        return d.r.fail("root must be a script function");
    }
    d.r.finish()?;
    Ok(Decoded { source: root.text, version })
}
impl Decoder<'_> {
    fn id(&mut self) -> Result<String> {
        let raw = self.r.id()?;
        self.name(&raw)
    }
    fn name(&mut self, raw: &[u16]) -> Result<String> {
        if let Some(name) = self.internal_names.get(raw) {
            return Ok(name.clone());
        }
        match identifier(raw) {
            Ok(name) => {
                if self.internal_names.values().any(|n| *n == name) {
                    return self.r.fail("compiler temporary collides with a source identifier");
                }
                self.seen_names.insert(name.clone());
                Ok(name)
            }
            Err(_) if raw.first() == Some(&u16::from(b'|')) => {
                let mut serial = self.internal_names.len();
                let name = loop {
                    let name = format!("__jsxbin_internal_{serial}");
                    if !self.seen_names.contains(&name) {
                        break name;
                    }
                    serial += 1;
                };
                self.seen_names.insert(name.clone());
                self.internal_names.insert(raw.to_vec(), name.clone());
                Ok(name)
            }
            Err(e) => self.r.fail(format!("nodes {}: {}", self.path, e.message)),
        }
    }
    fn op(&mut self) -> Result<String> {
        let s = self.r.id()?;
        let s = String::from_utf16(&s).map_err(|_| self.r.err("invalid operator"))?;
        if [
            "+",
            "-",
            "*",
            "/",
            "%",
            "==",
            "!=",
            "===",
            "!==",
            "<",
            ">",
            "<=",
            ">=",
            "<<",
            ">>",
            ">>>",
            "&",
            "|",
            "^",
            "&&",
            "||",
            ",",
            "!",
            "~",
            "typeof",
            "void",
            "delete",
            "in",
            "instanceof",
        ]
        .contains(&s.as_str())
        {
            Ok(s)
        } else {
            self.r.fail(format!("unknown operator {s:?}"))
        }
    }
    fn required(&mut self) -> Result<Node> {
        self.node()?.ok_or_else(|| self.r.err("missing required node"))
    }
    fn expr(&mut self) -> Result<String> {
        Ok(self.required()?.text)
    }
    fn optional(&mut self) -> Result<String> {
        Ok(self.node()?.map_or_else(String::new, |n| n.text))
    }
    fn children(&mut self) -> Result<Vec<Option<Node>>> {
        let count = self.r.count()?;
        let mut out = Vec::new();
        for _ in 0..count {
            out.push(self.node()?);
        }
        Ok(out)
    }
    fn line(&mut self) -> Result<Line> {
        let line = self.r.int()?.abs();
        let child = self.node()?;
        let count = self.r.count()?;
        let mut labels = String::new();
        for _ in 0..count {
            labels.push_str(&self.id()?);
            labels.push(':');
        }
        Ok(Line { line, body: child.map_or_else(String::new, |n| n.stmt()), labels })
    }
    fn embedded(&self, n: Node) -> Result<String> {
        if n.kind == b'b' {
            let mut values = Vec::new();
            for child in n.list.into_iter().flatten() {
                values.push(self.embedded(child)?);
            }
            if values.is_empty() {
                return self.r.fail("empty XML interpolation");
            }
            Ok(format!("({})", values.join(",")))
        } else if n.kind == b'J' || !n.text.contains(';') {
            Ok(n.text)
        } else {
            self.r.fail("unsupported XML interpolation expression")
        }
    }
    fn operand(&self, node: Option<Node>, literal: Option<String>) -> Result<Option<String>> {
        match (node, literal) {
            (Some(n), None) => Ok(Some(n.text)),
            (None, l) => Ok(l),
            _ => self.r.fail("both node and literal supplied for operand"),
        }
    }
    fn budget(&mut self, n: Node) -> Result<Node> {
        self.emitted = self.emitted.checked_add(n.text.len()).ok_or_else(|| self.r.err("output size overflow"))?;
        // Bound cumulative intermediate strings as well as the final source.
        if n.text.len() > self.r.limits.output_bytes || self.emitted > self.r.limits.output_bytes.saturating_mul(8) {
            return self.r.fail("decoded output budget exceeded");
        }
        Ok(n)
    }
    fn node(&mut self) -> Result<Option<Node>> {
        self.nodes += 1;
        if self.nodes > self.r.limits.nodes {
            return self.r.fail("node limit exceeded");
        }
        if self.depth >= self.r.limits.depth {
            return self.r.fail("nesting limit exceeded");
        }
        let kind = self.r.take()?;
        if kind == b'n' {
            return Ok(None);
        }
        self.depth += 1;
        self.path.push(char::from(kind));
        let result = self.body(kind);
        self.depth -= 1;
        self.path.pop();
        let mut node = result?;
        node.kind = kind;
        Ok(Some(self.budget(node)?))
    }
    fn body(&mut self, kind: u8) -> Result<Node> {
        let mut n = Node::default();
        match kind {
            b'F' => n.text = self.r.variant()?.ok_or_else(|| self.r.err("missing literal"))?,
            b'j' | b'e' | b'V' => {
                let (s, _) = self.r.reference()?;
                n.text =
                    if s.first() == Some(&u16::from(b'@')) { format!("(this).attribute({})", quote(s.get(1..).unwrap_or_default())) } else { self.name(&s)? };
                if kind == b'V' {
                    self.r.int()?;
                }
            }
            b'R' => {
                n.list = self.children()?;
                self.r.boolean()?;
                // Function signatures already declare the locals. Declaration nodes
                // in a comma sequence are initializer assignments, not nested `var`s.
                let values: Vec<&str> = n.list.iter().map(|x| x.as_ref().map_or("", |n| n.text.strip_prefix("var ").unwrap_or(&n.text))).collect();
                n.text = format!("({})", values.join(","));
            }
            b'A' => {
                let args = self.node()?;
                n.text = if let Some(a) = args {
                    if a.kind != b'R' {
                        return self.r.fail("array requires argument list");
                    }
                    let trailing = if a.list.last().is_some_and(Option::is_none) { "," } else { "" };
                    format!("[{}{trailing}]", a.list.iter().map(|x| x.as_ref().map_or("", |n| n.text.as_str())).collect::<Vec<_>>().join(","))
                } else {
                    "[]".into()
                };
            }
            b'C' | b'U' => {
                let op = self.op()?;
                let left = self.node()?;
                let right = self.node()?;
                let lv = self.r.variant()?;
                let rv = self.r.variant()?;
                let left = self.operand(left, lv)?;
                let right = self.operand(right, rv)?;
                n.text = match (left, right) {
                    (Some(l), Some(r)) => {
                        if op == "+" {
                            format!("__jsxbinAdd({l},{r})")
                        } else {
                            format!("({l} {op} {r})")
                        }
                    }
                    (Some(v), None) | (None, Some(v)) if kind == b'C' => {
                        n.binary = Some((op, v));
                        String::new()
                    }
                    _ => return self.r.fail("missing binary operand"),
                };
            }
            b'S' | b'G' | b'B' => {
                let mut member = None;
                let target = if kind == b'B' {
                    let target = self.required()?;
                    member = target.member;
                    target.text
                } else {
                    let id = self.id()?;
                    self.r.int()?;
                    id
                };
                let expr = self.node()?;
                let literal = self.r.variant()?;
                let compound = self.r.boolean()?;
                let declare = if kind == b'B' { false } else { self.r.boolean()? };
                if kind == b'G' {
                    self.constants.push(target.clone());
                }
                let prefix = if kind == b'G' {
                    "const "
                } else if declare {
                    "var "
                } else {
                    ""
                };
                if compound {
                    let (op, rhs) = expr.and_then(|n| n.binary).ok_or_else(|| self.r.err("invalid compound assignment"))?;
                    if !["+", "-", "*", "/", "%", "<<", ">>", ">>>", "&", "|", "^"].contains(&op.as_str()) {
                        return self.r.fail("invalid assignment operator");
                    }
                    let rhs = literal.unwrap_or(rhs);
                    n.text = if op == "+" {
                        if let Some((obj, key)) = member {
                            // Read the old property before evaluating RHS; evaluate the
                            // receiver/key only once (including getters and i++ keys).
                            format!("(function(o,k){{var old=o[k];return function(v){{return o[k]=__jsxbinAdd(old,v);}};}})({obj},{key})({rhs})")
                        } else {
                            format!("{target}=__jsxbinAdd({target},{rhs})")
                        }
                    } else {
                        format!("{prefix}{target} {op}= {rhs}")
                    };
                } else {
                    let rhs = self.operand(expr, literal)?;
                    n.text = if let Some(rhs) = rhs {
                        format!("{prefix}{target} = {rhs}")
                    } else if declare {
                        format!("{prefix}{target}")
                    } else {
                        return self.r.fail("missing assignment value");
                    };
                }
                if prefix.is_empty() {
                    n.text = format!("({})", n.text);
                }
            }
            b'X' => {
                let (key, attribute) = self.r.reference()?;
                let obj = self.optional()?;
                let key = if attribute { format!("\"@\"+{}", quote(&key)) } else { quote(&key) };
                n.text = format!("({obj})[{key}]");
                n.member = Some((obj, key));
            }
            b'Q' => {
                let (_, attribute) = self.r.reference()?;
                let obj = self.expr()?;
                let index = self.expr()?;
                let index = if attribute { format!("\"@\"+({index})") } else { index };
                n.text = format!("({obj})[{index}]");
                n.member = Some((obj, index));
            }
            b'W' => {
                self.r.id()?;
                let count = self.r.count()?;
                let mut props = Vec::new();
                for _ in 0..count {
                    let key = self.r.id()?;
                    let value = self.expr()?;
                    props.push(format!("[{}]:{value}", quote(&key)));
                }
                n.text = format!("({{{}}})", props.join(","));
            }
            b'E' => {
                let callee = self.expr()?;
                let args = self.node()?;
                if args.as_ref().is_some_and(|n| n.kind != b'R') {
                    return self.r.fail("call requires argument list");
                }
                let args = args.map_or_else(String::new, |n| n.list.iter().map(|x| x.as_ref().map_or("", |n| n.text.as_str())).collect::<Vec<_>>().join(","));
                let constructor = self.r.boolean()?;
                n.text = if constructor { format!("(new ({callee})({args}))") } else { format!("({callee})({args})") };
            }
            b'h' | b'i' => {
                let op = self.op()?;
                let expr = self.required()?;
                n.text = if op == "typeof" && expr.kind != b'j' { format!("__jsxbinTypeof({})", expr.text) } else { format!("({op} {})", expr.text) };
            }
            b'T' | b'P' => {
                let target = if kind == b'T' {
                    let id = self.id()?;
                    self.r.int()?;
                    id
                } else {
                    self.expr()?
                };
                let direction = self.r.number()?;
                let op = match direction.as_str() {
                    "1" => "++",
                    "-1" => "--",
                    _ => return self.r.fail("invalid increment"),
                };
                let post = self.r.boolean()?;
                n.text = if post { format!("({target}{op})") } else { format!("({op}{target})") };
            }
            b'd' => {
                let c = self.expr()?;
                let t = self.expr()?;
                let f = self.expr()?;
                n.text = format!("({c}?{t}:{f})");
            }
            b'Y' => {
                let pattern = quote(&self.r.string()?);
                let flags = quote(&self.r.string()?);
                n.text = format!("(new __jsxbinRegExp({pattern},{flags}))");
            }
            b'M' => {
                let scope = self.constants.len();
                let line = self.line()?;
                n.line = line.line;
                let count = self.r.count()?;
                let mut params = Vec::new();
                let mut locals = Vec::new();
                for _ in 0..count {
                    let raw = self.r.id()?;
                    let index = self.r.int()?.abs();
                    if (0x2000_0000..0x3000_0000).contains(&index) {
                        params.push((index, self.name(&raw)?));
                    } else if (0x4000_0000..0x5000_0000).contains(&index) {
                        locals.push(self.name(&raw)?);
                    }
                }
                self.r.int()?;
                self.r.int()?;
                self.r.int()?;
                let raw_name = self.r.id()?;
                let header = self.r.int()?;
                // A script wrapper's diagnostic name need not be a JS identifier.
                let name = if header == 1 { String::new() } else { self.name(&raw_name)? };
                self.r.int()?;
                params.sort_by_key(|p| p.0);
                let params = params.into_iter().map(|p| p.1).collect::<Vec<_>>().join(",");
                let constants = self.constants.split_off(scope);
                locals.retain(|n| !constants.contains(n));
                let locals = if locals.is_empty() { String::new() } else { format!("var {};\n", locals.join(",")) };
                if header != 1 && !name.is_empty() {
                    n.declaration = Some(name.clone());
                }
                n.text = if header == 1 { format!("{locals}{}", line.body) } else { format!("function {name}({params}){{\n{locals}{}}}", line.body) };
            }
            b'N' => {
                let line = self.line()?;
                n.line = line.line;
                n.text = format!("({})", self.expr()?);
            }
            b'b' => {
                let line = self.line()?;
                n.line = line.line;
                let mut children = self.children()?;
                children.extend(self.children()?);
                // Line metadata is diagnostic, not execution order: #include
                // can restart it. Keep wire order, only hoisting declarations.
                children.sort_by_key(|n| n.as_ref().is_none_or(|n| n.declaration.is_none()));
                let body = children.iter().flatten().map(Node::stmt).collect::<String>();
                n.list = children;
                n.text = if line.labels.is_empty() { body } else { format!("{}{{\n{body}}}", line.labels) };
            }
            b'J' | b'H' | b'Z' | b'f' | b'D' => {
                let line = self.line()?;
                n.line = line.line;
                let body = match kind {
                    b'J' => self.optional()?,
                    b'H' => "debugger".into(),
                    b'Z' => format!("return {};", self.optional()?),
                    b'f' => format!("throw {}", self.expr()?),
                    b'D' => {
                        let id = self.id()?;
                        let b = self.r.boolean()?;
                        format!("{} {id};", if b { "break" } else { "continue" })
                    }
                    _ => String::new(),
                };
                n.text = format!("{}{body}", line.labels);
            }
            b'K' | b'a' | b'L' | b'l' | b'I' | b'O' | b'm' => {
                let line = self.line()?;
                n.line = line.line;
                let body = format!("{{\n{}}}", line.body);
                let control = match kind {
                    b'K' => {
                        let init = self.optional()?;
                        let test = self.optional()?;
                        let update = self.optional()?;
                        format!("for({init};{test};{update}){body}")
                    }
                    b'a' => {
                        let var = self.expr()?;
                        let start = self.r.number()?;
                        let end = self.expr()?;
                        let step = self.r.number()?;
                        self.r.int()?;
                        let op = self.op()?;
                        format!("for({var}={start};{var}{op}{end};{var}+={step}){body}")
                    }
                    b'L' => {
                        let var = self.expr()?;
                        let obj = self.expr()?;
                        self.r.int()?;
                        self.r.id()?;
                        let each = self.r.v2 && self.r.boolean()?;
                        if each {
                            format!("for({var} of (function*(o){{for(var k in o)yield o[k];}})({obj})){body}")
                        } else {
                            format!("for({var} in {obj}){body}")
                        }
                    }
                    b'l' => {
                        let test = self.node()?.map_or_else(|| "true".into(), |n| n.text);
                        format!("while({test}){body}")
                    }
                    b'I' => {
                        let test = self.expr()?;
                        format!("do{body}while({test})")
                    }
                    b'O' => {
                        let test = self.expr()?;
                        let tail = self.node()?;
                        let mut s = format!("if({test}){body}");
                        if let Some(t) = tail {
                            s.push_str(&format!("else{{\n{}}}", t.stmt()));
                        }
                        s
                    }
                    b'm' => {
                        let obj = self.expr()?;
                        format!("with({obj}){body}")
                    }
                    _ => String::new(),
                };
                n.text = format!("{}{control}", line.labels);
            }
            b'c' => {
                let line = self.line()?;
                n.line = line.line;
                let test = self.expr()?;
                let cases = self.children()?;
                let bodies = self.children()?;
                if cases.len() != bodies.len() {
                    return self.r.fail("switch cases/bodies length mismatch");
                }
                let mut out = format!("{}switch({test}){{\n", line.labels);
                for (case, body) in cases.into_iter().zip(bodies) {
                    let case = case.ok_or_else(|| self.r.err("missing case list"))?;
                    if case.kind != b'R' {
                        return self.r.fail("invalid case list");
                    }
                    if case.list.is_empty() {
                        out.push_str("default:\n");
                    }
                    for c in case.list {
                        if let Some(c) = c {
                            out.push_str(&format!("case {}:\n", c.text));
                        } else {
                            out.push_str("default:\n");
                        }
                    }
                    if let Some(b) = body {
                        out.push_str(&b.stmt());
                    }
                }
                out.push('}');
                n.text = out;
            }
            b'g' => {
                let line = self.line()?;
                n.line = line.line;
                let count = self.r.count()?;
                let finally = self.node()?;
                let mut out = format!("{}try{{\n{}}}", line.labels, line.body);
                let mut catches = Vec::new();
                for _ in 0..count {
                    catches.push((self.id()?, self.node()?, self.node()?));
                }
                let mut tail: Option<(String, String)> = None;
                for (name, filter, body) in catches.into_iter().rev() {
                    let body = body.map_or_else(String::new, |n| n.stmt());
                    let next = if let Some((next_name, next_body)) = tail {
                        format!("try{{throw {name};}}catch({next_name}){{{next_body}}}")
                    } else {
                        format!("throw {name};")
                    };
                    let body = if let Some(filter) = filter { format!("if({}){{{body}}}else{{{next}}}", filter.text) } else { body };
                    tail = Some((name, body));
                }
                if let Some((name, body)) = tail {
                    out.push_str(&format!("catch({name}){{\n{body}}}"));
                }
                if let Some(f) = finally {
                    out.push_str(&format!("finally{{\n{}}}", f.stmt()));
                }
                n.text = out;
            }
            b'o' => {
                let count = self.r.count()?;
                let mut parts = Vec::new();
                for _ in 0..count {
                    let expr = self.required()?;
                    let expr = self.embedded(expr)?;
                    let mode = self.r.int()?;
                    if !(0..=3).contains(&mode) {
                        return self.r.fail("unknown XML interpolation mode");
                    }
                    parts.push(format!("__jsxbinXmlPart({expr},{mode})"));
                }
                n.text = if parts.is_empty() { "\"\"".into() } else { format!("__jsxbinXml({})", parts.join("+")) };
            }
            b'k' => {
                n.text = self.expr()?;
            }
            b'p' | b'q' => {
                let (name, attribute) = self.r.reference()?;
                let obj = self.optional()?;
                let second = self.optional()?;
                let third = self.optional()?;
                let key = if kind == b'p' { quote(&self.r.id()?) } else { quote(&name) };
                let ns = if kind == b'p' {
                    if name == [u16::from(b'*')] { "null".into() } else { identifier(&name).map_err(|e| self.r.err(e.message))? }
                } else {
                    "null".into()
                };
                n.text = format!(
                    "__jsxbinXmlAccess({}, {}, {}, {}, {}, {}, {ns})",
                    if obj.is_empty() { "null" } else { &obj },
                    key,
                    if second.is_empty() { "null" } else { &second },
                    if third.is_empty() { "null" } else { &third },
                    attribute,
                    kind == b'q'
                );
            }
            b'r' => {
                let (name, attribute) = self.r.reference()?;
                let obj = self.optional()?;
                let member = self.expr()?;
                n.text = format!(
                    "__jsxbinXmlFilter({},function(){{with(this){{return ({member});}}}}, {}, {})",
                    if obj.is_empty() { "null" } else { &obj },
                    quote(&name),
                    attribute
                );
            }
            b's' => {
                let op = self.r.id()?;
                let expr = self.expr()?;
                n.text = format!("__jsxbinXmlUnary({}, {expr})", quote(&op));
            }
            _ => return self.r.fail(format!("unknown node marker '{}'", char::from(kind))),
        }
        Ok(n)
    }
}
