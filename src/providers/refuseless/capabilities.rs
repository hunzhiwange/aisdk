//! Capabilities for refuseless models.
//!
//! This module defines model types and their capabilities for refuseless providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::refuseless::Refuseless;

model_capabilities! {
    provider: Refuseless,
    models: {
        DeepseekV41FlashAbliterated {
            model_name: "deepseek-v4.1-flash-abliterated",
            constructor_name: deepseek_v4_1_flash_abliterated,
            display_name: "DeepSeek V4.1 Flash 1017 Abliterated",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41FlashAbliteratedFree {
            model_name: "deepseek-v4.1-flash-abliterated-free",
            constructor_name: deepseek_v4_1_flash_abliterated_free,
            display_name: "DeepSeek V4.1 Flash 1017 Abliterated Free",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41FlashFast {
            model_name: "deepseek-v4.1-flash-fast",
            constructor_name: deepseek_v4_1_flash_fast,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Abliterated {
            model_name: "glm-5.3-abliterated",
            constructor_name: glm_5_3_abliterated,
            display_name: "GLM 5.3 Abliterated",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53FlashAbliterated {
            model_name: "glm-5.3-flash-abliterated",
            constructor_name: glm_5_3_flash_abliterated,
            display_name: "GLM 5.3 Flash Abliterated",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM31Flash {
            model_name: "minimax-m3.1-flash",
            constructor_name: minimax_m3_1_flash,
            display_name: "MiniMax M3.1 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
