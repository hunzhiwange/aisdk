//! Capabilities for unrestricted models.
//!
//! This module defines model types and their capabilities for unrestricted providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::unrestricted::Unrestricted;

model_capabilities! {
    provider: Unrestricted,
    models: {
        DeepseekV41FlashUncensored {
            model_name: "deepseek-v4.1-flash-uncensored",
            constructor_name: deepseek_v4_1_flash_uncensored,
            display_name: "DeepSeek V4.1 Flash 1017 Uncensored",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53FlashUncensored {
            model_name: "glm-5.3-flash-uncensored",
            constructor_name: glm_5_3_flash_uncensored,
            display_name: "GLM-5.3 Flash Uncensored",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MimoV26FlashUncensored {
            model_name: "mimo-v2.6-flash-uncensored",
            constructor_name: mimo_v2_6_flash_uncensored,
            display_name: "MiMo V2.6 Flash Uncensored",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3827bUncensored {
            model_name: "qwen3.8-27b-uncensored",
            constructor_name: qwen3_8_27b_uncensored,
            display_name: "Qwen3.8 27B Uncensored",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
