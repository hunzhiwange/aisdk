//! Capabilities for byteplus models.
//!
//! This module defines model types and their capabilities for byteplus providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::byteplus::Byteplus;

model_capabilities! {
    provider: Byteplus,
    models: {
        DeepseekV41Flash260910 {
            model_name: "deepseek-v4-1-flash-260910",
            constructor_name: deepseek_v4_1_flash_260910,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash261017 {
            model_name: "deepseek-v4-1-flash-261017",
            constructor_name: deepseek_v4_1_flash_261017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41FlashEconomy261017 {
            model_name: "deepseek-v4-1-flash-economy-261017",
            constructor_name: deepseek_v4_1_flash_economy_261017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Flash260425 {
            model_name: "deepseek-v4-flash-260425",
            constructor_name: deepseek_v4_flash_260425,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4FlashGa260731 {
            model_name: "deepseek-v4-flash-ga-260731",
            constructor_name: deepseek_v4_flash_ga_260731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Pro260425 {
            model_name: "deepseek-v4-pro-260425",
            constructor_name: deepseek_v4_pro_260425,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4ProGa260813 {
            model_name: "deepseek-v4-pro-ga-260813",
            constructor_name: deepseek_v4_pro_ga_260813,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DolaSeed21Turbo260628 {
            model_name: "dola-seed-2-1-turbo-260628",
            constructor_name: dola_seed_2_1_turbo_260628,
            display_name: "Seed 2.1 Turbo",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm52260617 {
            model_name: "glm-5-2-260617",
            constructor_name: glm_5_2_260617,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Flash260828 {
            model_name: "glm-5-3-flash-260828",
            constructor_name: glm_5_3_flash_260828,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Seed20CodePreview260328 {
            model_name: "seed-2-0-code-preview-260328",
            constructor_name: seed_2_0_code_preview_260328,
            display_name: "Seed 2.0 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Seed20Lite260428 {
            model_name: "seed-2-0-lite-260428",
            constructor_name: seed_2_0_lite_260428,
            display_name: "Seed 2.0 Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Seed20Mini260428 {
            model_name: "seed-2-0-mini-260428",
            constructor_name: seed_2_0_mini_260428,
            display_name: "Seed 2.0 Mini",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Seed20Pro260328 {
            model_name: "seed-2-0-pro-260328",
            constructor_name: seed_2_0_pro_260328,
            display_name: "Seed 2.0 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
