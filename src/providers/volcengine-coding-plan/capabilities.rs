//! Capabilities for volcengine_coding_plan models.
//!
//! This module defines model types and their capabilities for volcengine_coding_plan providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::volcengine_coding_plan::VolcengineCodingPlan;

model_capabilities! {
    provider: VolcengineCodingPlan,
    models: {
        DeepseekV4Flash {
            model_name: "deepseek-v4-flash",
            constructor_name: deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Pro {
            model_name: "deepseek-v4-pro",
            constructor_name: deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash {
            model_name: "deepseek-v4.1-flash",
            constructor_name: deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41FlashEconomy {
            model_name: "deepseek-v4.1-flash-economy",
            constructor_name: deepseek_v4_1_flash_economy,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DoubaoSeed20Mini {
            model_name: "doubao-seed-2.0-mini",
            constructor_name: doubao_seed_2_0_mini,
            display_name: "Seed 2.0 Mini",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DoubaoSeed21Lite {
            model_name: "doubao-seed-2.1-lite",
            constructor_name: doubao_seed_2_1_lite,
            display_name: "Seed 2.1 Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DoubaoSeed21Pro {
            model_name: "doubao-seed-2.1-pro",
            constructor_name: doubao_seed_2_1_pro,
            display_name: "Seed 2.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DoubaoSeedEvolving {
            model_name: "doubao-seed-evolving",
            constructor_name: doubao_seed_evolving,
            display_name: "Seed Evolving",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm53 {
            model_name: "glm-5.3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Flash {
            model_name: "glm-5.3-flash",
            constructor_name: glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm54 {
            model_name: "glm-5.4",
            constructor_name: glm_5_4,
            display_name: "GLM-5.4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK28Preview {
            model_name: "kimi-k2.8-preview",
            constructor_name: kimi_k2_8_preview,
            display_name: "Kimi K2.8 Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK31Pro {
            model_name: "kimi-k3.1-pro",
            constructor_name: kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
