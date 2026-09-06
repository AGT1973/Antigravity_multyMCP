use mcp_core::server::{ProviderConfig, run_server};

struct QwenProvider;

impl ProviderConfig for QwenProvider {
    fn name(&self) -> &str {
        "reason_with_qwen"
    }

    fn description(&self) -> &str {
        "Delegate to Alibaba Qwen (DashScope) for multilingual reasoning."
    }

    fn env_prefix(&self) -> &str {
        "QWEN_API_KEY"
    }

    fn api_url(&self) -> &str {
        "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions"
    }

    fn default_model(&self) -> &str {
        "qwen-plus"
    }
}

#[tokio::main]
async fn main() {
    run_server(QwenProvider).await;
}
