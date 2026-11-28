//! Capabilities for tokenwatt models.
//!
//! This module defines model types and their capabilities for tokenwatt providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::tokenwatt::Tokenwatt;

model_capabilities! {
    provider: Tokenwatt,
    models: {
        QwenQwen3Coder30bA3bInstruct {
            model_name: "qwen/qwen3-coder-30b-a3b-instruct",
            constructor_name: qwen_qwen3_coder_30b_a3b_instruct,
            display_name: "Qwen3-Coder 30B-A3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
