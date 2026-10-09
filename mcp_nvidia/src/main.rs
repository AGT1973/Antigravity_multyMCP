use mcp_core::server::{ProviderConfig, run_server};

struct NvidiaProvider;

impl ProviderConfig for NvidiaProvider {
    fn name(&self) -> &str {
        "reason_with_nvidia_nim"
    }

    fn description(&self) -> &str {
        "Delegate to Nvidia NIM for deep reasoning, GPU optimized inference and parallel code processing. Use when task requires math or advanced system analysis."
    }

    fn env_prefix(&self) -> &str {
        "NVIDIA_API_KEY"
    }

    fn api_url(&self) -> &str {
        "https://integrate.api.nvidia.com/v1/chat/completions"
    }

    fn default_model(&self) -> &str {
        "deepseek-ai/deepseek-v4-flash-0731"
    }
}

#[tokio::main]
async fn main() {
    run_server(NvidiaProvider).await;
}
