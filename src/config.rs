use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct ClaudeCodePluginConfig {
    pub dist_url: String,
}

impl Default for ClaudeCodePluginConfig {
    fn default() -> Self {
        Self {
            dist_url: "https://storage.googleapis.com/claude-code-dist-86c565f3-f756-42ad-8dfa-d59b1c096819/claude-code-releases".into(),
        }
    }
}
