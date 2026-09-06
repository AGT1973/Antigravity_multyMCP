use mcp_core::server::{ProviderConfig, run_server};

struct GlmProvider;

impl ProviderConfig for GlmProvider {
    fn name(&self) -> &str {
        "reason_with_glm"
    }

    fn description(&self) -> &str {
        "Delegate to Zhipu GLM (Z.ai international) for Chinese/English reasoning."
    }

    fn env_prefix(&self) -> &str {
        "GLM_API_KEY"
    }

    fn api_url(&self) -> &str {
        "https://api.z.ai/api/paas/v4/chat/completions"
    }

    fn default_model(&self) -> &str {
        "glm-5.3"
    }
}

#[tokio::main]
async fn main() {
    run_server(GlmProvider).await;
}
