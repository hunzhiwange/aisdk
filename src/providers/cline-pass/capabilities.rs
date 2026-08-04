//! Capabilities for cline_pass models.
//!
//! This module defines model types and their capabilities for cline_pass providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::cline_pass::ClinePass;

model_capabilities! {
    provider: ClinePass,
    models: {
        ClinePassDeepseekV4Flash {
            model_name: "cline-pass/deepseek-v4-flash",
            constructor_name: cline_pass_deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClinePassDeepseekV4Pro {
            model_name: "cline-pass/deepseek-v4-pro",
            constructor_name: cline_pass_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClinePassGlm52 {
            model_name: "cline-pass/glm-5.2",
            constructor_name: cline_pass_glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClinePassKimiK26 {
            model_name: "cline-pass/kimi-k2.6",
            constructor_name: cline_pass_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClinePassKimiK27Code {
            model_name: "cline-pass/kimi-k2.7-code",
            constructor_name: cline_pass_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClinePassKimiK3 {
            model_name: "cline-pass/kimi-k3",
            constructor_name: cline_pass_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClinePassMimoV25 {
            model_name: "cline-pass/mimo-v2.5",
            constructor_name: cline_pass_mimo_v2_5,
            display_name: "MiMo-V2.5",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClinePassMimoV25Pro {
            model_name: "cline-pass/mimo-v2.5-pro",
            constructor_name: cline_pass_mimo_v2_5_pro,
            display_name: "MiMo-V2.5-Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClinePassMinimaxM3 {
            model_name: "cline-pass/minimax-m3",
            constructor_name: cline_pass_minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClinePassQwen37Max {
            model_name: "cline-pass/qwen3.7-max",
            constructor_name: cline_pass_qwen3_7_max,
            display_name: "Qwen3.7 Max",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClinePassQwen37Plus {
            model_name: "cline-pass/qwen3.7-plus",
            constructor_name: cline_pass_qwen3_7_plus,
            display_name: "Qwen3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
