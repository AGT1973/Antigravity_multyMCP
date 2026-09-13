use mcp_core::server::{ProviderConfig, run_server};

struct GeminiProvider;

impl ProviderConfig for GeminiProvider {
    fn name(&self) -> &str {
        "reason_with_gemini"
    }

    fn description(&self) -> &str {
        "Delegate to Gemini 3.7 Flash via OpenAI compat endpoint for fast agentic reasoning."
    }

    fn env_prefix(&self) -> &str {
        "GEMINI_API_KEY"
    }

    fn api_url(&self) -> &str {
        "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions"
    }

    fn default_model(&self) -> &str {
        "gemini-3.6-flash"
    }
}

#[tokio::main]
async fn main() {
    run_server(GeminiProvider).await;
}
