//! Capabilities for aipg models.
//!
//! This module defines model types and their capabilities for aipg providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::aipg::Aipg;

model_capabilities! {
    provider: Aipg,
    models: {
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen38FlashNextNvfp4 {
            model_name: "qwen38-flash-next-nvfp4",
            constructor_name: qwen38_flash_next_nvfp4,
            display_name: "Qwen3.8 Flash Next",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
