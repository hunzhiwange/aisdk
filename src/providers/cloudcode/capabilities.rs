//! Capabilities for cloudcode models.
//!
//! This module defines model types and their capabilities for cloudcode providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::cloudcode::Cloudcode;

model_capabilities! {
    provider: Cloudcode,
    models: {
        DeepseekV4Pro {
            model_name: "deepseek-v4-pro",
            constructor_name: deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash {
            model_name: "deepseek-v4.1-flash",
            constructor_name: deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53 {
            model_name: "glm-5.3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Flash {
            model_name: "glm-5.3-flash",
            constructor_name: glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm5vTurbo {
            model_name: "glm-5v-turbo",
            constructor_name: glm_5v_turbo,
            display_name: "GLM-5V-Turbo",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MistralLarge4 {
            model_name: "mistral-large-4",
            constructor_name: mistral_large_4,
            display_name: "Mistral Large 4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
