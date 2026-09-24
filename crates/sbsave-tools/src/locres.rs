//! Unreal Engine `.locres` reader (port of the former Python tool).

use std::collections::HashMap;
use std::path::Path;

const LOCRES_MAGIC: [u8; 16] = [
    0x0E, 0x14, 0x74, 0x75, 0x67, 0x4A, 0x03, 0xFC, 0x4A, 0x15, 0x90, 0x9D, 0xC3, 0x37, 0x7F, 0x1B,
];

/// Parsed namespaces in file order; a repeated namespace name replaces the
/// earlier entries in place, matching Python dict semantics.
pub type Namespaces = Vec<(String, HashMap<String, String>)>;

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], String> {
        let end = self
            .pos
            .checked_add(length)
            .ok_or_else(|| "locres 读取越界".to_string())?;
        if end > self.data.len() {
            return Err(format!(
                "locres 读取越界: pos={} length={} total={}",
                self.pos,
                length,
                self.data.len()
            ));
        }
        let slice = &self.data[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }

    fn u32(&mut self) -> Result<u32, String> {
        let bytes: [u8; 4] = self.take(4)?.try_into().expect("length checked");
        Ok(u32::from_le_bytes(bytes))
    }

    fn i32(&mut self) -> Result<i32, String> {
        let bytes: [u8; 4] = self.take(4)?.try_into().expect("length checked");
        Ok(i32::from_le_bytes(bytes))
    }

    fn i64(&mut self) -> Result<i64, String> {
        let bytes: [u8; 8] = self.take(8)?.try_into().expect("length checked");
        Ok(i64::from_le_bytes(bytes))
    }

    fn fstring(&mut self) -> Result<String, String> {
        let length = self.i32()?;
        if length == 0 {
            return Ok(String::new());
        }
        if length > 0 {
            let raw = self.take(length as usize)?;
            let text = String::from_utf8_lossy(raw);
            return Ok(text.split('\0').next().unwrap_or_default().to_string());
        }
        let units = (-length) as usize;
        let raw = self.take(units * 2)?;
        let utf16: Vec<u16> = raw
            .as_chunks::<2>()
            .0
            .iter()
            .map(|chunk| u16::from_le_bytes(*chunk))
            .collect();
        let text = String::from_utf16_lossy(&utf16);
        Ok(text.split('\0').next().unwrap_or_default().to_string())
    }
}

/// Parse a `.locres` file into namespaces of key -> text.
pub fn read_locres(path: &Path) -> Result<Namespaces, String> {
    let data =
        std::fs::read(path).map_err(|error| format!("读取 {} 失败: {error}", path.display()))?;
    let mut reader = Reader::new(&data);
    let version = if data.starts_with(&LOCRES_MAGIC) {
        reader.pos = 16;
        reader.u8()?
    } else {
        0
    };

    let mut strings: Vec<String> = Vec::new();
    if version >= 2 {
        let offset = reader.i64()?;
        if offset != -1 {
            let saved = reader.pos;
            reader.pos =
                usize::try_from(offset).map_err(|_| format!("locres 字符串偏移非法: {offset}"))?;
            let count = reader.i32()?;
            for _ in 0..count.max(0) {
                strings.push(reader.fstring()?);
                if version >= 3 {
                    reader.i32()?;
                }
            }
            reader.pos = saved;
        }
    }
    if version >= 3 {
        reader.take(4)?; // total entry count
    }

    let namespaces_count = reader.u32()?;
    let mut namespaces: Namespaces = Vec::new();
    for _ in 0..namespaces_count {
        if version >= 3 {
            reader.u32()?; // namespace hash
        }
        let namespace = reader.fstring()?;
        let entry_count = reader.u32()?;
        let mut entries: HashMap<String, String> = HashMap::new();
        for _ in 0..entry_count {
            if version >= 3 {
                reader.u32()?; // key hash
            }
            let key = reader.fstring()?;
            reader.u32()?; // source string hash
            let index = reader.i32()?;
            if index >= 0 {
                if let Some(text) = strings.get(index as usize) {
                    entries.insert(key, text.clone());
                }
            }
            if version > 3 {
                reader.take(4)?; // Stellar Blade extra field
            }
        }
        match namespaces
            .iter_mut()
            .find(|(existing, _)| existing == &namespace)
        {
            Some((_, existing)) => *existing = entries,
            None => namespaces.push((namespace, entries)),
        }
    }
    Ok(namespaces)
}

/// Flatten namespaces into a single map; the first occurrence of a key wins.
pub fn flatten(namespaces: &Namespaces) -> HashMap<String, String> {
    let mut flat: HashMap<String, String> = HashMap::new();
    for (_, entries) in namespaces {
        for (key, text) in entries {
            flat.entry(key.to_lowercase())
                .or_insert_with(|| text.clone());
        }
    }
    flat
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{flatten, read_locres, LOCRES_MAGIC};

    fn push_i32(buffer: &mut Vec<u8>, value: i32) {
        buffer.extend_from_slice(&value.to_le_bytes());
    }

    fn push_u32(buffer: &mut Vec<u8>, value: u32) {
        buffer.extend_from_slice(&value.to_le_bytes());
    }

    fn push_fstring(buffer: &mut Vec<u8>, text: &str) {
        let bytes = text.as_bytes();
        push_i32(buffer, bytes.len() as i32 + 1);
        buffer.extend_from_slice(bytes);
        buffer.push(0);
    }

    #[test]
    fn reads_version_three_file() {
        let mut strings_section: Vec<u8> = Vec::new();
        push_i32(&mut strings_section, 1);
        push_fstring(&mut strings_section, "Hello 世界");
        push_i32(&mut strings_section, 0);

        let mut tail: Vec<u8> = Vec::new();
        push_u32(&mut tail, 1); // total entry count (skipped)
        push_u32(&mut tail, 1); // namespace count
        push_u32(&mut tail, 0); // namespace hash
        push_fstring(&mut tail, "SBNames");
        push_u32(&mut tail, 1); // entry count
        push_u32(&mut tail, 0); // key hash
        push_fstring(&mut tail, "Can_001");
        push_u32(&mut tail, 0); // source string hash
        push_i32(&mut tail, 0); // string index

        let header_size = 16 + 1 + 8;
        let strings_offset = header_size + tail.len();

        let mut file: Vec<u8> = Vec::new();
        file.extend_from_slice(&LOCRES_MAGIC);
        file.push(3);
        file.extend_from_slice(&(strings_offset as i64).to_le_bytes());
        file.extend_from_slice(&tail);
        file.extend_from_slice(&strings_section);

        let path = std::env::temp_dir().join("sbsave-tools-locres-test.locres");
        std::fs::write(&path, &file).expect("write");
        let namespaces = read_locres(&path).expect("parse");
        assert_eq!(namespaces[0].0, "SBNames");
        assert_eq!(namespaces[0].1["Can_001"], "Hello 世界");
        let flat = flatten(&namespaces);
        assert_eq!(flat["can_001"], "Hello 世界");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn flatten_keeps_first_occurrence() {
        let namespaces = vec![
            (
                "A".to_string(),
                HashMap::from([("Key".to_string(), "first".to_string())]),
            ),
            (
                "B".to_string(),
                HashMap::from([("KEY".to_string(), "second".to_string())]),
            ),
        ];
        let flat = flatten(&namespaces);
        assert_eq!(flat["key"], "first");
    }
}
