//! Capabilities for watt models.
//!
//! This module defines model types and their capabilities for watt providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::watt::Watt;

model_capabilities! {
    provider: Watt,
    models: {
        DeepseekV4Flash {
            model_name: "DeepSeek-V4-Flash",
            constructor_name: deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Flash0731 {
            model_name: "DeepSeek-V4-Flash-0731",
            constructor_name: deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Pro {
            model_name: "DeepSeek-V4-Pro",
            constructor_name: deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash {
            model_name: "DeepSeek-V4.1-Flash",
            constructor_name: deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash1017 {
            model_name: "DeepSeek-V4.1-Flash-1017",
            constructor_name: deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm51 {
            model_name: "GLM-5.1",
            constructor_name: glm_5_1,
            display_name: "GLM-5.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52 {
            model_name: "GLM-5.2",
            constructor_name: glm_5_2,
            display_name: "GLM-5.2",
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
        Hy4Preview {
            model_name: "Hy4-preview",
            constructor_name: hy4_preview,
            display_name: "Hy4 preview",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK25 {
            model_name: "Kimi-K2.5",
            constructor_name: kimi_k2_5,
            display_name: "Kimi K2.5",
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
        MinimaxM3 {
            model_name: "MiniMax-M3",
            constructor_name: minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen35122bA10b {
            model_name: "Qwen3.5-122B-A10B",
            constructor_name: qwen3_5_122b_a10b,
            display_name: "Qwen3.5 122B-A10B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3527b {
            model_name: "Qwen3.5-27B",
            constructor_name: qwen3_5_27b,
            display_name: "Qwen3.5 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3535bA3b {
            model_name: "Qwen3.5-35B-A3B",
            constructor_name: qwen3_5_35b_a3b,
            display_name: "Qwen3.5 35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen35397bA17b {
            model_name: "Qwen3.5-397B-A17B",
            constructor_name: qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B-A17B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
