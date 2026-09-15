//! JSON writer matching Python's `json.dumps(..., ensure_ascii=False, indent=1)`
//! so that generated data files stay byte-identical to the former Python tools.
//! Like Python's text-mode `write_text`, newlines follow the platform separator
//! (`\r\n` on Windows).

use std::io;

use serde::Serialize;

pub struct PythonFormatter {
    indent: &'static [u8],
    depth: usize,
    has_value: Vec<bool>,
}

impl PythonFormatter {
    pub fn new() -> Self {
        Self {
            indent: b" ",
            depth: 0,
            has_value: Vec::new(),
        }
    }

    fn mark_value(&mut self) {
        if let Some(has_value) = self.has_value.last_mut() {
            *has_value = true;
        }
    }

    fn write_indent<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        for _ in 0..self.depth {
            writer.write_all(self.indent)?;
        }
        Ok(())
    }

    fn write_close<W>(&mut self, writer: &mut W, close: u8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let had_value = self.has_value.pop().unwrap_or(false);
        self.depth = self.depth.saturating_sub(1);
        if had_value {
            writer.write_all(b"\n")?;
            self.write_indent(writer)?;
        }
        writer.write_all(&[close])
    }
}

impl Default for PythonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl serde_json::ser::Formatter for PythonFormatter {
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.depth += 1;
        self.has_value.push(false);
        writer.write_all(b"[")
    }

    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.write_close(writer, b']')
    }

    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.mark_value();
        if first {
            writer.write_all(b"\n")
        } else {
            writer.write_all(b",\n")
        }?;
        self.write_indent(writer)
    }

    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.depth += 1;
        self.has_value.push(false);
        writer.write_all(b"{")
    }

    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.write_close(writer, b'}')
    }

    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.mark_value();
        if first {
            writer.write_all(b"\n")
        } else {
            writer.write_all(b",\n")
        }?;
        self.write_indent(writer)
    }

    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b": ")
    }
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>, serde_json::Error>
where
    T: ?Sized + Serialize,
{
    let mut buffer = Vec::new();
    let mut serializer =
        serde_json::Serializer::with_formatter(&mut buffer, PythonFormatter::new());
    value.serialize(&mut serializer)?;
    Ok(platform_newlines(buffer))
}

fn platform_newlines(bytes: Vec<u8>) -> Vec<u8> {
    if !cfg!(windows) {
        return bytes;
    }
    let mut translated = Vec::with_capacity(bytes.len() + bytes.len() / 16);
    for byte in bytes {
        if byte == b'\n' {
            translated.push(b'\r');
        }
        translated.push(byte);
    }
    translated
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::to_bytes;

    #[derive(Serialize)]
    struct Inner {
        name: &'static str,
        empty: Vec<&'static str>,
    }

    #[derive(Serialize)]
    struct Sample {
        number: i64,
        text: &'static str,
        flag: bool,
        missing: Option<&'static str>,
        inner: Inner,
        list: Vec<Inner>,
    }

    #[test]
    fn formats_like_python_indent_one() {
        let sample = Sample {
            number: 1,
            text: "中文",
            flag: true,
            missing: None,
            inner: Inner {
                name: "a",
                empty: Vec::new(),
            },
            list: vec![Inner {
                name: "b",
                empty: Vec::new(),
            }],
        };
        let text = String::from_utf8(to_bytes(&sample).expect("serialize")).expect("utf8");
        let expected = concat!(
            "{\n",
            " \"number\": 1,\n",
            " \"text\": \"中文\",\n",
            " \"flag\": true,\n",
            " \"missing\": null,\n",
            " \"inner\": {\n",
            "  \"name\": \"a\",\n",
            "  \"empty\": []\n",
            " },\n",
            " \"list\": [\n",
            "  {\n",
            "   \"name\": \"b\",\n",
            "   \"empty\": []\n",
            "  }\n",
            " ]\n",
            "}"
        );
        assert_eq!(text, expected_with_platform_newlines(expected));
    }

    fn expected_with_platform_newlines(expected: &str) -> String {
        if cfg!(windows) {
            expected.replace('\n', "\r\n")
        } else {
            expected.to_string()
        }
    }
}
