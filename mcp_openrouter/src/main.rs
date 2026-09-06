use mcp_core::server::{ProviderConfig, run_server};

struct OpenrouterProvider;

impl ProviderConfig for OpenrouterProvider {
    fn name(&self) -> &str {
        "reason_with_openrouter"
    }

    fn description(&self) -> &str {
        "Delegate to OpenRouter for general LLM routing."
    }

    fn env_prefix(&self) -> &str {
        "OPENROUTER_API_KEY"
    }

    fn api_url(&self) -> &str {
        "https://openrouter.ai/api/v1/chat/completions"
    }

    fn default_model(&self) -> &str {
        "deepseek/deepseek-chat"
    }
}

#[tokio::main]
async fn main() {
    run_server(OpenrouterProvider).await;
}
