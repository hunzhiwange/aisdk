//! Capabilities for umans_ai models.
//!
//! This module defines model types and their capabilities for umans_ai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::umans_ai::UmansAi;

model_capabilities! {
    provider: UmansAi,
    models: {
        UmansCoder {
            model_name: "umans-coder",
            constructor_name: umans_coder,
            display_name: "Umans Coder",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UmansDeepseekV4Flash0731 {
            model_name: "umans-deepseek-v4-flash-0731",
            constructor_name: umans_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UmansFlash {
            model_name: "umans-flash",
            constructor_name: umans_flash,
            display_name: "Umans Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UmansGlm51 {
            model_name: "umans-glm-5.1",
            constructor_name: umans_glm_5_1,
            display_name: "GLM 5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UmansGlm52 {
            model_name: "umans-glm-5.2",
            constructor_name: umans_glm_5_2,
            display_name: "GLM 5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UmansKimiK27 {
            model_name: "umans-kimi-k2.7",
            constructor_name: umans_kimi_k2_7,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        UmansKimiK3 {
            model_name: "umans-kimi-k3",
            constructor_name: umans_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
