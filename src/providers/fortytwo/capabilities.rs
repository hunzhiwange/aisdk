//! Capabilities for fortytwo models.
//!
//! This module defines model types and their capabilities for fortytwo providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::fortytwo::Fortytwo;

model_capabilities! {
    provider: Fortytwo,
    models: {
        QwenQwen3827b {
            model_name: "qwen/qwen3.8-27b",
            constructor_name: qwen_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
