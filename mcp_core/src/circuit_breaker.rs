use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyState {
    pub key: String,
    pub consumed_at: Option<u64>,     // Timestamp UNIX en segundos de cuando se agotó/dio 429
    pub cooldown_seconds: u64,        // Periodo de enfriamiento/recarga (ej: 18000s = 5h, 259200s = 3 días)
    pub is_exhausted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMatrixState {
    pub provider_name: String,
    pub keys: Vec<KeyState>,
    pub default_cooldown_seconds: u64,
}

pub struct KeyMatrix {
    provider_name: String,
    state: ProviderMatrixState,
    state_file_path: PathBuf,
}

impl KeyMatrix {
    pub fn new(env_prefix: &str) -> Self {
        let provider_name = env_prefix.to_lowercase().replace("_api_key", "");
        
        // Determinar periodo de recarga por defecto según el proveedor
        // Groq/Cerebras: ventana de horas (ej: 5 horas = 18,000s)
        // Gemini / Nvidia / OpenRouter free tiers: cuota diaria/semanal (ej: 24h = 86,400s o 72h = 259,200s)
        let default_cooldown = match provider_name.as_str() {
            "groq" | "cerebras" => 5 * 3600,         // 5 horas
            "gemini" | "aistudio" => 24 * 3600,      // 24 horas (cuota diaria)
            "deepseek" => 12 * 3600,                 // 12 horas
            "kimi" | "moonshot" => 24 * 3600,        // 24 horas
            "nvidia" | "nvapi" => 48 * 3600,         // 48 horas
            "sambanova" => 6 * 3600,                 // 6 horas
            "hf" | "huggingface" => 24 * 3600,       // 24 horas
            _ => 18000,                              // 5 horas por defecto
        };

        // Ruta de persistencia de estado
        let mut state_dir = dirs_fallback();
        state_dir.push("matrix_state");
        fs::create_dir_all(&state_dir).ok();
        let state_file_path = state_dir.join(format!("{}_state.json", provider_name));

        // Cargar estado previo si existe
        let mut loaded_state: Option<ProviderMatrixState> = None;
        if state_file_path.exists() {
            if let Ok(mut file) = File::open(&state_file_path) {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    loaded_state = serde_json::from_str(&contents).ok();
                }
            }
        }

        // Cargar claves desde variables de entorno
        let mut raw_keys = Vec::new();
        for i in 1..=60 {
            if let Ok(key) = env::var(format!("{}_{}", env_prefix, i)) {
                let trimmed = key.trim().to_string();
                if !trimmed.is_empty() && !raw_keys.contains(&trimmed) {
                    raw_keys.push(trimmed);
                }
            }
        }
        if raw_keys.is_empty() {
            if let Ok(key) = env::var(env_prefix) {
                let trimmed = key.trim().to_string();
                if !trimmed.is_empty() {
                    raw_keys.push(trimmed);
                }
            }
        }

        // Sincronizar claves cargadas con el estado guardado
        let mut keys_state = Vec::new();
        let current_time = now_secs();

        for k in raw_keys {
            let mut existing = loaded_state.as_ref().and_then(|s| s.keys.iter().find(|ks| ks.key == k).cloned());
            if let Some(mut ks) = existing {
                // Verificar si ya se cumplió el periodo de enfriamiento
                if ks.is_exhausted {
                    if let Some(consumed) = ks.consumed_at {
                        if current_time >= consumed + ks.cooldown_seconds {
                            // Clave recuperada automáticamente tras pasar el periodo
                            ks.is_exhausted = false;
                            ks.consumed_at = None;
                        }
                    }
                }
                keys_state.push(ks);
            } else {
                keys_state.push(KeyState {
                    key: k,
                    consumed_at: None,
                    cooldown_seconds: default_cooldown,
                    is_exhausted: false,
                });
            }
        }

        let mut matrix = Self {
            provider_name: provider_name.clone(),
            state: ProviderMatrixState {
                provider_name,
                keys: keys_state,
                default_cooldown_seconds: default_cooldown,
            },
            state_file_path,
        };

        matrix.reorder_keys();
        matrix.save_state();
        matrix
    }

    /// Reordena continuamente las llaves:
    /// 1. Primero las activas (no agotadas).
    /// 2. Luego las agotadas pero ordenadas por fecha de próxima recarga (las que se recuperan más pronto primero).
    pub fn reorder_keys(&mut self) {
        let current_time = now_secs();
        
        // Auto-recuperar las que ya hayan cumplido el periodo
        for ks in &mut self.state.keys {
            if ks.is_exhausted {
                if let Some(consumed) = ks.consumed_at {
                    if current_time >= consumed + ks.cooldown_seconds {
                        ks.is_exhausted = false;
                        ks.consumed_at = None;
                    }
                }
            }
        }

        self.state.keys.sort_by(|a, b| {
            match (a.is_exhausted, b.is_exhausted) {
                (false, true) => std::cmp::Ordering::Less,
                (true, false) => std::cmp::Ordering::Greater,
                (false, false) => std::cmp::Ordering::Equal,
                (true, true) => {
                    let eta_a = a.consumed_at.unwrap_or(0) + a.cooldown_seconds;
                    let eta_b = b.consumed_at.unwrap_or(0) + b.cooldown_seconds;
                    eta_a.cmp(&eta_b)
                }
            }
        });
    }

    /// Obtiene la llave más óptima disponible
    pub fn get_current_key(&mut self) -> Option<String> {
        self.reorder_keys();
        self.state.keys.first().filter(|k| !k.is_exhausted).map(|k| k.key.clone())
    }

    /// Marca la llave actual como agotada por 429 con timestamp actual y reordena
    pub fn mark_failure(&mut self, is_429: bool) {
        if self.state.keys.is_empty() {
            return;
        }

        if is_429 {
            let current_time = now_secs();
            if let Some(first) = self.state.keys.first_mut() {
                first.is_exhausted = true;
                first.consumed_at = Some(current_time);
            }
            // Reordenar para pasar inmediatamente a la siguiente llave activa
            self.reorder_keys();
            self.save_state();
        }
    }

    pub fn is_available(&mut self) -> bool {
        self.reorder_keys();
        self.state.keys.iter().any(|k| !k.is_exhausted)
    }

    fn save_state(&self) {
        if let Ok(serialized) = serde_json::to_string_pretty(&self.state) {
            fs::write(&self.state_file_path, serialized).ok();
        }
    }
}

fn now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs()
}

fn dirs_fallback() -> PathBuf {
    if let Ok(val) = env::var("POLYDIM_STATE_DIR") {
        PathBuf::from(val)
    } else {
        PathBuf::from(r"E:\POLYDIM_EINSOF")
    }
}
