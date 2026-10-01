use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    // API Keys
    pub openrouter_api_key: Option<String>,
    pub groq_api_key: Option<String>,
    pub gemini_api_key: Option<String>,
    pub hf_token: Option<String>,
    pub cerebras_api_key: Option<String>,
    pub sambanova_api_key: Option<String>,
    pub kimi_api_key: Option<String>,
    pub nvidia_api_key: Option<String>,
    pub perplexity_api_key: Option<String>,

    // Models (Zero hardcoding - configured in config.json or passed dynamically)
    pub groq_model: Option<String>,
    pub gemini_model: Option<String>,
    pub hf_model: Option<String>,
    pub cerebras_model: Option<String>,
    pub sambanova_model: Option<String>,
    pub kimi_model: Option<String>,
    pub nvidia_model: Option<String>,
    pub openrouter_model_1: Option<String>,
    pub openrouter_model_2: Option<String>,
    pub openrouter_model_3: Option<String>,
    pub ollama_model: Option<String>,
    // Perplexity: preset ("fast"|"low"|"medium"|"high") o model ID directo
    pub perplexity_preset: Option<String>,

    // Toggles
    #[serde(default)] pub enable_openrouter: bool,
    #[serde(default)] pub enable_groq: bool,
    #[serde(default)] pub enable_gemini: bool,
    #[serde(default)] pub enable_hf: bool,
    #[serde(default)] pub enable_cerebras: bool,
    #[serde(default)] pub enable_sambanova: bool,
    #[serde(default)] pub enable_kimi: bool,
    #[serde(default)] pub enable_nvidia: bool,
    #[serde(default)] pub enable_local_ops: bool,
    #[serde(default)] pub enable_ollama: bool,
    #[serde(default)] pub enable_perplexity: bool,
}

pub fn load_config() -> Config {
    // Try current_exe path first, then current_dir
    let mut config_path = PathBuf::from("config.json");
    
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let candidate = parent.join("config.json");
            if candidate.exists() {
                config_path = candidate;
            } else if let Some(project_root) = parent.parent().and_then(|p| p.parent()) {
                let dev_candidate = project_root.join("config.json");
                if dev_candidate.exists() {
                    config_path = dev_candidate;
                }
            }
        }
    }

    if let Ok(content) = fs::read_to_string(&config_path) {
        if let Ok(config) = serde_json::from_str(&content) {
            return config;
        }
    }
    Config::default()
}

pub struct MultiCloudProvider {
    client: Client,
    pub config: Config,
}

impl MultiCloudProvider {
    pub fn new() -> Self {
        Self {
            client: Client::builder().timeout(Duration::from_secs(120)).build().unwrap_or_default(),
            config: load_config(),
        }
    }

    pub fn reload_config(&mut self) {
        self.config = load_config();
    }

    fn msgs(prompt: &str, system: &str) -> Vec<Value> {
        let mut m = vec![];
        if !system.is_empty() {
            m.push(json!({"role": "system", "content": system}));
        }
        m.push(json!({"role": "user", "content": prompt}));
        m
    }

