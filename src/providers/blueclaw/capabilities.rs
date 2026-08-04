//! Capabilities for blueclaw models.
//!
//! This module defines model types and their capabilities for blueclaw providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::blueclaw::Blueclaw;

model_capabilities! {
    provider: Blueclaw,
    models: {
        QwenQwen3635bA3bFp8 {
            model_name: "Qwen/Qwen3.6-35B-A3B-FP8",
            constructor_name: qwen_qwen3_6_35b_a3b_fp8,
            display_name: "Qwen3.6 35B A3B FP8",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3627b {
            model_name: "Qwen3.6-27B",
            constructor_name: qwen3_6_27b,
            display_name: "Qwen3.6 27B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
