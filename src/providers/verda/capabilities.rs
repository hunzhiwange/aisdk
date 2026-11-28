//! Capabilities for verda models.
//!
//! This module defines model types and their capabilities for verda providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::verda::Verda;

model_capabilities! {
    provider: Verda,
    models: {
        KimiK31Pro {
            model_name: "kimi-k3-1-pro",
            constructor_name: kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
