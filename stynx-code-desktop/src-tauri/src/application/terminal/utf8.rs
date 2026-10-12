#[derive(Default)]
pub struct Utf8Carry {
    pending: Vec<u8>,
}

impl Utf8Carry {
    pub fn push(&mut self, bytes: &[u8]) -> String {
        self.pending.extend_from_slice(bytes);
        let mut out = String::new();
        let mut rest: &[u8] = &self.pending;
        loop {
            match std::str::from_utf8(rest) {
                Ok(text) => {
                    out.push_str(text);
                    rest = &[];
                    break;
                }
                Err(error) => {
                    let (valid, after) = rest.split_at(error.valid_up_to());
                    out.push_str(std::str::from_utf8(valid).unwrap_or_default());
                    match error.error_len() {
                        Some(bad) => {
                            out.push('\u{FFFD}');
                            rest = &after[bad..];
                        }
                        None => {
                            rest = after;
                            break;
                        }
                    }
                }
            }
        }
        self.pending = rest.to_vec();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_characters_split_across_reads() {
        let bytes = "héllo 🚀 日本".as_bytes();
        let mut carry = Utf8Carry::default();
        let mut out = String::new();
        for chunk in bytes.chunks(3) {
            out.push_str(&carry.push(chunk));
        }
        assert_eq!(out, "héllo 🚀 日本");
    }

    #[test]
    fn replaces_invalid_bytes_and_continues() {
        let mut carry = Utf8Carry::default();
        assert_eq!(carry.push(&[b'a', 0xff, b'b']), "a\u{FFFD}b");
    }

    #[test]
    fn holds_incomplete_tail() {
        let mut carry = Utf8Carry::default();
        let rocket = "🚀".as_bytes();
        assert_eq!(carry.push(&rocket[..2]), "");
        assert_eq!(carry.push(&rocket[2..]), "🚀");
    }
}
