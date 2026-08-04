//! Capabilities for llmtr models.
//!
//! This module defines model types and their capabilities for llmtr providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::llmtr::Llmtr;

model_capabilities! {
    provider: Llmtr,
    models: {
        Gemma4 {
            model_name: "gemma-4",
            constructor_name: gemma_4,
            display_name: "Gemma 4",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Magibu11bV8 {
            model_name: "magibu-11b-v8",
            constructor_name: magibu_11b_v8,
            display_name: "Magibu 11B v8",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Medgemma4b {
            model_name: "medgemma-4b",
            constructor_name: medgemma_4b,
            display_name: "MedGemma 4B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3635b {
            model_name: "qwen3-6-35b",
            constructor_name: qwen3_6_35b,
            display_name: "Qwen3.6 35B-A3B",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Sincap {
            model_name: "sincap",
            constructor_name: sincap,
            display_name: "Sincap",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Trendyol7b {
            model_name: "trendyol-7b",
            constructor_name: trendyol_7b,
            display_name: "Trendyol 7B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
    }
}
