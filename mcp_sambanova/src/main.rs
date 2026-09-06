use mcp_core::server::{ProviderConfig, run_server};

struct SambanovaProvider;

impl ProviderConfig for SambanovaProvider {
    fn name(&self) -> &str {
        "reason_with_sambanova"
    }

    fn description(&self) -> &str {
        "Delegate to SambaNova for high-throughput SN40L RDU inference."
    }

    fn env_prefix(&self) -> &str {
        "SAMBANOVA_API_KEY"
    }

    fn api_url(&self) -> &str {
        "https://api.sambanova.ai/v1/chat/completions"
    }

    fn default_model(&self) -> &str {
        "Meta-Llama-3.3-70B-Instruct"
    }
}

#[tokio::main]
async fn main() {
    run_server(SambanovaProvider).await;
}
