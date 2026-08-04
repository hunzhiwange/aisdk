//! Capabilities for hetzner models.
//!
//! This module defines model types and their capabilities for hetzner providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::hetzner::Hetzner;

model_capabilities! {
    provider: Hetzner,
    models: {
        QwenQwen3635bA3bFp8 {
            model_name: "Qwen/Qwen3.6-35B-A3B-FP8",
            constructor_name: qwen_qwen3_6_35b_a3b_fp8,
            display_name: "Qwen3.6 35B A3B FP8",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
