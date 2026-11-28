//! Capabilities for nokeep models.
//!
//! This module defines model types and their capabilities for nokeep providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::nokeep::Nokeep;

model_capabilities! {
    provider: Nokeep,
    models: {
        DeepseekV41Flash1017Uncensored {
            model_name: "deepseek-v4.1-flash-1017-uncensored",
            constructor_name: deepseek_v4_1_flash_1017_uncensored,
            display_name: "DeepSeek V4.1 Flash 1017 Uncensored",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41FlashUncensored {
            model_name: "deepseek-v4.1-flash-uncensored",
            constructor_name: deepseek_v4_1_flash_uncensored,
            display_name: "DeepSeek V4.1 Flash Uncensored",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53FlashUncensored {
            model_name: "glm-5.3-flash-uncensored",
            constructor_name: glm_5_3_flash_uncensored,
            display_name: "GLM 5.3 Flash Uncensored",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
