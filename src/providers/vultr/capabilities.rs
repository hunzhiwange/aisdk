//! Capabilities for vultr models.
//!
//! This module defines model types and their capabilities for vultr providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::vultr::Vultr;

model_capabilities! {
    provider: Vultr,
    models: {
        MinimaxaiMinimaxM27 {
            model_name: "MiniMaxAI/MiniMax-M2.7",
            constructor_name: minimaxai_minimax_m2_7,
            display_name: "MiniMax-M2.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen35397bA17b {
            model_name: "Qwen/Qwen3.5-397B-A17B",
            constructor_name: qwen_qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B-A17B",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3627b {
            model_name: "Qwen/Qwen3.6-27B",
            constructor_name: qwen_qwen3_6_27b,
            display_name: "Qwen3.6 27B",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XiaomimimoMimoV25Pro {
            model_name: "XiaomiMiMo/MiMo-V2.5-Pro",
            constructor_name: xiaomimimo_mimo_v2_5_pro,
            display_name: "MiMo-V2.5-Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV4Flash {
            model_name: "deepseek-ai/DeepSeek-V4-Flash",
            constructor_name: deepseek_ai_deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK26 {
            model_name: "moonshotai/Kimi-K2.6",
            constructor_name: moonshotai_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaDeepseekV32Nvfp4 {
            model_name: "nvidia/DeepSeek-V3.2-NVFP4",
            constructor_name: nvidia_deepseek_v3_2_nvfp4,
            display_name: "DeepSeek V3.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3NanoOmni30bA3bReasoningBf16 {
            model_name: "nvidia/Nemotron-3-Nano-Omni-30B-A3B-Reasoning-BF16",
            constructor_name: nvidia_nemotron_3_nano_omni_30b_a3b_reasoning_bf16,
            display_name: "NVIDIA Nemotron 3 Nano Omni",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronCascade230bA3b {
            model_name: "nvidia/Nemotron-Cascade-2-30B-A3B",
            constructor_name: nvidia_nemotron_cascade_2_30b_a3b,
            display_name: "NVIDIA Nemotron Cascade 2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm52Fp8 {
            model_name: "zai-org/GLM-5.2-FP8",
            constructor_name: zai_org_glm_5_2_fp8,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
