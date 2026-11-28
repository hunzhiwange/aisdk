//! Capabilities for sambanova models.
//!
//! This module defines model types and their capabilities for sambanova providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::sambanova::Sambanova;

model_capabilities! {
    provider: Sambanova,
    models: {
        Glm53 {
            model_name: "GLM-5.3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM31Flash {
            model_name: "MiniMax-M3.1-Flash",
            constructor_name: minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bIt {
            model_name: "gemma-4-31B-it",
            constructor_name: gemma_4_31b_it,
            display_name: "Gemma 4 31B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
