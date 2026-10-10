// Wire grammar adapted from autoboosh's MIT-licensed JSXBIN converter (2015).
// Rust validation and bounded parsing are original. See LICENSE-reference-MIT.
use crate::{Error, Limits, Result};
use std::collections::BTreeMap;

pub(crate) struct Reader<'a> {
    input: &'a [u8],
    pub pos: usize,
    zeros: usize,
    symbols: BTreeMap<u32, Vec<u16>>,
    pub limits: Limits,
    units: usize,
    pub v2: bool,
    pub version: &'static str,
}
impl<'a> Reader<'a> {
    pub fn new(source: &'a str, limits: Limits) -> Result<Self> {
        let mut r = Self { input: source.as_bytes(), pos: 0, zeros: 0, symbols: BTreeMap::new(), limits, units: 0, v2: true, version: "2.0" };
        if source.len() > limits.input_bytes {
            return r.fail("input size limit exceeded");
        }
        let start = source.trim_start_matches('\u{feff}').trim_start();
        r.pos = source.len() - start.len();
        if start.starts_with("@JSXBIN@ES@2.1@") {
            r.v2 = true;
            r.version = "2.1";
        } else if start.starts_with("@JSXBIN@ES@2.0@") {
            r.v2 = true;
        } else if start.starts_with("@JSXBIN@ES@1.0@") {
            r.v2 = false;
            r.version = "1.0";
        } else {
            let version = start
                .strip_prefix("@JSXBIN@ES@")
                .and_then(|s| s.split('@').next())
                .filter(|v| v.len() < 12 && v.chars().all(|c| c.is_ascii_digit() || c == '.'));
            return r.fail(if let Some(v) = version {
                format!("unsupported JSXBIN version {v}; supported: 1.0, 2.0, 2.1")
            } else {
                "expected a JSXBIN ES 1.0, 2.0 or 2.1 header".into()
            });
        }
        r.pos += 15;
        Ok(r)
    }
    pub fn err(&self, message: impl Into<String>) -> Error {
        Error { offset: self.pos, message: message.into() }
    }
    pub fn fail<T>(&self, message: impl Into<String>) -> Result<T> {
        Err(self.err(message))
    }
    pub fn peek(&mut self) -> Result<u8> {
        while self.input.get(self.pos).is_some_and(|c| c.is_ascii_whitespace()) {
            self.pos += 1;
        }
        // Line continuations are accepted only as a backslash followed by a newline.
        if self.input.get(self.pos) == Some(&b'\\') && self.input.get(self.pos + 1).is_some_and(|c| *c == b'\r' || *c == b'\n') {
            self.pos += 1;
            while self.input.get(self.pos).is_some_and(|c| c.is_ascii_whitespace()) {
                self.pos += 1;
            }
        }
        self.input.get(self.pos).copied().ok_or_else(|| self.err("unexpected end of input"))
    }
    pub fn take(&mut self) -> Result<u8> {
        let c = self.peek()?;
        self.pos += 1;
        Ok(c)
    }
    pub fn finish(&mut self) -> Result<()> {
        if self.zeros != 0 {
            return self.fail("unconsumed zero run");
        }
        while self.input.get(self.pos).is_some_and(|c| c.is_ascii_whitespace()) {
            self.pos += 1;
        }
        if self.pos != self.input.len() {
            return self.fail("trailing encoded data");
        }
        Ok(())
    }
    fn zero(&mut self) -> Result<bool> {
        if self.zeros > 0 {
            self.zeros -= 1;
            return Ok(true);
        }
        let c = self.peek()?;
        if c == b'A' {
            self.take()?;
            return Ok(true);
        }
        if c != b'0' {
            return Ok(false);
        }
        let mut count = 0usize;
        loop {
            if self.peek()? == b'A' {
                self.take()?;
                count += 1;
                break;
            }
            if self.peek()? != b'0' {
                break;
            }
            self.take()?;
            let run = self.take()?;
            if !(b'A'..=b'z').contains(&run) {
                return self.fail("invalid zero run");
            }
            let n = usize::from(run - b'?');
            count = count.checked_add(n).ok_or_else(|| self.err("zero run overflow"))?;
            if count > self.limits.output_bytes {
                return self.fail("zero run limit exceeded");
            }
            if n <= 27 {
                break;
            }
        }
        if count == 0 {
            return self.fail("empty zero run");
        }
        self.zeros = count - 1;
        Ok(true)
    }
    pub fn byte(&mut self) -> Result<u8> {
        if self.zero()? {
            return Ok(0);
        }
        let first = self.take()?;
        match first {
            b'A'..=b'Z' => Ok(first - b'A'),
            b'g'..=b'n' => {
                let low = self.take()?;
                let n = match low {
                    b'A'..=b'Z' => low - b'A',
                    b'a'..=b'f' => low - b'a' + 26,
                    _ => return self.fail("invalid byte digit"),
                };
                Ok((first - b'g') * 32 + n)
            }
            _ => self.fail("invalid byte prefix"),
        }
    }
    pub fn int(&mut self) -> Result<i64> {
        if self.zero()? {
            return Ok(0);
        }
        let negative = self.peek()? == b'y';
        if negative {
            self.take()?;
        }
        let size = match self.peek()? {
            b'2' => {
                self.take()?;
                2
            }
            b'4' => {
                self.take()?;
                4
            }
            _ => 1,
        };
        let mut value = 0i64;
        for i in 0..size {
            value |= i64::from(self.byte()?) << (i * 8);
        }
        Ok(if negative { -value } else { value })
    }
    pub fn count(&mut self) -> Result<usize> {
        let n = self.int()?.unsigned_abs();
        let n = usize::try_from(n).map_err(|_| self.err("count overflow"))?;
        if n > self.limits.nodes {
            return self.fail("collection limit exceeded");
        }
        Ok(n)
    }
    pub fn number(&mut self) -> Result<String> {
        if self.zeros == 0 && self.peek()? == b'8' {
            self.take()?;
            let mut bytes = [0u8; 8];
            for b in &mut bytes {
                *b = self.byte()?;
            }
            let f = f64::from_le_bytes(bytes);
            Ok(if f.is_nan() {
                "(0/0)".into()
            } else if f == f64::INFINITY {
                "(1/0)".into()
            } else if f == f64::NEG_INFINITY {
                "(-1/0)".into()
            } else {
                f.to_string()
            })
        } else {
            let n = self.int()?;
            Ok(if n > i64::from(i32::MAX) { n - (1i64 << 32) } else { n }.to_string())
        }
    }
    pub fn boolean(&mut self) -> Result<bool> {
        match self.take()? {
            b't' => Ok(true),
            b'f' => Ok(false),
            _ => self.fail("expected boolean"),
        }
    }
    pub fn string(&mut self) -> Result<Vec<u16>> {
        let n = self.int()?;
        let n = usize::try_from(n).map_err(|_| self.err("negative string length"))?;
        self.units = self.units.checked_add(n).ok_or_else(|| self.err("string size overflow"))?;
        if self.units > self.limits.string_units {
            return self.fail("string budget exceeded");
        }
        let mut s = Vec::new();
        for _ in 0..n {
            let c = self.int()?;
            if let Ok(unit) = u16::try_from(c) {
                s.push(unit);
            } else if let Some(c) = u32::try_from(c).ok().and_then(char::from_u32) {
                s.extend_from_slice(c.encode_utf16(&mut [0; 2]));
            } else {
                return self.fail("invalid string code point");
            }
        }
        Ok(s)
    }
    pub fn id(&mut self) -> Result<Vec<u16>> {
        if self.peek()? == b'z' {
            self.take()?;
            let s = self.string()?;
            let id = u32::try_from(self.int()?).map_err(|_| self.err("invalid symbol id"))?;
            if self.symbols.len() >= self.limits.nodes {
                return self.fail("symbol budget exceeded");
            }
            self.symbols.insert(id, s.clone());
            Ok(s)
        } else {
            let id = u32::try_from(self.int()?).map_err(|_| self.err("invalid symbol reference"))?;
            self.symbols.get(&id).cloned().ok_or_else(|| self.err(format!("unknown symbol {id}")))
        }
    }
    pub fn reference(&mut self) -> Result<(Vec<u16>, bool)> {
        let s = self.id()?;
        let flag = self.v2 && self.boolean()?;
        Ok((s, flag))
    }
    pub fn variant(&mut self) -> Result<Option<String>> {
        Ok(match self.take()? {
            b'n' => None,
            b'a' => Some("(void 0)".into()),
            b'b' => Some("null".into()),
            b'c' => Some(self.boolean()?.to_string()),
            b'd' => Some(self.number()?),
            b'e' => Some(quote(&self.string()?)),
            _ => return self.fail("unknown literal type"),
        })
    }
}

pub(crate) fn quote(s: &[u16]) -> String {
    let mut out = String::from("\"");
    for &c in s {
        match c {
            32..=126 if c != 34 && c != 92 => out.push(char::from(c as u8)),
            34 => out.push_str("\\\""),
            92 => out.push_str("\\\\"),
            _ => out.push_str(&format!("\\u{c:04x}")),
        }
    }
    out.push('"');
    out
}
pub(crate) fn identifier(s: &[u16]) -> Result<String> {
    let mut out = String::new();
    for (i, &c) in s.iter().enumerate() {
        if c < 128 {
            let b = c as u8;
            if b == b'$' || b == b'_' || b.is_ascii_alphabetic() || (i > 0 && b.is_ascii_digit()) {
                out.push(char::from(b));
            } else {
                return Err(Error { offset: 0, message: format!("invalid identifier character U+{c:04X} at unit {i}") });
            }
        } else {
            out.push_str(&format!("\\u{c:04x}"));
        }
    }
    Ok(out)
}
