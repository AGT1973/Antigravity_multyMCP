use crate::protocol::*;
use crate::circuit_breaker::KeyMatrix;
use crate::http_client::execute_openai_compatible;
use std::io::{self, BufRead, Write};
use serde_json::Value;

pub trait ProviderConfig {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn env_prefix(&self) -> &str;
    fn api_url(&self) -> &str;
    fn default_model(&self) -> &str;
}

pub async fn run_server<P: ProviderConfig>(provider: P) {
    let mut keys = KeyMatrix::new(provider.env_prefix());
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    while handle.read_line(&mut buffer).unwrap_or(0) > 0 {
        if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(&buffer) {
            let res = match req.method.as_str() {
                "initialize" => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: create_initialize_result(),
                },
                "tools/list" => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: serde_json::json!({
                        "tools": [{
                            "name": provider.name(),
                            "description": provider.description(),
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "prompt": { "type": "string" }
                                },
                                "required": ["prompt"]
                            }
                        }]
                    }),
                },
                "tools/call" => {
                    let mut call_result = serde_json::json!({
                        "isError": true,
                        "content": [{ "type": "text", "text": "No valid API keys available." }]
                    });

                    // Parse prompt
                    let prompt = req.params.as_ref()
                        .and_then(|p| p.get("arguments"))
                        .and_then(|a| a.get("prompt"))
                        .and_then(|pr| pr.as_str())
                        .unwrap_or("Hello")
                        .to_string();

                    // REGLA: No más hardcodeos. Buscar versión dinámica desde ENV/JSON.
                    let mut actual_model = provider.default_model().to_string();
                    let prefix = provider.env_prefix().split('_').next().unwrap_or("API");
                    let specific_env = format!("{}_MODEL", prefix);
                    
                    if let Ok(env_val) = std::env::var(&specific_env).or_else(|_| std::env::var("MODEL")) {
                        if !env_val.is_empty() {
                            actual_model = env_val;
                        }
                    }

                    // Intentamos con la matriz de llaves (Retry/Circuit breaker loop)
                    for _ in 0..3 {
                        if !keys.is_available() {
                            break;
                        }

                        if let Some(key) = keys.get_current_key() {
                            match execute_openai_compatible(
                                provider.api_url(),
                                &actual_model,
                                &key,
                                &prompt
                            ).await {
                                Ok(content) => {
                                    call_result = serde_json::json!({
                                        "content": [{
                                            "type": "text",
                                            "text": content
                                        }]
                                    });
                                    break;
                                }
                                Err(err) => {
                                    let err_msg = err.to_string();
                                    if err_msg.contains("429") {
                                        keys.mark_failure(true); // Gira la llave!
                                    } else {
                                        keys.mark_failure(false); // Circuit break temporal
                                        call_result = serde_json::json!({
                                            "isError": true,
                                            "content": [{ "type": "text", "text": format!("API Error: {}", err_msg) }]
                                        });
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: call_result,
                    }
                }
                _ => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: serde_json::json!({ "error": "Method not found" }),
                },
            };

            let out_str = serde_json::to_string(&res).unwrap();
            println!("{}", out_str);
            stdout.flush().unwrap();
        }
        buffer.clear();
    }
}
