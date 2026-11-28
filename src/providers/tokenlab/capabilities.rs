//! Capabilities for tokenlab models.
//!
//! This module defines model types and their capabilities for tokenlab providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::tokenlab::Tokenlab;

model_capabilities! {
    provider: Tokenlab,
    models: {
        Grok420NonReasoning {
            model_name: "grok-4.20-non-reasoning",
            constructor_name: grok_4_20_non_reasoning,
            display_name: "Grok 4.20 (Non-Reasoning)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
