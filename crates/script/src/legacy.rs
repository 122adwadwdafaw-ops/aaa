//! Small source-side ExtendScript adapters, in addition to JSXBIN node lowering.
//! This intentionally recognizes only a complete XML attribute selector expression.
//! Other E4X source syntax remains unsupported rather than rewriting arbitrary JS.
use std::borrow::Cow;

pub(crate) fn source(code: &str) -> Cow<'_, str> {
    let trimmed = code.trim();
    let expression = trimmed.strip_suffix(';').unwrap_or(trimmed).trim_end();
    let mut chars = expression.chars().peekable();
    let mut out = String::new();
    let ident = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '$';
    let start = |c: char| c.is_ascii_alphabetic() || c == '_' || c == '$';
    if !chars.peek().is_some_and(|&c| start(c)) {
        return Cow::Borrowed(code);
    }
    while chars.peek().is_some_and(|&c| ident(c)) {
        if let Some(c) = chars.next() {
            out.push(c);
        }
    }
    let mut adapted = false;
    loop {
        while chars.peek().is_some_and(|c| c.is_whitespace()) {
            chars.next();
        }
        if chars.peek().is_none() {
            break;
        }
        // A method on the selected XML value may have arbitrary ordinary JS
        // arguments. Leave that suffix untouched for Boa to parse normally.
        if adapted && chars.peek() == Some(&'(') {
            out.extend(chars);
            return Cow::Owned(out);
        }
        if chars.next() != Some('.') {
            return Cow::Borrowed(code);
        }
        while chars.peek().is_some_and(|c| c.is_whitespace()) {
            chars.next();
        }
        let attribute = chars.peek() == Some(&'@');
        if attribute {
            chars.next();
        }
        if !chars.peek().is_some_and(|&c| start(c)) {
            return Cow::Borrowed(code);
        }
        let mut name = String::new();
        while chars.peek().is_some_and(|&c| ident(c)) {
            if let Some(c) = chars.next() {
                name.push(c);
            }
        }
        if attribute {
            out.push_str(&format!("[\"@{name}\"]"));
            adapted = true;
        } else {
            out.push('.');
            out.push_str(&name);
        }
    }
    if adapted { Cow::Owned(out) } else { Cow::Borrowed(code) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn selectors_do_not_rewrite_strings_comments_or_general_javascript() {
        assert_eq!(super::source(" xmlData.@matchname; "), "xmlData[\"@matchname\"]");
        assert_eq!(super::source("data.child . @name"), "data.child[\"@name\"]");
        assert_eq!(super::source("data.@name.split('=')"), "data[\"@name\"].split('=')");
        for text in ["'xml.@name'", "// xml.@name", "/x.@y/", "foo.@name = x", "foo.@name;bar()", "`xml.@name`"] {
            assert!(matches!(super::source(text), std::borrow::Cow::Borrowed(_)), "{text}");
        }
    }
}
