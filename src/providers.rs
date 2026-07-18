use reqwest::Client;
use serde::{Deserialize, Serialize};
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
    pub ollama_api_key: Option<String>,
    
    // Toggles
    #[serde(default)] pub enable_openrouter: bool,
    #[serde(default)] pub enable_groq: bool,
    #[serde(default)] pub enable_gemini: bool,
    #[serde(default)] pub enable_hf: bool,
    #[serde(default)] pub enable_cerebras: bool,
    #[serde(default)] pub enable_sambanova: bool,
    #[serde(default)] pub enable_ollama: bool,
    #[serde(default)] pub enable_local_ops: bool,
}

pub fn load_config() -> Config {
    // Try current_exe path first, then current_dir
    let mut config_path = PathBuf::from("config.json");
    
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let candidate = parent.join("config.json");
            if candidate.exists() {
                config_path = candidate;
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

    pub async fn openrouter(&self, prompt: &str, system: &str) -> Result<String, String> {
        if !self.config.enable_openrouter { return Err("OpenRouter is disabled".into()); }
        let key = self.config.openrouter_api_key.as_deref().ok_or("Sin openrouter_api_key")?;
        let res = self.client.post("https://openrouter.ai/api/v1/chat/completions")
            .bearer_auth(key)
            .json(&json!({
                "model": "anthropic/claude-3.5-sonnet",
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
        Err(format!("OpenRouter HTTP {}: {}", status, body))
    }

    pub async fn ollama(&self, prompt: &str, system: &str) -> Result<String, String> {
        if !self.config.enable_ollama { return Err("Ollama is disabled".into()); }
        let res = self.client.post("http://localhost:11434/api/chat")
            .json(&json!({
                "model": "deepseek-r1:14b",
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
}
