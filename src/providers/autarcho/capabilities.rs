//! Capabilities for autarcho models.
//!
//! This module defines model types and their capabilities for autarcho providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::autarcho::Autarcho;

model_capabilities! {
    provider: Autarcho,
    models: {
        Llama323bUncensored {
            model_name: "llama-32-3b-uncensored",
            constructor_name: llama_32_3b_uncensored,
            display_name: "Llama 3.2 3B Uncensored",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
    }
}
