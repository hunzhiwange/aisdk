//! Capabilities for isoquant models.
//!
//! This module defines model types and their capabilities for isoquant providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::isoquant::Isoquant;

model_capabilities! {
    provider: Isoquant,
    models: {
        DeepseekV41Flash1017Fast {
            model_name: "deepseek-v4.1-flash-1017-fast",
            constructor_name: deepseek_v4_1_flash_1017_fast,
            display_name: "DeepSeek V4.1 Flash 1017 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Flash {
            model_name: "glm-5.3-flash",
            constructor_name: glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
