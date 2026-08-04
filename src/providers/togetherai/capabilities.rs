//! Capabilities for togetherai models.
//!
//! This module defines model types and their capabilities for togetherai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::togetherai::TogetherAI;

model_capabilities! {
    provider: TogetherAI,
    models: {
        LiquidaiLfm224bA2b {
            model_name: "LiquidAI/LFM2-24B-A2B",
            constructor_name: liquidai_lfm2_24b_a2b,
            display_name: "LFM2-24B-A2B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MinimaxaiMinimaxM27 {
            model_name: "MiniMaxAI/MiniMax-M2.7",
            constructor_name: minimaxai_minimax_m2_7,
            display_name: "MiniMax-M2.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxaiMinimaxM3 {
            model_name: "MiniMaxAI/MiniMax-M3",
            constructor_name: minimaxai_minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen257bInstructTurbo {
            model_name: "Qwen/Qwen2.5-7B-Instruct-Turbo",
            constructor_name: qwen_qwen2_5_7b_instruct_turbo,
            display_name: "Qwen 2.5 7B Instruct Turbo",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen359b {
            model_name: "Qwen/Qwen3.5-9B",
            constructor_name: qwen_qwen3_5_9b,
            display_name: "Qwen3.5 9B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen36Plus {
            model_name: "Qwen/Qwen3.6-Plus",
            constructor_name: qwen_qwen3_6_plus,
            display_name: "Qwen3.6 Plus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen37Max {
            model_name: "Qwen/Qwen3.7-Max",
            constructor_name: qwen_qwen3_7_max,
            display_name: "Qwen3.7 Max",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepcogitoCogitoV21671b {
            model_name: "deepcogito/cogito-v2-1-671b",
            constructor_name: deepcogito_cogito_v2_1_671b,
            display_name: "Cogito v2.1 671B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekV4Pro {
            model_name: "deepseek-ai/DeepSeek-V4-Pro",
            constructor_name: deepseek_ai_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma3nE4bIt {
            model_name: "google/gemma-3n-E4B-it",
            constructor_name: google_gemma_3n_e4b_it,
            display_name: "Gemma 3N E4B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31B-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma 4 31B Instruct",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama3370bInstructTurbo {
            model_name: "meta-llama/Llama-3.3-70B-Instruct-Turbo",
            constructor_name: meta_llama_llama_3_3_70b_instruct_turbo,
            display_name: "Llama 3.3 70B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaMetaLlama38bInstructLite {
            model_name: "meta-llama/Meta-Llama-3-8B-Instruct-Lite",
            constructor_name: meta_llama_meta_llama_3_8b_instruct_lite,
            display_name: "Meta Llama 3 8B Instruct Lite",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MoonshotaiKimiK26 {
            model_name: "moonshotai/Kimi-K2.6",
            constructor_name: moonshotai_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiK27Code {
            model_name: "moonshotai/Kimi-K2.7-Code",
            constructor_name: moonshotai_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK3 {
            model_name: "moonshotai/Kimi-K3",
            constructor_name: moonshotai_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NvidiaNemotron3Ultra550bA55b {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b,
            display_name: "Nemotron 3 Ultra 550B A55B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20b {
            model_name: "openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PearlAiGemma431bIt {
            model_name: "pearl-ai/gemma-4-31b-it",
            constructor_name: pearl_ai_gemma_4_31b_it,
            display_name: "Pearl AI Gemma 4 31B Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ThinkingmachinesInkling {
            model_name: "thinkingmachines/Inkling",
            constructor_name: thinkingmachines_inkling,
            display_name: "Inkling",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm52 {
            model_name: "zai-org/GLM-5.2",
            constructor_name: zai_org_glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
