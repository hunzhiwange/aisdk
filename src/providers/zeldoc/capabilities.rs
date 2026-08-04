//! Capabilities for zeldoc models.
//!
//! This module defines model types and their capabilities for zeldoc providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::zeldoc::Zeldoc;

model_capabilities! {
    provider: Zeldoc,
    models: {
        ZCode {
            model_name: "z-code",
            constructor_name: z_code,
            display_name: "Z-Code",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
