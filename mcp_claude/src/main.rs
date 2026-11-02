use mcp_core::server::{ProviderConfig, run_server};

struct ClaudeProvider;

impl ProviderConfig for ClaudeProvider {
    fn name(&self) -> &str {
        "reason_with_claude"
    }

    fn description(&self) -> &str {
        "Delegate to Anthropic Claude (Paid Tier USD 15.01 - claude-sonnet-4-6 / claude-opus-4-7) for formal Red-Team and multi-model arbitration."
    }

    fn env_prefix(&self) -> &str {
        "CLAUDE_API_KEY"
    }

    fn api_url(&self) -> &str {
        "https://api.anthropic.com/v1/chat/completions"
    }

    fn default_model(&self) -> &str {
        "claude-sonnet-4-6"
    }
}

#[tokio::main]
async fn main() {
    run_server(ClaudeProvider).await;
}
