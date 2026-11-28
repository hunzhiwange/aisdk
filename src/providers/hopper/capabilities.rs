//! Capabilities for hopper models.
//!
//! This module defines model types and their capabilities for hopper providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::hopper::Hopper;

model_capabilities! {
    provider: Hopper,
    models: {
        DeepseekV41Flash1017 {
            model_name: "deepseek-v4.1-flash-1017",
            constructor_name: deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash1017Abliterated {
            model_name: "deepseek-v4.1-flash-1017-abliterated",
            constructor_name: deepseek_v4_1_flash_1017_abliterated,
            display_name: "DeepSeek V4.1 Flash 1017 Abliterated",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
