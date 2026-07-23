use serde_json::{json, Value};
use crate::providers::MultiCloudProvider;
use crate::local_ops;
use std::sync::Arc;

fn tool(name: &str, desc: &str, props: Value, required: &[&str]) -> Value {
    json!({
        "name": name,
        "description": desc,
        "inputSchema": {
            "type": "object",
            "properties": props,
            "required": required
        }
    })
}

fn ok_response(id: Value, text: String) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": { "content": [{ "type": "text", "text": text }] }
    })
}

fn err_response(id: Value, err: String) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": { "content": [{ "type": "text", "text": format!("Error: {}", err) }], "isError": true }
    })
}

pub async fn handle_request(req: Value, provider: Arc<MultiCloudProvider>) -> Option<Value> {
    let id = req.get("id").cloned().unwrap_or(json!(null));
    let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or(json!({}));

    if method == "notifications/initialized" {
        return None;
    }

    // ─── initialize (SOTA Stateless Update July 2026) ───────────────────────
    if method == "initialize" {
        return Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": {
                    "name": "Rust-Unified-Bridge",
                    "version": "2.0.0-StatelessEnterprise"
                }
            }
        }));
    }

    // ─── tools/list ──────────────────────────────────────────────────────────
    if method == "tools/list" {
        let c = &provider.config;
        let mut tools: Vec<Value> = vec![];

        let ia_props = json!({
            "mensaje": { "type": "string", "description": "Tu pregunta o prompt" },
            "sistema": { "type": "string", "description": "Instrucción de sistema (opcional)" }
        });

        // ── Hubs Gratuitos / Cloud ──────────────────────────────────────────
        if c.enable_groq {
            tools.push(tool(
                "ask_groq",
                "Groq · Llama-3.3-70b-versatile (ultra-rápido, gratuito)",
                ia_props.clone(), &["mensaje"]
            ));
        }
        if c.enable_gemini {
            tools.push(tool(
                "ask_gemini",
                "Google · Gemini 2.0 Flash (gratuito)",
                ia_props.clone(), &["mensaje"]
            ));
        }
        if c.enable_hf {
            let hf_props = json!({
                "mensaje": { "type": "string" },
                "modelo": { "type": "string", "description": "Ej: meta-llama/Llama-3.3-70B-Instruct" },
                "sistema": { "type": "string" }
            });
            tools.push(tool(
                "ask_hf",
                "HuggingFace · Inference API (modelos open-weight)",
                hf_props, &["mensaje"]
            ));
        }
        if c.enable_cerebras {
            tools.push(tool(
                "ask_cerebras",
                "Cerebras · Llama-3.3-70b (inferencia en wafer, gratuito)",
                ia_props.clone(), &["mensaje"]
            ));
        }
        if c.enable_sambanova {
            tools.push(tool(
                "ask_sambanova",
                "SambaNova · Llama-3.3-70B (hardware RDU, gratuito)",
                ia_props.clone(), &["mensaje"]
            ));
        }
        if c.enable_kimi {
            let kimi_props = json!({
                "mensaje": { "type": "string" },
                "modelo": { "type": "string", "description": "Ej: moonshot-v1-8k, moonshot-v1-auto" },
                "sistema": { "type": "string" }
            });
            tools.push(tool(
                "ask_kimi",
                "Moonshot AI · Kimi 2.6 (modelo libre asiático)",
                kimi_props, &["mensaje"]
            ));
        }
        if c.enable_nvidia {
            let nv_props = json!({
                "mensaje": { "type": "string" },
                "modelo": { "type": "string", "description": "Ej: meta/llama-3.1-70b-instruct, deepseek-ai/deepseek-coder-33b-instruct" },
                "sistema": { "type": "string" }
            });
            tools.push(tool(
                "ask_nvidia",
                "NVIDIA NIM · 100+ Modelos de frontera (gratuito)",
                nv_props, &["mensaje"]
            ));
        }

        // ── OpenRouter Pago (3 slots configurables) ─────────────────────────
        if c.enable_openrouter {
            let m1 = c.openrouter_model_1.as_deref().unwrap_or("anthropic/claude-sonnet-4-5");
            let m2 = c.openrouter_model_2.as_deref().unwrap_or("thinkingmachines/inkling");
            let m3 = c.openrouter_model_3.as_deref().unwrap_or("openai/gpt-4o");

            tools.push(tool(
                "ask_openrouter_1",
                &format!("OpenRouter · Slot 1 → {} (pago)", m1),
                ia_props.clone(), &["mensaje"]
            ));
            tools.push(tool(
                "ask_openrouter_2",
                &format!("OpenRouter · Slot 2 → {} (pago)", m2),
                ia_props.clone(), &["mensaje"]
            ));
            tools.push(tool(
                "ask_openrouter_3",
                &format!("OpenRouter · Slot 3 → {} (pago)", m3),
                ia_props.clone(), &["mensaje"]
            ));
        }

        // ── Ollama Local (solo si está habilitado) ──────────────────────────
        if c.enable_ollama {
            let ollama_props = json!({
                "mensaje": { "type": "string" },
                "modelo": { "type": "string", "description": "Ej: deepseek-r1:14b, llama3.1:8b" },
                "sistema": { "type": "string" }
            });
            tools.push(tool(
                "ask_ollama",
                "Ollama · Modelos locales (modo nocturno)",
                ollama_props, &["mensaje"]
            ));
        }

        // ── Operaciones Locales ─────────────────────────────────────────────
        if c.enable_local_ops {
            tools.push(tool(
                "ejecutar",
                "Dispatcher local: archivos TXT/MD/JSON, Git, sistema",
                json!({
                    "operacion": { "type": "string", "description": "Nombre de la operación" },
                    "args": { "type": "string", "description": "Argumentos en JSON string" }
                }),
                &["operacion"]
            ));
            tools.push(tool(
                "listar_operaciones",
                "Lista todas las operaciones disponibles en el dispatcher local",
                json!({}), &[]
            ));
        }

        return Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": { "tools": tools }
        }));
    }

    // ─── tools/call ──────────────────────────────────────────────────────────
    if method == "tools/call" {
        let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
        let args  = params.get("arguments").cloned().unwrap_or(json!({}));
        let msg   = args.get("mensaje").and_then(|m| m.as_str()).unwrap_or("");
        let sys   = args.get("sistema").and_then(|s| s.as_str()).unwrap_or("");
        let mdl   = args.get("modelo").and_then(|m| m.as_str()).unwrap_or("");

        let res: Result<String, String> = match name {
            "ask_groq"         => provider.groq(msg, "llama-3.3-70b-versatile", sys).await,
            "ask_gemini"       => provider.gemini(msg, "gemini-2.0-flash", sys).await,
            "ask_hf"           => {
                let m = if mdl.is_empty() { "meta-llama/Llama-3.3-70B-Instruct" } else { mdl };
                provider.hf(msg, m, sys).await
            },
            "ask_cerebras"     => provider.cerebras(msg, "llama-3.3-70b", sys).await,
            "ask_sambanova"    => provider.sambanova(msg, "Meta-Llama-3.3-70B-Instruct", sys).await,
            "ask_kimi"         => {
                let m = if mdl.is_empty() { "moonshot-v1-8k" } else { mdl };
                provider.kimi(msg, m, sys).await
            },
            "ask_nvidia"       => {
                let m = if mdl.is_empty() { "meta/llama-3.1-70b-instruct" } else { mdl };
                provider.nvidia(msg, m, sys).await
            },
            "ask_openrouter_1" => provider.openrouter(msg, sys, 1).await,
            "ask_openrouter_2" => provider.openrouter(msg, sys, 2).await,
            "ask_openrouter_3" => provider.openrouter(msg, sys, 3).await,
            "ask_ollama"       => {
                let m = if mdl.is_empty() { "deepseek-r1:14b" } else { mdl };
                provider.ollama(msg, m, sys).await
            },
            "listar_operaciones" => Ok(
                "leer_txt, leer_md, leer_json, leer_csv, guardar_archivo, guardar_json, \
                 agregar_linea, reemplazar_texto, eliminar_archivo, mover_archivo, copiar_archivo, \
                 info_archivo, listar_archivos, listar_directorio, crear_directorio, \
                 eliminar_directorio, git_status, git_log, git_diff, ejecutar_cmd, \
                 tiempo_actual, uuid_gen".into()
            ),
            "ejecutar" => {
                let op = args.get("operacion").and_then(|o| o.as_str()).unwrap_or("");
                let inner_str = args.get("args").and_then(|a| a.as_str()).unwrap_or("{}");
                let inner = serde_json::from_str::<Value>(inner_str).unwrap_or(json!({}));
                local_ops::ejecutar(op, inner)
            },
            _ => Err(format!("Herramienta desconocida: {}", name)),
        };

        return Some(match res {
            Ok(text) => ok_response(id, text),
            Err(err) => err_response(id, err),
        });
    }

    // ─── Method not found ─────────────────────────────────────────────────
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
