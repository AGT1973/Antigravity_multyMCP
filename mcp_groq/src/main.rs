use mcp_core::server::{ProviderConfig, run_server};

struct GroqProvider;

impl ProviderConfig for GroqProvider {
    fn name(&self) -> &str {
        "reason_with_groq"
    }

    fn description(&self) -> &str {
        "Delegate to Groq for extremely fast, low latency LPU inference."
    }

    fn env_prefix(&self) -> &str {
        "GROQ_API_KEY"
    }

    fn api_url(&self) -> &str {
        "https://api.groq.com/openai/v1/chat/completions"
    }

    fn default_model(&self) -> &str {
        "openai/gpt-oss-120b"
    }
}

#[tokio::main]
async fn main() {
    run_server(GroqProvider).await;
}
