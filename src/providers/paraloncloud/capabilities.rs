//! Capabilities for paraloncloud models.
//!
//! This module defines model types and their capabilities for paraloncloud providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::paraloncloud::Paraloncloud;

model_capabilities! {
    provider: Paraloncloud,
    models: {
        Qwen3827b {
            model_name: "qwen3.8-27b",
            constructor_name: qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
