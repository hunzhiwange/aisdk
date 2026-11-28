//! Capabilities for confidential_ai models.
//!
//! This module defines model types and their capabilities for confidential_ai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::confidential_ai::ConfidentialAi;

model_capabilities! {
    provider: ConfidentialAi,
    models: {
        DeepseekAiDeepseekV4Flash0731 {
            model_name: "deepseek-ai/DeepSeek-V4-Flash-0731",
            constructor_name: deepseek_ai_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
