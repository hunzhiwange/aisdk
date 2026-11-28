//! Capabilities for fiftyx models.
//!
//! This module defines model types and their capabilities for fiftyx providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::fiftyx::Fiftyx;

model_capabilities! {
    provider: Fiftyx,
    models: {
        DeepseekAiDeepseekV41Flash1017 {
            model_name: "deepseek-ai/DeepSeek-V4.1-Flash-1017",
            constructor_name: deepseek_ai_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
