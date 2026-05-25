//! Capabilities for vultr models.
//!
//! This module defines model types and their capabilities for vultr providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::vultr::Vultr;

model_capabilities! {
    provider: Vultr,
    models: {
        DeepseekV32 {
            model_name: "DeepSeek-V3.2",
            constructor_name: deepseek_v3_2,
            display_name: "DeepSeek V3.2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm5Fp8 {
            model_name: "GLM-5-FP8",
            constructor_name: glm_5_fp8,
            display_name: "GLM 5 FP8",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK25 {
            model_name: "Kimi-K2.5",
            constructor_name: kimi_k2_5,
            display_name: "Kimi K2 Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM25 {
            model_name: "MiniMax-M2.5",
            constructor_name: minimax_m2_5,
            display_name: "MiniMax M2.5",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
