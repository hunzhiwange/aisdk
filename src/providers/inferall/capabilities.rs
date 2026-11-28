//! Capabilities for inferall models.
//!
//! This module defines model types and their capabilities for inferall providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::inferall::Inferall;

model_capabilities! {
    provider: Inferall,
    models: {
        NvidiaNemotron3Ultra550bA55b {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b,
            display_name: "Nemotron 3 Ultra 550B A55B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35Lightning30bA3b {
            model_name: "nvidia/nemotron-3.5-lightning-30b-a3b",
            constructor_name: nvidia_nemotron_3_5_lightning_30b_a3b,
            display_name: "Nemotron 3.5 Lightning 30B A3B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35SuperVl120bA12b {
            model_name: "nvidia/nemotron-3.5-super-vl-120b-a12b",
            constructor_name: nvidia_nemotron_3_5_super_vl_120b_a12b,
            display_name: "Nemotron 3.5 Super VL 120B A12B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        OpenaiGptOss20b {
            model_name: "openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "GPT OSS 20B",
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
