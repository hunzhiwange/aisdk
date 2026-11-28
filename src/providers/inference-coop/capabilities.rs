//! Capabilities for inference_coop models.
//!
//! This module defines model types and their capabilities for inference_coop providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::inference_coop::InferenceCoop;

model_capabilities! {
    provider: InferenceCoop,
    models: {
        GreenptDeepseekV41Flash {
            model_name: "greenpt/deepseek-v4.1-flash",
            constructor_name: greenpt_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GreenptGlm53 {
            model_name: "greenpt/glm-5.3",
            constructor_name: greenpt_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GreenptGlm53Flash {
            model_name: "greenpt/glm-5.3-flash",
            constructor_name: greenpt_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RegoloApertus70b {
            model_name: "regolo/apertus-70b",
            constructor_name: regolo_apertus_70b,
            display_name: "Apertus 1.5 70B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RegoloQwen35122b {
            model_name: "regolo/qwen3.5-122b",
            constructor_name: regolo_qwen3_5_122b,
            display_name: "Qwen3.5-122B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TinfoilDeepseekV41Flash {
            model_name: "tinfoil/deepseek-v4-1-flash",
            constructor_name: tinfoil_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TinfoilGlm53 {
            model_name: "tinfoil/glm-5.3",
            constructor_name: tinfoil_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TinfoilGptOss120b {
            model_name: "tinfoil/gpt-oss-120b",
            constructor_name: tinfoil_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
