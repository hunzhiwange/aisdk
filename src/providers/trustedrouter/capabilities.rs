//! Capabilities for trustedrouter models.
//!
//! This module defines model types and their capabilities for trustedrouter providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::trustedrouter::Trustedrouter;

model_capabilities! {
    provider: Trustedrouter,
    models: {
        Auto {
            model_name: "auto",
            constructor_name: auto,
            display_name: "Auto",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Cheap {
            model_name: "cheap",
            constructor_name: cheap,
            display_name: "Cheap",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        E2e {
            model_name: "e2e",
            constructor_name: e2e,
            display_name: "End-to-End Encrypted",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Fast {
            model_name: "fast",
            constructor_name: fast,
            display_name: "Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Synth {
            model_name: "synth",
            constructor_name: synth,
            display_name: "Synth",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SynthCode {
            model_name: "synth-code",
            constructor_name: synth_code,
            display_name: "Synth Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Zdr {
            model_name: "zdr",
            constructor_name: zdr,
            display_name: "Zero Data Retention",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
