//! Capabilities for apiko models.
//!
//! This module defines model types and their capabilities for apiko providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::apiko::Apiko;

model_capabilities! {
    provider: Apiko,
    models: {
        ClaudeOpus55 {
            model_name: "claude-opus-5-5",
            constructor_name: claude_opus_5_5,
            display_name: "Claude Opus 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet55 {
            model_name: "claude-sonnet-5-5",
            constructor_name: claude_sonnet_5_5,
            display_name: "Claude Sonnet 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Pro {
            model_name: "deepseek-v4-pro",
            constructor_name: deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
