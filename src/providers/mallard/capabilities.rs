//! Capabilities for mallard models.
//!
//! This module defines model types and their capabilities for mallard providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::mallard::Mallard;

model_capabilities! {
    provider: Mallard,
    models: {
        QwenQwen3635bA3b {
            model_name: "qwen/qwen3.6-35b-a3b",
            constructor_name: qwen_qwen3_6_35b_a3b,
            display_name: "Qwen3.6 35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
