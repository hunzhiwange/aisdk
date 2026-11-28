//! Capabilities for anonrouter models.
//!
//! This module defines model types and their capabilities for anonrouter providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::anonrouter::Anonrouter;

model_capabilities! {
    provider: Anonrouter,
    models: {
        DeepseekDeepseekV3 {
            model_name: "deepseek/deepseek-v3",
            constructor_name: deepseek_deepseek_v3,
            display_name: "DeepSeek-V3",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma312bIt {
            model_name: "google/gemma-3-12b-it",
            constructor_name: google_gemma_3_12b_it,
            display_name: "Gemma 3 12B IT",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma327bIt {
            model_name: "google/gemma-3-27b-it",
            constructor_name: google_gemma_3_27b_it,
            display_name: "Gemma 3 27B IT",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
