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
        UmansDeepseekV41Flash {
            model_name: "umans-deepseek-v4.1-flash",
            constructor_name: umans_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UmansDeepseekV41Flash1017 {
            model_name: "umans-deepseek-v4.1-flash-1017",
            constructor_name: umans_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UmansFlash {
            model_name: "umans-flash",
            constructor_name: umans_flash,
            display_name: "Umans Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UmansGlm53 {
            model_name: "umans-glm-5.3",
            constructor_name: umans_glm_5_3,
            display_name: "GLM 5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UmansKimiK31Pro {
            model_name: "umans-kimi-k3.1-pro",
            constructor_name: umans_kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
