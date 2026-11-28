//! Capabilities for aistupidlevel models.
//!
//! This module defines model types and their capabilities for aistupidlevel providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::aistupidlevel::Aistupidlevel;

model_capabilities! {
    provider: Aistupidlevel,
    models: {
        Auto {
            model_name: "auto",
            constructor_name: auto,
            display_name: "Auto (account strategy)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AutoBest {
            model_name: "auto-best",
            constructor_name: auto_best,
            display_name: "Auto Best",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AutoCoding {
            model_name: "auto-coding",
            constructor_name: auto_coding,
            display_name: "Auto Coding",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AutoFastestQuality {
            model_name: "auto-fastest-quality",
            constructor_name: auto_fastest_quality,
            display_name: "Auto Fastest Quality",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AutoReasoning {
            model_name: "auto-reasoning",
            constructor_name: auto_reasoning,
            display_name: "Auto Reasoning",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AutoTooling {
            model_name: "auto-tooling",
            constructor_name: auto_tooling,
            display_name: "Auto Tool Use",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AutoValue {
            model_name: "auto-value",
            constructor_name: auto_value,
            display_name: "Auto Value",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AutoValueCoding {
            model_name: "auto-value-coding",
            constructor_name: auto_value_coding,
            display_name: "Auto Value Coding",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AutoValueReasoning {
            model_name: "auto-value-reasoning",
            constructor_name: auto_value_reasoning,
            display_name: "Auto Value Reasoning",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AutoValueTooling {
            model_name: "auto-value-tooling",
            constructor_name: auto_value_tooling,
            display_name: "Auto Value Tool Use",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
