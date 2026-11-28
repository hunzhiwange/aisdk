//! Capabilities for stepping_stone models.
//!
//! This module defines model types and their capabilities for stepping_stone providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::stepping_stone::SteppingStone;

model_capabilities! {
    provider: SteppingStone,
    models: {
        BaaiBgeM3 {
            model_name: "BAAI/bge-m3",
            constructor_name: baai_bge_m3,
            display_name: "BGE M3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BaaiBgeRerankerV2M3 {
            model_name: "BAAI/bge-reranker-v2-m3",
            constructor_name: baai_bge_reranker_v2_m3,
            display_name: "BGE Reranker V2 M3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MinimaxaiMinimaxM25 {
            model_name: "MiniMaxAI/MiniMax-M2.5",
            constructor_name: minimaxai_minimax_m2_5,
            display_name: "MiniMax-M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNvidiaNemotron3Super120bA12b {
            model_name: "NVIDIA/NVIDIA-Nemotron-3-Super-120B-A12B",
            constructor_name: nvidia_nvidia_nemotron_3_super_120b_a12b,
            display_name: "Nemotron 3 Super 120B A12B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderNext {
            model_name: "Qwen/Qwen3-Coder-Next",
            constructor_name: qwen_qwen3_coder_next,
            display_name: "Qwen3 Coder Next",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3535bA3bFp8 {
            model_name: "Qwen/Qwen3.5-35B-A3B-FP8",
            constructor_name: qwen_qwen3_5_35b_a3b_fp8,
            display_name: "Qwen3.5 35B-A3B FP8",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3827b {
            model_name: "Qwen/Qwen3.8-27B",
            constructor_name: qwen_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AllenaiOlmocr27b {
            model_name: "allenai/olmOCR-2-7B",
            constructor_name: allenai_olmocr_2_7b,
            display_name: "olmOCR 2 7B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ApertusAiApertusV158b {
            model_name: "apertus-ai/Apertus-v1.5-8B",
            constructor_name: apertus_ai_apertus_v1_5_8b,
            display_name: "Apertus v1.5 8B",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV4Flash0731 {
            model_name: "deepseek-ai/DeepSeek-V4-Flash-0731",
            constructor_name: deepseek_ai_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31B-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma 4 31B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LightonaiLightonocr21b {
            model_name: "lightonai/LightOnOCR-2-1B",
            constructor_name: lightonai_lightonocr_2_1b,
            display_name: "LightOnOCR 2 1B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpendatalabMineru25250912b {
            model_name: "opendatalab/MinerU2.5-2509-1.2B",
            constructor_name: opendatalab_mineru2_5_2509_1_2b,
            display_name: "MinerU2.5 2509 1.2B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm53Flash {
            model_name: "zai-org/GLM-5.3-Flash",
            constructor_name: zai_org_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
