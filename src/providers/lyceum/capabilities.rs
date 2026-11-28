//! Capabilities for lyceum models.
//!
//! This module defines model types and their capabilities for lyceum providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::lyceum::Lyceum;

model_capabilities! {
    provider: Lyceum,
    models: {
        DeepseekDeepseekV4Flash0731 {
            model_name: "deepseek/deepseek-v4-flash-0731",
            constructor_name: deepseek_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash {
            model_name: "deepseek/deepseek-v4.1-flash",
            constructor_name: deepseek_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK27Code {
            model_name: "moonshotai/kimi-k2.7-code",
            constructor_name: moonshotai_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK31Pro {
            model_name: "moonshotai/kimi-k3.1-pro",
            constructor_name: moonshotai_kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3824tA95b {
            model_name: "qwen/qwen3.8-2.4t-a95b",
            constructor_name: qwen_qwen3_8_2_4t_a95b,
            display_name: "Qwen3.8 2.4T A95B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827b {
            model_name: "qwen/qwen3.8-27b",
            constructor_name: qwen_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53 {
            model_name: "z-ai/glm-5.3",
            constructor_name: z_ai_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53Flash {
            model_name: "z-ai/glm-5.3-flash",
            constructor_name: z_ai_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
