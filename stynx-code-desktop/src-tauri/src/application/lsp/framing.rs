use serde_json::Value;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt};

const MAX_MESSAGE: usize = 64 * 1024 * 1024;

pub async fn read_message<R: AsyncBufRead + Unpin>(reader: &mut R) -> Result<Option<Value>, String> {
    let mut length: Option<usize> = None;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).await.map_err(|e| e.to_string())? == 0 {
            return Ok(None);
        }
        let line = line.trim_end();
        if line.is_empty() {
            if length.is_some() {
                break;
            }
            continue;
        }
        if let Some((name, value)) = line.split_once(':')
            && name.trim().eq_ignore_ascii_case("content-length")
        {
            length = value.trim().parse().ok();
        }
    }
    let length = length.filter(|n| *n <= MAX_MESSAGE).ok_or("invalid Content-Length")?;
    let mut body = vec![0u8; length];
    reader.read_exact(&mut body).await.map_err(|e| e.to_string())?;
    serde_json::from_slice(&body).map(Some).map_err(|e| e.to_string())
}

pub async fn write_message<W: AsyncWrite + Unpin>(writer: &mut W, message: &Value) -> Result<(), String> {
    let body = serde_json::to_vec(message).map_err(|e| e.to_string())?;
    let header = format!("Content-Length: {}\r\n\r\n", body.len());
    writer.write_all(header.as_bytes()).await.map_err(|e| e.to_string())?;
    writer.write_all(&body).await.map_err(|e| e.to_string())?;
    writer.flush().await.map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::io::BufReader;

    #[tokio::test]
    async fn reads_back_to_back_messages_and_extra_headers() {
        let a = json!({"jsonrpc": "2.0", "id": 1, "result": "héllo"});
        let b = json!({"jsonrpc": "2.0", "method": "x"});
        let mut raw = Vec::new();
        write_message(&mut raw, &a).await.unwrap();
        let body = serde_json::to_vec(&b).unwrap();
        raw.extend(format!("Content-Type: application/vscode-jsonrpc\r\ncontent-length: {}\r\n\r\n", body.len()).bytes());
        raw.extend(body);
        let mut reader = BufReader::with_capacity(7, raw.as_slice());
        assert_eq!(read_message(&mut reader).await.unwrap(), Some(a));
        assert_eq!(read_message(&mut reader).await.unwrap(), Some(b));
        assert_eq!(read_message(&mut reader).await.unwrap(), None);
    }
}
