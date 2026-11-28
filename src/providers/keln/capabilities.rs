//! Capabilities for keln models.
//!
//! This module defines model types and their capabilities for keln providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::keln::Keln;

model_capabilities! {
    provider: Keln,
    models: {
        DeepseekAiDeepseekV4Pro0813 {
            model_name: "deepseek-ai/deepseek-v4-pro-0813",
            constructor_name: deepseek_ai_deepseek_v4_pro_0813,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV41Flash {
            model_name: "deepseek-ai/deepseek-v4.1-flash",
            constructor_name: deepseek_ai_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV41Flash1017 {
            model_name: "deepseek-ai/deepseek-v4.1-flash-1017",
            constructor_name: deepseek_ai_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31b-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma 4 31B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxaiMinimaxM31Flash {
            model_name: "minimaxai/minimax-m3.1-flash",
            constructor_name: minimaxai_minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK31Pro {
            model_name: "moonshotai/kimi-k3.1-pro",
            constructor_name: moonshotai_kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3Ultra550bA55b {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b,
            display_name: "Nemotron 3 Ultra 550B A55B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827b {
            model_name: "qwen/qwen3.8-27b",
            constructor_name: qwen_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm52 {
            model_name: "zai-org/glm-5.2",
            constructor_name: zai_org_glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm53 {
            model_name: "zai-org/glm-5.3",
            constructor_name: zai_org_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm53Flash {
            model_name: "zai-org/glm-5.3-flash",
            constructor_name: zai_org_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
