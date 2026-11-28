//! Capabilities for unbiased models.
//!
//! This module defines model types and their capabilities for unbiased providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::unbiased::Unbiased;

model_capabilities! {
    provider: Unbiased,
    models: {
        Pareto {
            model_name: "pareto",
            constructor_name: pareto,
            display_name: "Pareto 26.10",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
