//! Capabilities for graphn models.
//!
//! This module defines model types and their capabilities for graphn providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::graphn::Graphn;

model_capabilities! {
    provider: Graphn,
    models: {
        DeepseekV41Flash {
            model_name: "deepseek-v4.1-flash",
            constructor_name: deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma4E4b {
            model_name: "gemma-4-e4b",
            constructor_name: gemma_4_e4b,
            display_name: "Gemma 4 E4B",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53 {
            model_name: "glm-5.3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen380b {
            model_name: "qwen3-80b",
            constructor_name: qwen3_80b,
            display_name: "Qwen3 80B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Vl {
            model_name: "qwen3-vl",
            constructor_name: qwen3_vl,
            display_name: "Qwen3 VL",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3827b {
            model_name: "qwen3.8-27b",
            constructor_name: qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
