//! Capabilities for cheapllm models.
//!
//! This module defines model types and their capabilities for cheapllm providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::cheapllm::Cheapllm;

model_capabilities! {
    provider: Cheapllm,
    models: {
        DeepseekV41Flash1017 {
            model_name: "deepseek-v4.1-flash-1017",
            constructor_name: deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
