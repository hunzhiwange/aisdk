//! Capabilities for frequency models.
//!
//! This module defines model types and their capabilities for frequency providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::frequency::Frequency;

model_capabilities! {
    provider: Frequency,
    models: {
        GoogleGemma426bA4bIt {
            model_name: "google/gemma-4-26b-a4b-it",
            constructor_name: google_gemma_4_26b_a4b_it,
            display_name: "Gemma 4 26B A4B IT",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
