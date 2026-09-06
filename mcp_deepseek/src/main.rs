use mcp_core::server::{ProviderConfig, run_server};

struct DeepseekProvider;

impl ProviderConfig for DeepseekProvider {
    fn name(&self) -> &str {
        "reason_with_deepseek"
    }

    fn description(&self) -> &str {
        "Delegate to DeepSeek API directly (no OpenRouter) for deep reasoning."
    }

    fn env_prefix(&self) -> &str {
        "DEEPSEEK_API_KEY"
    }

    fn api_url(&self) -> &str {
        "https://api.deepseek.com/chat/completions"
    }

    fn default_model(&self) -> &str {
        "deepseek-chat"
    }
}

#[tokio::main]
async fn main() {
    run_server(DeepseekProvider).await;
}
