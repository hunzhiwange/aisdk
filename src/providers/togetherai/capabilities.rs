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
        MinimaxaiMinimaxM31Flash {
            model_name: "MiniMaxAI/MiniMax-M3.1-Flash",
            constructor_name: minimaxai_minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        PrismMlTernaryBonsai27b {
            model_name: "Prism-ML/Ternary-Bonsai-27B",
            constructor_name: prism_ml_ternary_bonsai_27b,
            display_name: "Ternary Bonsai 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen359b {
            model_name: "Qwen/Qwen3.5-9B",
            constructor_name: qwen_qwen3_5_9b,
            display_name: "Qwen3.5 9B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen36Plus {
            model_name: "Qwen/Qwen3.6-Plus",
            constructor_name: qwen_qwen3_6_plus,
            display_name: "Qwen3.6 Plus",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen37Max {
            model_name: "Qwen/Qwen3.7-Max",
            constructor_name: qwen_qwen3_7_max,
            display_name: "Qwen3.7 Max",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen37Plus {
            model_name: "Qwen/Qwen3.7-Plus",
            constructor_name: qwen_qwen3_7_plus,
            display_name: "Qwen3.7 Plus",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen38Flash {
            model_name: "Qwen/Qwen3.8-Flash",
            constructor_name: qwen_qwen3_8_flash,
            display_name: "Qwen3.8 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen4Max {
            model_name: "Qwen/Qwen4-Max",
            constructor_name: qwen_qwen4_max,
            display_name: "Qwen4 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
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
        DeepseekAiDeepseekV41Flash1017Fast {
            model_name: "deepseek-ai/DeepSeek-V4.1-Flash-1017-fast",
            constructor_name: deepseek_ai_deepseek_v4_1_flash_1017_fast,
            display_name: "DeepSeek V4.1 Flash 1017 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ElorianLabsGlimpse10 {
            model_name: "elorian-labs/glimpse-1.0",
            constructor_name: elorian_labs_glimpse_1_0,
            display_name: "Glimpse 1.0",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MetaModelsMuseGlimmer30b {
            model_name: "meta-models/Muse-Glimmer-30B",
            constructor_name: meta_models_muse_glimmer_30b,
            display_name: "Muse Glimmer 30B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaModelsMuseSparkOpen816bA42b {
            model_name: "meta-models/Muse-Spark-Open-816B-A42B",
            constructor_name: meta_models_muse_spark_open_816b_a42b,
            display_name: "Muse Spark Open",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiK3 {
            model_name: "moonshotai/Kimi-K3",
            constructor_name: moonshotai_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK31Pro {
            model_name: "moonshotai/Kimi-K3.1-Pro",
            constructor_name: moonshotai_kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNvidiaNemotron35SuperVl {
            model_name: "nvidia/NVIDIA-Nemotron-3.5-Super-VL",
            constructor_name: nvidia_nvidia_nemotron_3_5_super_vl,
            display_name: "Nemotron 3.5 Super VL 120B A12B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ReflectionBeam10501bA23b {
            model_name: "reflection/Beam-1.0-501B-A23B",
            constructor_name: reflection_beam_1_0_501b_a23b,
            display_name: "Beam 1.0 501B A23B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RicursiveAiRipple65bPreview {
            model_name: "ricursive-ai/Ripple-65B-Preview",
            constructor_name: ricursive_ai_ripple_65b_preview,
            display_name: "Ripple 65B Preview",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TencentHy4 {
            model_name: "tencent/Hy4",
            constructor_name: tencent_hy4,
            display_name: "Hy4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInkling2Small {
            model_name: "thinkingmachines/Inkling-2-Small",
            constructor_name: thinkingmachines_inkling_2_small,
            display_name: "Inkling 2 Small",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm52 {
            model_name: "zai-org/GLM-5.2",
            constructor_name: zai_org_glm_5_2,
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
