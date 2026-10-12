use std::path::{Path, PathBuf};

const KEEP: &[u8] = b"-._~/";

pub fn to_uri(path: &Path) -> String {
    let raw = path.to_string_lossy().replace('\\', "/");
    let mut out = String::from("file://");
    if !raw.starts_with('/') {
        out.push('/');
    }
    for byte in raw.bytes() {
        if byte.is_ascii_alphanumeric() || KEEP.contains(&byte) || (byte == b':' && cfg!(windows)) {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

pub fn from_uri(uri: &str) -> Option<PathBuf> {
    let rest = uri.strip_prefix("file://")?;
    let bytes = rest.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[i + 1..i + 3]).ok()?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    let text = String::from_utf8(out).ok()?;
    let text = if cfg!(windows) { text.trim_start_matches('/').to_string() } else { text };
    Some(PathBuf::from(text))
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn round_trips_spaces_and_unicode() {
        let path = Path::new("/Users/ms/My Project/日本/a#b.rs");
        let uri = to_uri(path);
        assert_eq!(uri, "file:///Users/ms/My%20Project/%E6%97%A5%E6%9C%AC/a%23b.rs");
        assert_eq!(from_uri(&uri).as_deref(), Some(path));
    }
}
