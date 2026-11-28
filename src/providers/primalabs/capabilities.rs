//! Capabilities for primalabs models.
//!
//! This module defines model types and their capabilities for primalabs providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::primalabs::Primalabs;

model_capabilities! {
    provider: Primalabs,
    models: {
        PrimalabsAiDeepseekV41Flash1017 {
            model_name: "primalabs-ai/DeepSeek-V4.1-Flash-1017",
            constructor_name: primalabs_ai_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PrimalabsAiGlm53Flash {
            model_name: "primalabs-ai/GLM-5.3-Flash",
            constructor_name: primalabs_ai_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PrimalabsAiKimiK31Pro {
            model_name: "primalabs-ai/Kimi-K3.1-Pro",
            constructor_name: primalabs_ai_kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PrimalabsAiMimoV26Pro {
            model_name: "primalabs-ai/MiMo-V2.6-Pro",
            constructor_name: primalabs_ai_mimo_v2_6_pro,
            display_name: "MiMo-V2.6-Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PrimalabsAiNemotron35LightningNvfp4 {
            model_name: "primalabs-ai/Nemotron-3.5-Lightning-NVFP4",
            constructor_name: primalabs_ai_nemotron_3_5_lightning_nvfp4,
            display_name: "Nemotron 3.5 Lightning 30B A3B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PrimalabsAiNemotron35SuperVlNvfp4 {
            model_name: "primalabs-ai/Nemotron-3.5-Super-VL-NVFP4",
            constructor_name: primalabs_ai_nemotron_3_5_super_vl_nvfp4,
            display_name: "Nemotron 3.5 Super VL 120B A12B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
