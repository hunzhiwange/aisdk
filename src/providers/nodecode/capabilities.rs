//! Capabilities for nodecode models.
//!
//! This module defines model types and their capabilities for nodecode providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::nodecode::Nodecode;

model_capabilities! {
    provider: Nodecode,
    models: {
        NodecodeCyberV1 {
            model_name: "nodecode-cyber-v1",
            constructor_name: nodecode_cyber_v1,
            display_name: "Cyber v1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
