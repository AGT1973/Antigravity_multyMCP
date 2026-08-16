use mcp_core::server::{ProviderConfig, run_server};

struct HfProvider;

impl ProviderConfig for HfProvider {
    fn name(&self) -> &str {
        "reason_with_hf"
    }

    fn description(&self) -> &str {
        "Delegate to HuggingFace Serverless inference."
    }

    fn env_prefix(&self) -> &str {
        "HF_API_KEY"
    }

    fn api_url(&self) -> &str {
        "https://api-inference.huggingface.co/v1/chat/completions"
    }

    fn default_model(&self) -> &str {
        "meta-llama/Meta-Llama-3-70B-Instruct"
    }
}

#[tokio::main]
async fn main() {
    run_server(HfProvider).await;
}
