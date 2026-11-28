//! Capabilities for doubleword models.
//!
//! This module defines model types and their capabilities for doubleword providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::doubleword::Doubleword;

model_capabilities! {
    provider: Doubleword,
    models: {
        QwenQwen3Embedding8b {
            model_name: "Qwen/Qwen3-Embedding-8B",
            constructor_name: qwen_qwen3_embedding_8b,
            display_name: "Qwen3 Embedding 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3Vl235bA22bInstructFp8 {
            model_name: "Qwen/Qwen3-VL-235B-A22B-Instruct-FP8",
            constructor_name: qwen_qwen3_vl_235b_a22b_instruct_fp8,
            display_name: "Qwen3 VL 235B A22B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3535bA3bFp8Dottxt {
            model_name: "Qwen/Qwen3.5-35B-A3B-FP8-dottxt",
            constructor_name: qwen_qwen3_5_35b_a3b_fp8_dottxt,
            display_name: "Qwen3.5 35B A3B dottxt",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3635bA3bFp8 {
            model_name: "Qwen/Qwen3.6-35B-A3B-FP8",
            constructor_name: qwen_qwen3_6_35b_a3b_fp8,
            display_name: "Qwen3.6 35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827bFp8 {
            model_name: "Qwen/Qwen3.8-27B-FP8",
            constructor_name: qwen_qwen3_8_27b_fp8,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV4Flash0731 {
            model_name: "deepseek-ai/DeepSeek-V4-Flash-0731",
            constructor_name: deepseek_ai_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV4Pro0813 {
            model_name: "deepseek-ai/DeepSeek-V4-Pro-0813",
            constructor_name: deepseek_ai_deepseek_v4_pro_0813,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV41Flash {
            model_name: "deepseek-ai/DeepSeek-V4.1-Flash",
            constructor_name: deepseek_ai_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV41Flash1017 {
            model_name: "deepseek-ai/DeepSeek-V4.1-Flash-1017",
            constructor_name: deepseek_ai_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31B-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma 4 31B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaModelsMuseGlimmer30b {
            model_name: "meta-models/Muse-Glimmer-30B",
            constructor_name: meta_models_muse_glimmer_30b,
            display_name: "Muse Glimmer 30B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseSparkOpen {
            model_name: "meta/muse-spark-open",
            constructor_name: meta_muse_spark_open,
            display_name: "Muse Spark Open",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK31Pro {
            model_name: "moonshotai/Kimi-K3.1-Pro",
            constructor_name: moonshotai_kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK3 {
            model_name: "moonshotai/kimi-k3",
            constructor_name: moonshotai_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNvidiaNemotron3Ultra550bA55bNvfp4 {
            model_name: "nvidia/NVIDIA-Nemotron-3-Ultra-550B-A55B-NVFP4",
            constructor_name: nvidia_nvidia_nemotron_3_ultra_550b_a55b_nvfp4,
            display_name: "Nemotron 3 Ultra 550B A55B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm52Fp8 {
            model_name: "zai-org/GLM-5.2-FP8",
            constructor_name: zai_org_glm_5_2_fp8,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm53 {
            model_name: "zai-org/GLM-5.3",
            constructor_name: zai_org_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm53Flash {
            model_name: "zai-org/GLM-5.3-Flash",
            constructor_name: zai_org_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
