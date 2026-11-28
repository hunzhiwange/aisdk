//! Capabilities for sokkan models.
//!
//! This module defines model types and their capabilities for sokkan providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::sokkan::Sokkan;

model_capabilities! {
    provider: Sokkan,
    models: {
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
        QwenQwen35397bA17b {
            model_name: "qwen/qwen3.5-397b-a17b",
            constructor_name: qwen_qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B-A17B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3635bA3bEu {
            model_name: "qwen/qwen3.6-35b-a3b-eu",
            constructor_name: qwen_qwen3_6_35b_a3b_eu,
            display_name: "Qwen3.6 35B-A3B (EU)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3827b {
            model_name: "qwen/qwen3.8-27b",
            constructor_name: qwen_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
