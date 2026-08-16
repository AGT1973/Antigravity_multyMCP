use mcp_core::server::{ProviderConfig, run_server};

struct KimiProvider;

impl ProviderConfig for KimiProvider {
    fn name(&self) -> &str {
        "reason_with_kimi"
    }

    fn description(&self) -> &str {
        "Delegate to Moonshot Kimi for long context Chinese/English reasoning."
    }

    fn env_prefix(&self) -> &str {
        "KIMI_API_KEY"
    }

    fn api_url(&self) -> &str {
        "https://api.moonshot.cn/v1/chat/completions"
    }

    fn default_model(&self) -> &str {
        "moonshot-v1-128k"
    }
}

#[tokio::main]
async fn main() {
    run_server(KimiProvider).await;
}
