//! Capabilities for therouter models.
//!
//! This module defines model types and their capabilities for therouter providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::therouter::Therouter;

model_capabilities! {
    provider: Therouter,
    models: {
        OpenaiGpt4oMini {
            model_name: "openai/gpt-4o-mini",
            constructor_name: openai_gpt_4o_mini,
            display_name: "GPT-4o mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
