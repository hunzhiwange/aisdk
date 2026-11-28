//! Capabilities for munito_eu models.
//!
//! This module defines model types and their capabilities for munito_eu providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::munito_eu::MunitoEu;

model_capabilities! {
    provider: MunitoEu,
    models: {
        AlephAlphaKolibri1 {
            model_name: "aleph-alpha/kolibri-1",
            constructor_name: aleph_alpha_kolibri_1,
            display_name: "Kolibri 1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV4Flash {
            model_name: "deepseek-ai/deepseek-v4-flash",
            constructor_name: deepseek_ai_deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827b {
            model_name: "qwen/qwen3.8-27b",
            constructor_name: qwen_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZaiOrgGlm53Flash {
            model_name: "zai-org/glm-5.3-flash",
            constructor_name: zai_org_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