    pub async fn groq(&self, prompt: &str, model: &str, system: &str) -> Result<String, String> {
        if !self.config.enable_groq { return Err("Groq is disabled".into()); }
        let key = self.config.groq_api_key.as_deref().ok_or("Sin groq_api_key")?;
        let res = self.client.post("https://api.groq.com/openai/v1/chat/completions")
            .bearer_auth(key)
            .json(&json!({
                "model": model,
                "messages": Self::msgs(prompt, system)
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = res.status();
        let body: Value = res.json().await.map_err(|e| e.to_string())?;
        if status.is_success() {
            if let Some(c) = body["choices"][0]["message"]["content"].as_str() {
                return Ok(c.to_string());
            }
        }
        Err(format!("Groq HTTP {}: {}", status, body))
    }

    pub async fn gemini(&self, prompt: &str, model: &str, system: &str) -> Result<String, String> {
        if !self.config.enable_gemini { return Err("Gemini is disabled".into()); }
        let key = self.config.gemini_api_key.as_deref().ok_or("Sin gemini_api_key")?;
        let res = self.client.post("https://generativelanguage.googleapis.com/v1beta/openai/chat/completions")
            .bearer_auth(key)
            .json(&json!({
                "model": model,
                "messages": Self::msgs(prompt, system)
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = res.status();
        let body: Value = res.json().await.map_err(|e| e.to_string())?;
        if status.is_success() {
            if let Some(c) = body["choices"][0]["message"]["content"].as_str() {
                return Ok(c.to_string());
            }
        }
        Err(format!("Gemini HTTP {}: {}", status, body))
    }

    pub async fn hf(&self, prompt: &str, model: &str, system: &str) -> Result<String, String> {
        if !self.config.enable_hf { return Err("HuggingFace is disabled".into()); }
        let key = self.config.hf_token.as_deref().ok_or("Sin hf_token")?;
        let res = self.client.post("https://api-inference.huggingface.co/v1/chat/completions")
            .bearer_auth(key)
            .json(&json!({
                "model": model,
                "messages": Self::msgs(prompt, system),
                "max_tokens": 2048
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = res.status();
        let body: Value = res.json().await.map_err(|e| e.to_string())?;
        if status.is_success() {
            if let Some(c) = body["choices"][0]["message"]["content"].as_str() {
                return Ok(c.to_string());
            }
        }
        Err(format!("HuggingFace HTTP {}: {}", status, body))
    }

    pub async fn cerebras(&self, prompt: &str, model: &str, system: &str) -> Result<String, String> {
        if !self.config.enable_cerebras { return Err("Cerebras is disabled".into()); }
        let key = self.config.cerebras_api_key.as_deref().ok_or("Sin cerebras_api_key")?;
        let res = self.client.post("https://api.cerebras.ai/v1/chat/completions")
            .bearer_auth(key)
            .header("User-Agent", "Antigravity-MultiMCP/2.0")
            .json(&json!({
                "model": model,
                "messages": Self::msgs(prompt, system)
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = res.status();
        let body: Value = res.json().await.map_err(|e| e.to_string())?;
        if status.is_success() {
            if let Some(c) = body["choices"][0]["message"]["content"].as_str() {
                return Ok(c.to_string());
            }
        }
        Err(format!("Cerebras HTTP {}: {}", status, body))
    }

    pub async fn sambanova(&self, prompt: &str, model: &str, system: &str) -> Result<String, String> {
        if !self.config.enable_sambanova { return Err("SambaNova is disabled".into()); }
        let key = self.config.sambanova_api_key.as_deref().ok_or("Sin sambanova_api_key")?;
        let res = self.client.post("https://api.sambanova.ai/v1/chat/completions")
            .bearer_auth(key)
            .json(&json!({
                "model": model,
                "messages": Self::msgs(prompt, system)
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = res.status();
        let body: Value = res.json().await.map_err(|e| e.to_string())?;
        if status.is_success() {
            if let Some(c) = body["choices"][0]["message"]["content"].as_str() {
                return Ok(c.to_string());
            }
        }
        Err(format!("SambaNova HTTP {}: {}", status, body))
    }

    pub async fn kimi(&self, prompt: &str, model: &str, system: &str) -> Result<String, String> {
        if !self.config.enable_kimi { return Err("Kimi is disabled".into()); }
        let key = self.config.kimi_api_key.as_deref().ok_or("Sin kimi_api_key")?;
        let res = self.client.post("https://api.moonshot.ai/v1/chat/completions")
            .bearer_auth(key)
            .json(&json!({
                "model": model,
                "messages": Self::msgs(prompt, system)
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = res.status();
        let body: Value = res.json().await.map_err(|e| e.to_string())?;
        if status.is_success() {
            if let Some(c) = body["choices"][0]["message"]["content"].as_str() {
                return Ok(c.to_string());
            }
        }
        Err(format!("Kimi HTTP {}: {}", status, body))
    }

    pub async fn nvidia(&self, prompt: &str, model: &str, system: &str) -> Result<String, String> {
        if !self.config.enable_nvidia { return Err("NVIDIA is disabled".into()); }
        let key = self.config.nvidia_api_key.as_deref().ok_or("Sin nvidia_api_key")?;
        let res = self.client.post("https://integrate.api.nvidia.com/v1/chat/completions")
            .bearer_auth(key)
            .json(&json!({
                "model": model,
                "messages": Self::msgs(prompt, system),
                "max_tokens": 1024
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = res.status();
        let body: Value = res.json().await.map_err(|e| e.to_string())?;
        if status.is_success() {
            if let Some(c) = body["choices"][0]["message"]["content"].as_str() {
                return Ok(c.to_string());
            }
        }
        Err(format!("NVIDIA HTTP {}: {}", status, body))
    }

    // Llama a OpenRouter con modelo dinámico (pasado o de config)
    pub async fn openrouter(&self, prompt: &str, model: &str, system: &str) -> Result<String, String> {
        if !self.config.enable_openrouter { return Err("OpenRouter is disabled".into()); }
        let key = self.config.openrouter_api_key.as_deref().ok_or("Sin openrouter_api_key")?;

        let res = self.client.post("https://openrouter.ai/api/v1/chat/completions")
            .bearer_auth(key)
            .header("HTTP-Referer", "https://github.com/AGT1973/Antigravity_multyMCP")
            .header("X-Title", "Antigravity MultiMCP")
            .json(&json!({
                "model": model,
                "messages": Self::msgs(prompt, system)
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = res.status();
        let body: Value = res.json().await.map_err(|e| e.to_string())?;
        if status.is_success() {
            if let Some(c) = body["choices"][0]["message"]["content"].as_str() {
                return Ok(c.to_string());
            }
        }
        Err(format!("OpenRouter({}) HTTP {}: {}", model, status, body))
    }

    pub async fn ollama(&self, prompt: &str, model: &str, system: &str) -> Result<String, String> {
        if !self.config.enable_ollama { return Err("Ollama is disabled in this bridge. Use mcp-ollama-nocturno server.".into()); }
        let res = self.client.post("http://localhost:11434/api/chat")
            .json(&json!({
                "model": model,
                "messages": Self::msgs(prompt, system),
                "stream": false
            }))
            .send().await.map_err(|e| e.to_string())?;

        let status = res.status();
        let body: Value = res.json().await.map_err(|e| e.to_string())?;
        if status.is_success() {
            if let Some(c) = body["message"]["content"].as_str() {
                return Ok(c.to_string());
            }
        }
        Err(format!("Ollama HTTP {}: {}", status, body))
    }

    /// Perplexity Agent API (POST /v1/agent).
    /// `preset_or_model`: un preset ("fast"|"low"|"medium"|"high") o un model ID directo ("openai/gpt-5.6-sol", etc.)
    /// Si empieza con un nombre de proveedor conocido (openai/, anthropic/, google/, xai/) se trata como model.
    /// En caso contrario se envía como preset.
    pub async fn perplexity(&self, prompt: &str, preset_or_model: &str, system: &str) -> Result<String, String> {
        if !self.config.enable_perplexity { return Err("Perplexity is disabled. Set enable_perplexity: true in config.json".into()); }
        let key = self.config.perplexity_api_key.as_deref().ok_or("Sin perplexity_api_key en config.json")?;

        // Decide si es preset o model directo
        let known_providers = ["openai/", "anthropic/", "google/", "xai/", "meta/", "mistral/", "cohere/"];
        let is_model = known_providers.iter().any(|p| preset_or_model.starts_with(p));

        let payload = if is_model {
            let mut m = json!({
                "model": preset_or_model,
                "input": prompt,
                "tools": [{"type": "web_search"}]
            });
            if !system.is_empty() {
                m["instructions"] = json!(system);
            }
            m
        } else {
            // preset
            let preset = if preset_or_model.trim().is_empty() { "fast" } else { preset_or_model.trim() };
            let mut m = json!({
                "preset": preset,
                "input": prompt
            });
            if !system.is_empty() {
                m["instructions"] = json!(system);
            }
            m
        };

        let res = self.client.post("https://api.perplexity.ai/v1/agent")
            .bearer_auth(key)
            .header("User-Agent", "Antigravity-MultiMCP/2.1")
            .json(&payload)
            .send().await.map_err(|e| e.to_string())?;

        let status = res.status();
        let body: Value = res.json().await.map_err(|e| e.to_string())?;

        if status.is_success() {
            // Recorre el array output buscando type=="message" -> content[].type=="output_text"
            let mut texts: Vec<String> = vec![];
            if let Some(output) = body["output"].as_array() {
                for item in output {
                    if item["type"].as_str() == Some("message") {
                        if let Some(contents) = item["content"].as_array() {
                            for c in contents {
                                if c["type"].as_str() == Some("output_text") {
                                    if let Some(t) = c["text"].as_str() {
                                        texts.push(t.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !texts.is_empty() {
                return Ok(texts.join("\n\n"));
            }
            return Err(format!("Perplexity: respuesta sin output_text. Body: {}", body));
        }
        Err(format!("Perplexity HTTP {}: {}", status, body))
    }
}
