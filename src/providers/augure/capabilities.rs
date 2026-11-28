//! Capabilities for augure models.
//!
//! This module defines model types and their capabilities for augure providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::augure::Augure;

model_capabilities! {
    provider: Augure,
    models: {
        OpenGlm53 {
            model_name: "open-glm-5-3",
            constructor_name: open_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenQwen38FlashNext {
            model_name: "open-qwen-3.8-flash-next",
            constructor_name: open_qwen_3_8_flash_next,
            display_name: "Qwen3.8 Flash Next",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ossington41 {
            model_name: "ossington-4-1",
            constructor_name: ossington_4_1,
            display_name: "Ossington 4.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ossington5 {
            model_name: "ossington-5",
            constructor_name: ossington_5,
            display_name: "Ossington 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Rosedale1 {
            model_name: "rosedale-1",
            constructor_name: rosedale_1,
            display_name: "Rosedale One",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Tofino3 {
            model_name: "tofino-3",
            constructor_name: tofino_3,
            display_name: "Tofino 3",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
