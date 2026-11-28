//! Capabilities for oathbreaker models.
//!
//! This module defines model types and their capabilities for oathbreaker providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::oathbreaker::Oathbreaker;

model_capabilities! {
    provider: Oathbreaker,
    models: {
        OathbreakerGlm53 {
            model_name: "oathbreaker-glm-5.3",
            constructor_name: oathbreaker_glm_5_3,
            display_name: "Oathbreaker GLM-5.3 Abliterated",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OathbreakerGlm53Flash {
            model_name: "oathbreaker-glm-5.3-flash",
            constructor_name: oathbreaker_glm_5_3_flash,
            display_name: "Oathbreaker GLM-5.3 Flash Abliterated",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
