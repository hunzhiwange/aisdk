//! Capabilities for humain models.
//!
//! This module defines model types and their capabilities for humain providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::humain::Humain;

model_capabilities! {
    provider: Humain,
    models: {
        HumainM3Preview {
            model_name: "humain-m3-preview",
            constructor_name: humain_m3_preview,
            display_name: "HUMAIN M3 Preview",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HumainM3ResearchPreview {
            model_name: "humain-m3-research-preview",
            constructor_name: humain_m3_research_preview,
            display_name: "HUMAIN M3 Research Preview",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
