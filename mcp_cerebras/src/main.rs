use mcp_core::server::{ProviderConfig, run_server};

struct CerebrasProvider;

impl ProviderConfig for CerebrasProvider {
    fn name(&self) -> &str {
        "reason_with_cerebras"
    }

    fn description(&self) -> &str {
        "Delegate to Cerebras for ultra fast wafer-scale inference."
    }

    fn env_prefix(&self) -> &str {
        "CEREBRAS_API_KEY"
    }

    fn api_url(&self) -> &str {
        "https://api.cerebras.ai/v1/chat/completions"
    }

    fn default_model(&self) -> &str {
        "llama3.1-8b"
    }
}

#[tokio::main]
async fn main() {
    run_server(CerebrasProvider).await;
}
