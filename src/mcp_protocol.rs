use serde_json::{json, Value};
use crate::providers::MultiCloudProvider;
use crate::local_ops;
use std::sync::Arc;

pub async fn handle_request(req: Value, provider: Arc<MultiCloudProvider>) -> Option<Value> {
    let id = req.get("id").cloned().unwrap_or(json!(null));
    let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or(json!({}));

    // Notifications do not have an id in jsonrpc and expect no response
    if method == "notifications/initialized" {
        return None;
    }

    if method == "initialize" {
        return Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "Rust-Unified-Bridge",
                    "version": "1.0.0"
                }
            }
        }));
    } else if method == "tools/list" {
        let mut tools = vec![];
        
        let c = &provider.config;
        
        if c.enable_groq {
            tools.push(json!({ "name": "ask_groq", "description": "Groq Llama API", "inputSchema": { "type": "object", "properties": { "mensaje": { "type": "string" }, "sistema": { "type": "string" } }, "required": ["mensaje"] } }));
        }
        if c.enable_gemini {
            tools.push(json!({ "name": "ask_gemini", "description": "Gemini Flash API", "inputSchema": { "type": "object", "properties": { "mensaje": { "type": "string" }, "sistema": { "type": "string" } }, "required": ["mensaje"] } }));
        }
        if c.enable_hf {
            tools.push(json!({ "name": "ask_hf", "description": "HuggingFace API", "inputSchema": { "type": "object", "properties": { "mensaje": { "type": "string" }, "modelo": { "type": "string" }, "sistema": { "type": "string" } }, "required": ["mensaje"] } }));
        }
        if c.enable_cerebras {
            tools.push(json!({ "name": "ask_cerebras", "description": "Cerebras API", "inputSchema": { "type": "object", "properties": { "mensaje": { "type": "string" }, "sistema": { "type": "string" } }, "required": ["mensaje"] } }));
        }
        if c.enable_sambanova {
            tools.push(json!({ "name": "ask_sambanova", "description": "SambaNova API", "inputSchema": { "type": "object", "properties": { "mensaje": { "type": "string" }, "sistema": { "type": "string" } }, "required": ["mensaje"] } }));
        }
        if c.enable_openrouter {
            tools.push(json!({ "name": "ask_openrouter", "description": "OpenRouter API", "inputSchema": { "type": "object", "properties": { "mensaje": { "type": "string" }, "sistema": { "type": "string" } }, "required": ["mensaje"] } }));
        }
        if c.enable_ollama {
            tools.push(json!({ "name": "ask_ollama", "description": "Ollama Local", "inputSchema": { "type": "object", "properties": { "mensaje": { "type": "string" }, "sistema": { "type": "string" } }, "required": ["mensaje"] } }));
        }
        if c.enable_local_ops {
            tools.push(json!({
                "name": "ejecutar",
                "description": "Dispatcher local para archivos y sistema",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "operacion": { "type": "string" },
                        "args": { "type": "string", "description": "JSON object string" }
                    },
                    "required": ["operacion"]
                }
            }));
            tools.push(json!({
                "name": "listar_operaciones",
                "description": "Lista de operaciones de archivo y sistema",
                "inputSchema": { "type": "object", "properties": {} }
            }));
        }

        return Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": { "tools": tools }
        }));
    } else if method == "tools/call" {
        let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let args_json = params.get("arguments").cloned().unwrap_or(json!({}));
        let mensaje = args_json.get("mensaje").and_then(|m| m.as_str()).unwrap_or("");
        let sistema = args_json.get("sistema").and_then(|s| s.as_str()).unwrap_or("");
        let modelo = args_json.get("modelo").and_then(|m| m.as_str()).unwrap_or("");

        let res = match name {
            "ask_groq" => provider.groq(mensaje, "llama-3.3-70b-versatile", sistema).await,
            "ask_gemini" => provider.gemini(mensaje, "gemini-2.0-flash", sistema).await,
            "ask_hf" => {
                let m = if modelo.is_empty() { "meta-llama/Llama-3.3-70B-Instruct" } else { modelo };
                provider.hf(mensaje, m, sistema).await
            },
            "ask_cerebras" => provider.cerebras(mensaje, "llama-3.3-70b", sistema).await,
            "ask_sambanova" => provider.sambanova(mensaje, "Meta-Llama-3.3-70B-Instruct", sistema).await,
            "ask_openrouter" => provider.openrouter(mensaje, sistema).await,
            "ask_ollama" => provider.ollama(mensaje, sistema).await,
            "listar_operaciones" => Ok("Operaciones: leer_txt, leer_md, leer_json, leer_csv, guardar_archivo, guardar_json, agregar_linea, reemplazar_texto, eliminar_archivo, mover_archivo, copiar_archivo, info_archivo, listar_archivos, listar_directorio, crear_directorio, eliminar_directorio, git_status, git_log, git_diff, ejecutar_cmd, tiempo_actual, uuid_gen".into()),
            "ejecutar" => {
                let op = args_json.get("operacion").and_then(|o| o.as_str()).unwrap_or("");
                let inner_args_str = args_json.get("args").and_then(|a| a.as_str()).unwrap_or("{}");
                let inner_args = serde_json::from_str::<Value>(inner_args_str).unwrap_or(json!({}));
                local_ops::ejecutar(op, inner_args)
            }
            _ => Err(format!("Herramienta desconocida: {}", name)),
        };

        match res {
            Ok(content) => {
                return Some(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": { "content": [{ "type": "text", "text": content }] }
                }));
            },
            Err(err) => {
                return Some(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                         "content": [{ "type": "text", "text": format!("Error: {}", err) }],
                         "isError": true
                    }
                }));
            }
        }
    }

    if req.get("id").is_some() {
        Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": -32601, "message": "Method not found" }
        }))
    } else {
        None
    }
}
