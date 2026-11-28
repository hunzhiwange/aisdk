//! Capabilities for lithosai models.
//!
//! This module defines model types and their capabilities for lithosai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::lithosai::Lithosai;

model_capabilities! {
    provider: Lithosai,
    models: {
        DeepseekAiDeepseekV41Flash {
            model_name: "deepseek-ai/DeepSeek-V4.1-Flash",
            constructor_name: deepseek_ai_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV41FlashFast {
            model_name: "deepseek-ai/DeepSeek-V4.1-Flash-fast",
            constructor_name: deepseek_ai_deepseek_v4_1_flash_fast,
            display_name: "DeepSeek V4.1 Flash 1017 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV41FlashUltra {
            model_name: "deepseek-ai/DeepSeek-V4.1-Flash-ultra",
            constructor_name: deepseek_ai_deepseek_v4_1_flash_ultra,
            display_name: "DeepSeek V4.1 Flash 1017 Ultra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
