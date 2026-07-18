mod providers;
mod mcp_protocol;
mod local_ops;

use std::io::{self, BufRead, Write};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let provider = Arc::new(providers::MultiCloudProvider::new());

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut lines = stdin.lock().lines();

    // Loop over stdin lines (JSON-RPC messages)
    while let Some(Ok(line)) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Ok(req) = serde_json::from_str::<serde_json::Value>(line) {
            // Process the request
            let provider_clone = Arc::clone(&provider);
            
            // We use await directly here. If we wanted concurrent tool execution 
            // from the same client, we would tokio::spawn this.
            // But for MCP stdio, sequential is very safe.
            let res_opt = mcp_protocol::handle_request(req, provider_clone).await;
            
            if let Some(res) = res_opt {
                if let Ok(json_str) = serde_json::to_string(&res) {
                    println!("{}", json_str);
                    let _ = stdout.flush();
                }
            }
        }
    }
}
