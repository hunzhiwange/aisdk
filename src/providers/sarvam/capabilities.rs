//! Capabilities for sarvam models.
//!
//! This module defines model types and their capabilities for sarvam providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::sarvam::Sarvam;

model_capabilities! {
    provider: Sarvam,
    models: {
        Sarvam105b {
            model_name: "sarvam-105b",
            constructor_name: sarvam_105b,
            display_name: "Sarvam-105B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Sarvam30b {
            model_name: "sarvam-30b",
            constructor_name: sarvam_30b,
            display_name: "Sarvam-30B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
