//! Capabilities for scx_ai models.
//!
//! This module defines model types and their capabilities for scx_ai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::scx_ai::ScxAi;

model_capabilities! {
    provider: ScxAi,
    models: {
        DeepseekV4Flash {
            model_name: "DeepSeek-V4-flash",
            constructor_name: deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Pro {
            model_name: "DeepSeek-V4-pro",
            constructor_name: deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash {
            model_name: "DeepSeek-V4.1-flash",
            constructor_name: deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash1017 {
            model_name: "DeepSeek-V4.1-flash-1017",
            constructor_name: deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52 {
            model_name: "GLM-5.2",
            constructor_name: glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52Fast {
            model_name: "GLM-5.2-Fast",
            constructor_name: glm_5_2_fast,
            display_name: "GLM-5.2 Turbo",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53 {
            model_name: "GLM-5.3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Flash {
            model_name: "GLM-5.3-Flash",
            constructor_name: glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK27Code {
            model_name: "Kimi-K2.7-Code",
            constructor_name: kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK3 {
            model_name: "Kimi-K3",
            constructor_name: kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK31Pro {
            model_name: "Kimi-K3.1-Pro",
            constructor_name: kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MinimaxM27 {
            model_name: "MiniMax-M2.7",
            constructor_name: minimax_m2_7,
            display_name: "MiniMax-M2.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen38Max {
            model_name: "Qwen3.8-Max",
            constructor_name: qwen3_8_max,
            display_name: "Qwen3.8 Max",
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
