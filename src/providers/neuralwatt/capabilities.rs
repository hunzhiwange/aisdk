//! Capabilities for neuralwatt models.
//!
//! This module defines model types and their capabilities for neuralwatt providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::neuralwatt::Neuralwatt;

model_capabilities! {
    provider: Neuralwatt,
    models: {
        QwenQwen35397bA17bFp8 {
            model_name: "Qwen/Qwen3.5-397B-A17B-FP8",
            constructor_name: qwen_qwen3_5_397b_a17b_fp8,
            display_name: "Qwen3.5 397B A17B FP8",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3635bA3b {
            model_name: "Qwen/Qwen3.6-35B-A3B",
            constructor_name: qwen_qwen3_6_35b_a3b,
            display_name: "Qwen3.6 35B A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Flash {
            model_name: "deepseek-v4-flash",
            constructor_name: deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431b {
            model_name: "gemma-4-31b",
            constructor_name: gemma_4_31b,
            display_name: "Gemma 4 31B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52 {
            model_name: "glm-5.2",
            constructor_name: glm_5_2,
            display_name: "GLM 5.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52Fast {
            model_name: "glm-5.2-fast",
            constructor_name: glm_5_2_fast,
            display_name: "GLM 5.2 Fast",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52Flex {
            model_name: "glm-5.2-flex",
            constructor_name: glm_5_2_flex,
            display_name: "GLM 5.2 Flex",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52Short {
            model_name: "glm-5.2-short",
            constructor_name: glm_5_2_short,
            display_name: "GLM 5.2 Short",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52ShortFast {
            model_name: "glm-5.2-short-fast",
            constructor_name: glm_5_2_short_fast,
            display_name: "GLM 5.2 Short Fast",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52ShortFastFlex {
            model_name: "glm-5.2-short-fast-flex",
            constructor_name: glm_5_2_short_fast_flex,
            display_name: "GLM 5.2 Short Fast Flex",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52ShortFlex {
            model_name: "glm-5.2-short-flex",
            constructor_name: glm_5_2_short_flex,
            display_name: "GLM 5.2 Short Flex",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK25Fast {
            model_name: "kimi-k2.5-fast",
            constructor_name: kimi_k2_5_fast,
            display_name: "Kimi K2.5 Fast",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK26Fast {
            model_name: "kimi-k2.6-fast",
            constructor_name: kimi_k2_6_fast,
            display_name: "Kimi K2.6 Fast",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK26Flex {
            model_name: "kimi-k2.6-flex",
            constructor_name: kimi_k2_6_flex,
            display_name: "Kimi K2.6 Flex",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK27CodeFlex {
            model_name: "kimi-k2.7-code-flex",
            constructor_name: kimi_k2_7_code_flex,
            display_name: "Kimi K2.7 Code Flex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai/Kimi-K2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK26 {
            model_name: "moonshotai/Kimi-K2.6",
            constructor_name: moonshotai_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK27Code {
            model_name: "moonshotai/Kimi-K2.7-Code",
            constructor_name: moonshotai_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen35397bFast {
            model_name: "qwen3.5-397b-fast",
            constructor_name: qwen3_5_397b_fast,
            display_name: "Qwen3.5 397B Fast",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3635bFast {
            model_name: "qwen3.6-35b-fast",
            constructor_name: qwen3_6_35b_fast,
            display_name: "Qwen3.6 35B Fast",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
