//! Capabilities for phase models.
//!
//! This module defines model types and their capabilities for phase providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::phase::Phase;

model_capabilities! {
    provider: Phase,
    models: {
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Coder480b {
            model_name: "qwen3-coder-480b",
            constructor_name: qwen3_coder_480b,
            display_name: "Qwen3-Coder 480B-A35B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
