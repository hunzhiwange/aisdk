//! Capabilities for zenifra models.
//!
//! This module defines model types and their capabilities for zenifra providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::zenifra::Zenifra;

model_capabilities! {
    provider: Zenifra,
    models: {
        ZenifraDeepseekV4Flash0731 {
            model_name: "zenifra/deepseek-v4-flash-0731",
            constructor_name: zenifra_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZenifraDeepseekV4Pro {
            model_name: "zenifra/deepseek-v4-pro",
            constructor_name: zenifra_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZenifraGlm51 {
            model_name: "zenifra/glm-5.1",
            constructor_name: zenifra_glm_5_1,
            display_name: "GLM-5.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZenifraGlm52 {
            model_name: "zenifra/glm-5.2",
            constructor_name: zenifra_glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZenifraKimiK25 {
            model_name: "zenifra/kimi-k2.5",
            constructor_name: zenifra_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZenifraKimiK27Code {
            model_name: "zenifra/kimi-k2.7-code",
            constructor_name: zenifra_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZenifraKimiK3 {
            model_name: "zenifra/kimi-k3",
            constructor_name: zenifra_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZenifraQwen36Flash {
            model_name: "zenifra/qwen3.6-flash",
            constructor_name: zenifra_qwen3_6_flash,
            display_name: "Qwen3.6 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZenifraQwen36Plus {
            model_name: "zenifra/qwen3.6-plus",
            constructor_name: zenifra_qwen3_6_plus,
            display_name: "Qwen3.6 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZenifraQwen37Max {
            model_name: "zenifra/qwen3.7-max",
            constructor_name: zenifra_qwen3_7_max,
            display_name: "Qwen3.7 Max",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZenifraQwen37Plus {
            model_name: "zenifra/qwen3.7-plus",
            constructor_name: zenifra_qwen3_7_plus,
            display_name: "Qwen3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZenifraQwen38Flash {
            model_name: "zenifra/qwen3.8-flash",
            constructor_name: zenifra_qwen3_8_flash,
            display_name: "Qwen3.8 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZenifraQwen38Max {
            model_name: "zenifra/qwen3.8-max",
            constructor_name: zenifra_qwen3_8_max,
            display_name: "Qwen3.8 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
