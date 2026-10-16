use std::io::{self, BufRead, Write};
use std::sync::RwLock;
use serde_json::{json, Value};
use mcp_core::protocol::{create_initialize_result, JsonRpcRequest, JsonRpcResponse};

static TUNNEL_URL: RwLock<String> = RwLock::new(String::new());

fn get_tunnel_url() -> String {
    let r = TUNNEL_URL.read().unwrap();
    if r.is_empty() {
        std::env::var("COLAB_TUNNEL_URL").unwrap_or_else(|_| "http://localhost:8000".to_string())
    } else {
        r.clone()
    }
}

fn set_tunnel_url(url: String) {
    let mut w = TUNNEL_URL.write().unwrap();
    *w = url.trim_end_matches('/').to_string();
}

async fn handle_tool_call(tool_name: &str, args: &Value) -> Value {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());

    match tool_name {
        "colab_set_tunnel" => {
            let url = args.get("url").and_then(|u| u.as_str()).unwrap_or("");
            if url.is_empty() {
                json!({
                    "isError": true,
                    "content": [{ "type": "text", "text": "Error: Parameter 'url' is required." }]
                })
            } else {
                set_tunnel_url(url.to_string());
                let updated = get_tunnel_url();
                json!({
                    "content": [{ "type": "text", "text": format!("✅ URL del túnel de Colab actualizada a: {}", updated) }]
                })
            }
        }
        "colab_ping_hardware" => {
            let base_url = get_tunnel_url();
            let target = format!("{}/hardware", base_url);
            match client.get(&target).send().await {
                Ok(resp) => {
                    let status = resp.status();
                    let text = resp.text().await.unwrap_or_default();
                    if status.is_success() {
                        json!({
                            "content": [{ "type": "text", "text": format!("🟢 Conexión con Colab Exitosa:\n{}", text) }]
                        })
                    } else {
                        json!({
                            "isError": true,
                            "content": [{ "type": "text", "text": format!("❌ Error HTTP {} en {}: {}", status, target, text) }]
                        })
                    }
                }
                Err(e) => {
                    json!({
                        "isError": true,
                        "content": [{ "type": "text", "text": format!("❌ No se pudo conectar con Colab en {}: {}", base_url, e) }]
                    })
                }
            }
        }
        "colab_eval_code" => {
            let base_url = get_tunnel_url();
            let target = format!("{}/eval", base_url);
            let code = args.get("code")
                .or_else(|| args.get("python_code"))
                .and_then(|c| c.as_str())
                .unwrap_or("");

            match client.post(&target).json(&json!({ "code": code })).send().await {
                Ok(resp) => {
                    let status = resp.status();
                    let json_body: Result<Value, _> = resp.json().await;
                    if status.is_success() {
                        let output = json_body.as_ref()
                            .ok()
                            .and_then(|j| j.get("output"))
                            .and_then(|o| o.as_str())
                            .unwrap_or("");
                        json!({
                            "content": [{ "type": "text", "text": format!("📊 Resultado de Ejecución en Colab:\n{}", output) }]
                        })
                    } else {
                        let err_text = json_body.map(|j| j.to_string()).unwrap_or_default();
                        json!({
                            "isError": true,
                            "content": [{ "type": "text", "text": format!("❌ Error HTTP {} en {}: {}", status, target, err_text) }]
                        })
                    }
                }
                Err(e) => {
                    json!({
                        "isError": true,
                        "content": [{ "type": "text", "text": format!("❌ Error de ejecución en Colab {}: {}", target, e) }]
                    })
                }
            }
        }
        _ => {
            json!({
                "isError": true,
                "content": [{ "type": "text", "text": format!("Tool '{}' not found", tool_name) }]
            })
        }
    }
}

#[tokio::main]
async fn main() {
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
                    result: json!({
                        "tools": [
                            {
                                "name": "colab_set_tunnel",
                                "description": "Configura la URL pública del túnel Cloudflare o ngrok hacia Google Colab Live.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "url": { "type": "string", "description": "URL base del túnel HTTP" }
                                    },
                                    "required": ["url"]
                                }
                            },
                            {
                                "name": "colab_ping_hardware",
                                "description": "Consulta la GPU/TPU activa y memoria disponible en el runtime de Google Colab.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {}
                                }
                            },
                            {
                                "name": "colab_eval_code",
                                "description": "Ejecuta código de tensores, JAX, PyTorch o benchmarks en la GPU/TPU de Google Colab en tiempo real.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "code": { "type": "string", "description": "Código Python a ejecutar en Colab" }
                                    },
                                    "required": ["code"]
                                }
                            }
                        ]
                    }),
                },
                "tools/call" => {
                    let tool_name = req.params.as_ref()
                        .and_then(|p| p.get("name"))
                        .and_then(|n| n.as_str())
                        .unwrap_or("");
                    let default_args = json!({});
                    let args = req.params.as_ref()
                        .and_then(|p| p.get("arguments"))
                        .unwrap_or(&default_args);

                    let call_result = handle_tool_call(tool_name, args).await;
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: call_result,
                    }
                }
                _ => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: json!({ "error": "Method not found" }),
                },
            };

            let out_str = serde_json::to_string(&res).unwrap();
            println!("{}", out_str);
            stdout.flush().unwrap();
        }
        buffer.clear();
    }
}
