//! Capabilities for entrim models.
//!
//! This module defines model types and their capabilities for entrim providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::entrim::Entrim;

model_capabilities! {
    provider: Entrim,
    models: {
        QwenQwen3827b {
            model_name: "Qwen/Qwen3.8-27B",
            constructor_name: qwen_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DeepseekAiDeepseekV41Flash1017 {
            model_name: "deepseek-ai/DeepSeek-V4.1-Flash-1017",
            constructor_name: deepseek_ai_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm53 {
            model_name: "zai-org/GLM-5.3",
            constructor_name: zai_org_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm53Flash {
            model_name: "zai-org/GLM-5.3-Flash",
            constructor_name: zai_org_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
