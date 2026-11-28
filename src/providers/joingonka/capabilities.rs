//! Capabilities for joingonka models.
//!
//! This module defines model types and their capabilities for joingonka providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::joingonka::Joingonka;

model_capabilities! {
    provider: Joingonka,
    models: {
        MinimaxaiMinimaxM27 {
            model_name: "MiniMaxAI/MiniMax-M2.7",
            constructor_name: minimaxai_minimax_m2_7,
            display_name: "MiniMax-M2.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV4Flash0731 {
            model_name: "deepseek-ai/DeepSeek-V4-Flash-0731",
            constructor_name: deepseek_ai_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm53Flash {
            model_name: "zai-org/GLM-5.3-Flash",
            constructor_name: zai_org_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
