use super::*;

const FAKE: &str = r#"
import json, sys
def read():
    length = 0
    while True:
        line = sys.stdin.buffer.readline()
        if not line: sys.exit(0)
        line = line.decode().strip()
        if not line: break
        if line.lower().startswith("content-length:"): length = int(line.split(":")[1])
    return json.loads(sys.stdin.buffer.read(length))
def send(msg):
    body = json.dumps(msg).encode()
    sys.stdout.buffer.write(b"Content-Length: %d\r\n\r\n" % len(body) + body)
    sys.stdout.buffer.flush()
while True:
    msg = read()
    m = msg.get("method")
    if m == "initialize":
        send({"jsonrpc": "2.0", "id": 900, "method": "workspace/configuration", "params": {"items": [{}]}})
        send({"jsonrpc": "2.0", "id": msg["id"], "result": {"capabilities": {}}})
    elif m == "textDocument/didOpen":
        uri = msg["params"]["textDocument"]["uri"]
        send({"jsonrpc": "2.0", "method": "textDocument/publishDiagnostics", "params": {"uri": uri, "diagnostics": [{"message": "unused", "severity": 2}]}})
    elif m == "textDocument/hover":
        send({"jsonrpc": "2.0", "id": msg["id"], "result": {"contents": "fn main()"}})
"#;

#[tokio::test]
async fn talks_to_a_server_and_routes_notifications() {
    let Some(python) = super::super::catalog::find_in_path("python3", None) else { return };
    let dir = tempfile::tempdir().unwrap();
    let script = dir.path().join("fake.py");
    std::fs::write(&script, FAKE).unwrap();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<(String, Value)>();
    let script_arg = script.to_string_lossy().to_string();
    let args: Vec<&str> = vec![script_arg.as_str()];
    let server = LspServer::start(&python, &args, dir.path(), None, move |method, params| {
        let _ = tx.send((method, params));
    })
    .await
    .unwrap();
    let uri = to_uri(&dir.path().join("main.rs"));
    let doc = json!({ "uri": uri, "languageId": "rust", "version": 1, "text": "fn main() {}" });
    server.notify("textDocument/didOpen", json!({ "textDocument": doc })).await.unwrap();
    let (method, params) = tokio::time::timeout(Duration::from_secs(5), rx.recv()).await.unwrap().unwrap();
    assert_eq!(method, "textDocument/publishDiagnostics");
    assert_eq!(params["diagnostics"][0]["message"], "unused");
    let hover = server.request("textDocument/hover", json!({})).await.unwrap();
    assert_eq!(hover["contents"], "fn main()");
    server.shutdown().await;
}
