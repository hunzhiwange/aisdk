//! Capabilities for chutes models.
//!
//! This module defines model types and their capabilities for chutes providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::chutes::Chutes;

model_capabilities! {
    provider: Chutes,
    models: {
        QwenQwen3527bTee {
            model_name: "Qwen/Qwen3.5-27B-TEE",
            constructor_name: qwen_qwen3_5_27b_tee,
            display_name: "Qwen3.5 27B TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen35397bA17bTee {
            model_name: "Qwen/Qwen3.5-397B-A17B-TEE",
            constructor_name: qwen_qwen3_5_397b_a17b_tee,
            display_name: "Qwen3.5 397B A17B TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3627bTee {
            model_name: "Qwen/Qwen3.6-27B-TEE",
            constructor_name: qwen_qwen3_6_27b_tee,
            display_name: "Qwen3.6 27B TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3824tA95b {
            model_name: "Qwen/Qwen3.8-2.4T-A95B",
            constructor_name: qwen_qwen3_8_2_4t_a95b,
            display_name: "Qwen3.8 2.4T A95B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bTee {
            model_name: "Qwen/Qwen3.8-27B-TEE",
            constructor_name: qwen_qwen3_8_27b_tee,
            display_name: "Qwen3.8 27B TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen38Max0902 {
            model_name: "Qwen/Qwen3.8-Max-0902",
            constructor_name: qwen_qwen3_8_max_0902,
            display_name: "Qwen3.8 Max 0902",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen4Max {
            model_name: "Qwen/Qwen4-Max",
            constructor_name: qwen_qwen4_max,
            display_name: "Qwen4 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DeepseekAiDeepseekV32Tee {
            model_name: "deepseek-ai/DeepSeek-V3.2-TEE",
            constructor_name: deepseek_ai_deepseek_v3_2_tee,
            display_name: "DeepSeek V3.2 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV4Flash0731Tee {
            model_name: "deepseek-ai/DeepSeek-V4-Flash-0731-TEE",
            constructor_name: deepseek_ai_deepseek_v4_flash_0731_tee,
            display_name: "DeepSeek V4 Flash 0731 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV4Pro0813 {
            model_name: "deepseek-ai/DeepSeek-V4-Pro-0813",
            constructor_name: deepseek_ai_deepseek_v4_pro_0813,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV41Flash1017 {
            model_name: "deepseek-ai/DeepSeek-V4.1-Flash-1017",
            constructor_name: deepseek_ai_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma431bTurboTee {
            model_name: "google/gemma-4-31B-turbo-TEE",
            constructor_name: google_gemma_4_31b_turbo_tee,
            display_name: "gemma 4 31B turbo TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK26Tee {
            model_name: "moonshotai/Kimi-K2.6-TEE",
            constructor_name: moonshotai_kimi_k2_6_tee,
            display_name: "Kimi K2.6 TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiK27Code {
            model_name: "moonshotai/Kimi-K2.7-Code",
            constructor_name: moonshotai_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiK3Tee {
            model_name: "moonshotai/Kimi-K3-TEE",
            constructor_name: moonshotai_kimi_k3_tee,
            display_name: "Kimi K3 TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZaiOrgGlm51Tee {
            model_name: "zai-org/GLM-5.1-TEE",
            constructor_name: zai_org_glm_5_1_tee,
            display_name: "GLM 5.1 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm52Fast {
            model_name: "zai-org/GLM-5.2-Fast",
            constructor_name: zai_org_glm_5_2_fast,
            display_name: "GLM 5.2 Fast",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm52Tee {
            model_name: "zai-org/GLM-5.2-TEE",
            constructor_name: zai_org_glm_5_2_tee,
            display_name: "GLM 5.2 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm53 {
            model_name: "zai-org/GLM-5.3",
            constructor_name: zai_org_glm_5_3,
            display_name: "GLM 5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
