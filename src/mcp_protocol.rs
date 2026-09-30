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

fn resolve_model(passed: &str, config_val: Option<&String>, provider_name: &str) -> Result<String, String> {
    if !passed.trim().is_empty() {
        Ok(passed.trim().to_string())
    } else if let Some(m) = config_val {
        if !m.trim().is_empty() {
            return Ok(m.trim().to_string());
        }
        Err(format!("El modelo para '{}' en config.json está vacío. Especifique 'modelo' en la llamada o en config.json.", provider_name))
    } else {
        Err(format!("Sin modelo configurado para '{}' en config.json ni en el parámetro 'modelo'. Por favor defina el modelo en config.json o páselo en la consulta.", provider_name))
    }
}

pub async fn handle_request(req: Value, provider: Arc<MultiCloudProvider>) -> Option<Value> {
    let id = req.get("id").cloned().unwrap_or(json!(null));
    let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or(json!({}));

    if method == "notifications/initialized" {
        return None;
    }

    // ─── initialize ──────────────────────────────────────────────────────────
    if method == "initialize" {
        return Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": {
                    "name": "Rust-Unified-Bridge",
                    "version": "2.1.0-ZeroHardcode"
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
            "modelo": { "type": "string", "description": "Nombre del modelo (opcional, invalida la configuración de config.json)" },
            "sistema": { "type": "string", "description": "Instrucción de sistema (opcional)" }
        });

        // ── Hubs Cloud ───────────────────────────────────────────────────────
        if c.enable_groq {
            tools.push(tool(
                "ask_groq",
                "Groq · Inferencia LPU ultra-rápida (Modelo configurable via config.json o parámetro 'modelo')",
                ia_props.clone(), &["mensaje"]
            ));
        }
        if c.enable_gemini {
            tools.push(tool(
                "ask_gemini",
                "Google Gemini · API AI Studio (Modelo configurable via config.json o parámetro 'modelo')",
                ia_props.clone(), &["mensaje"]
            ));
        }
        if c.enable_hf {
            tools.push(tool(
                "ask_hf",
                "HuggingFace · Inference API (Modelo configurable via config.json o parámetro 'modelo')",
                ia_props.clone(), &["mensaje"]
            ));
        }
        if c.enable_cerebras {
            tools.push(tool(
                "ask_cerebras",
                "Cerebras WSE · Inferencia a ultra-alta velocidad (Modelo configurable via config.json o parámetro 'modelo')",
                ia_props.clone(), &["mensaje"]
            ));
        }
        if c.enable_sambanova {
            tools.push(tool(
                "ask_sambanova",
                "SambaNova · Hardware RDU (Modelo configurable via config.json o parámetro 'modelo')",
                ia_props.clone(), &["mensaje"]
            ));
        }
        if c.enable_kimi {
            tools.push(tool(
                "ask_kimi",
                "Moonshot AI / Kimi · Razonamiento y contexto masivo (Modelo configurable via config.json o parámetro 'modelo')",
                ia_props.clone(), &["mensaje"]
            ));
        }
        if c.enable_nvidia {
            tools.push(tool(
                "ask_nvidia",
                "NVIDIA NIM · Modelos de frontera (Modelo configurable via config.json o parámetro 'modelo')",
                ia_props.clone(), &["mensaje"]
            ));
        }

        // ── OpenRouter ───────────────────────────────────────────────────────
        if c.enable_openrouter {
            tools.push(tool(
                "ask_openrouter_1",
                &format!("OpenRouter Slot 1 · Modelo: {} (opcional override con 'modelo')", c.openrouter_model_1.as_deref().unwrap_or("no-configurado")),
                ia_props.clone(), &["mensaje"]
            ));
            tools.push(tool(
                "ask_openrouter_2",
                &format!("OpenRouter Slot 2 · Modelo: {} (opcional override con 'modelo')", c.openrouter_model_2.as_deref().unwrap_or("no-configurado")),
                ia_props.clone(), &["mensaje"]
            ));
            tools.push(tool(
                "ask_openrouter_3",
                &format!("OpenRouter Slot 3 · Modelo: {} (opcional override con 'modelo')", c.openrouter_model_3.as_deref().unwrap_or("no-configurado")),
                ia_props.clone(), &["mensaje"]
            ));
            tools.push(tool(
                "ask_openrouter",
                "OpenRouter Multi-Model Gateway (Requiere especificar 'modelo' o usa el de config.json)",
                ia_props.clone(), &["mensaje"]
            ));
        }

        // ── Ollama Local ─────────────────────────────────────────────────────
        if c.enable_ollama {
            tools.push(tool(
                "ask_ollama",
                "Ollama Local · Modelos en máquina (Modelo configurable via config.json o parámetro 'modelo')",
                ia_props.clone(), &["mensaje"]
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

        let c = &provider.config;

        let res: Result<String, String> = match name {
            "ask_groq" => match resolve_model(mdl, c.groq_model.as_ref(), "groq") {
                Ok(m) => provider.groq(msg, &m, sys).await,
                Err(e) => Err(e),
            },
            "ask_gemini" => match resolve_model(mdl, c.gemini_model.as_ref(), "gemini") {
                Ok(m) => provider.gemini(msg, &m, sys).await,
                Err(e) => Err(e),
            },
            "ask_hf" => match resolve_model(mdl, c.hf_model.as_ref(), "hf") {
                Ok(m) => provider.hf(msg, &m, sys).await,
                Err(e) => Err(e),
            },
            "ask_cerebras" => match resolve_model(mdl, c.cerebras_model.as_ref(), "cerebras") {
                Ok(m) => provider.cerebras(msg, &m, sys).await,
                Err(e) => Err(e),
            },
            "ask_sambanova" => match resolve_model(mdl, c.sambanova_model.as_ref(), "sambanova") {
                Ok(m) => provider.sambanova(msg, &m, sys).await,
                Err(e) => Err(e),
            },
            "ask_kimi" => match resolve_model(mdl, c.kimi_model.as_ref(), "kimi") {
                Ok(m) => provider.kimi(msg, &m, sys).await,
                Err(e) => Err(e),
            },
            "ask_nvidia" => match resolve_model(mdl, c.nvidia_model.as_ref(), "nvidia") {
                Ok(m) => provider.nvidia(msg, &m, sys).await,
                Err(e) => Err(e),
            },
            "ask_openrouter_1" => match resolve_model(mdl, c.openrouter_model_1.as_ref(), "openrouter_model_1") {
                Ok(m) => provider.openrouter(msg, &m, sys).await,
                Err(e) => Err(e),
            },
            "ask_openrouter_2" => match resolve_model(mdl, c.openrouter_model_2.as_ref(), "openrouter_model_2") {
                Ok(m) => provider.openrouter(msg, &m, sys).await,
                Err(e) => Err(e),
            },
            "ask_openrouter_3" => match resolve_model(mdl, c.openrouter_model_3.as_ref(), "openrouter_model_3") {
                Ok(m) => provider.openrouter(msg, &m, sys).await,
                Err(e) => Err(e),
            },
            "ask_openrouter" => match resolve_model(mdl, c.openrouter_model_1.as_ref(), "openrouter") {
                Ok(m) => provider.openrouter(msg, &m, sys).await,
                Err(e) => Err(e),
            },
            "ask_ollama" => match resolve_model(mdl, c.ollama_model.as_ref(), "ollama") {
                Ok(m) => provider.ollama(msg, &m, sys).await,
                Err(e) => Err(e),
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
