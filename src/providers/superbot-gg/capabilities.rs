//! Capabilities for superbot_gg models.
//!
//! This module defines model types and their capabilities for superbot_gg providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::superbot_gg::SuperbotGg;

model_capabilities! {
    provider: SuperbotGg,
    models: {
        Superbot {
            model_name: "superbot",
            constructor_name: superbot,
            display_name: "superbot",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SuperbotFast {
            model_name: "superbot-fast",
            constructor_name: superbot_fast,
            display_name: "superbot-fast",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
