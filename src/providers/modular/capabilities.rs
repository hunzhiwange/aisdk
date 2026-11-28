//! Capabilities for modular models.
//!
//! This module defines model types and their capabilities for modular providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::modular::Modular;

model_capabilities! {
    provider: Modular,
    models: {
        MinimaxaiMinimaxM31Flash {
            model_name: "MiniMaxAI/MiniMax-M3.1-Flash",
            constructor_name: minimaxai_minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DeepseekDeepseekV41Flash1017 {
            model_name: "deepseek/deepseek-v4.1-flash-1017",
            constructor_name: deepseek_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31b-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma 4 31B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM3 {
            model_name: "minimax/minimax-m3",
            constructor_name: minimax_minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
