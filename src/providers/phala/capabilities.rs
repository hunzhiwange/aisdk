//! Capabilities for phala models.
//!
//! This module defines model types and their capabilities for phala providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::phala::Phala;

model_capabilities! {
    provider: Phala,
    models: {
        DeepseekDeepseekV32 {
            model_name: "deepseek/deepseek-v3.2",
            constructor_name: deepseek_deepseek_v3_2,
            display_name: "DeepSeek V3.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash0731 {
            model_name: "deepseek/deepseek-v4-flash-0731",
            constructor_name: deepseek_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama3370bInstruct {
            model_name: "meta-llama/llama-3.3-70b-instruct",
            constructor_name: meta_llama_llama_3_3_70b_instruct,
            display_name: "Llama-3.3-70B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK26 {
            model_name: "moonshotai/kimi-k2.6",
            constructor_name: moonshotai_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK3 {
            model_name: "moonshotai/kimi-k3",
            constructor_name: moonshotai_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen257bInstruct {
            model_name: "qwen/qwen-2.5-7b-instruct",
            constructor_name: qwen_qwen_2_5_7b_instruct,
            display_name: "Qwen2.5 7B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl30bA3bInstruct {
            model_name: "qwen/qwen3-vl-30b-a3b-instruct",
            constructor_name: qwen_qwen3_vl_30b_a3b_instruct,
            display_name: "Qwen3 VL 30B A3B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3527b {
            model_name: "qwen/qwen3.5-27b",
            constructor_name: qwen_qwen3_5_27b,
            display_name: "Qwen3.5 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen35397bA17b {
            model_name: "qwen/qwen3.5-397b-a17b",
            constructor_name: qwen_qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B-A17B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3627b {
            model_name: "qwen/qwen3.6-27b",
            constructor_name: qwen_qwen3_6_27b,
            display_name: "Qwen3.6 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3635bA3b {
            model_name: "qwen/qwen3.6-35b-a3b",
            constructor_name: qwen_qwen3_6_35b_a3b,
            display_name: "Qwen3.6 35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3827b {
            model_name: "qwen/qwen3.8-27b",
            constructor_name: qwen_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZAiGlm51 {
            model_name: "z-ai/glm-5.1",
            constructor_name: z_ai_glm_5_1,
            display_name: "GLM-5.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm52 {
            model_name: "z-ai/glm-5.2",
            constructor_name: z_ai_glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53 {
            model_name: "z-ai/glm-5.3",
            constructor_name: z_ai_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53Flash {
            model_name: "z-ai/glm-5.3-flash",
            constructor_name: z_ai_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
