//! Capabilities for jad_ai models.
//!
//! This module defines model types and their capabilities for jad_ai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::jad_ai::JadAi;

model_capabilities! {
    provider: JadAi,
    models: {
        JadAiModel {
            model_name: "jad-ai",
            constructor_name: jad_ai_model,
            display_name: "Jad CoWork",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        JadCode {
            model_name: "jad-code",
            constructor_name: jad_code,
            display_name: "Jad Code",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        JadGo {
            model_name: "jad-go",
            constructor_name: jad_go,
            display_name: "Jad Go",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
