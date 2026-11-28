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
        ClinePassDeepseekV4Pro {
            model_name: "cline-pass/deepseek-v4-pro",
            constructor_name: cline_pass_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClinePassDeepseekV41Flash {
            model_name: "cline-pass/deepseek-v4.1-flash",
            constructor_name: cline_pass_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClinePassDeepseekV41Flash1017 {
            model_name: "cline-pass/deepseek-v4.1-flash-1017",
            constructor_name: cline_pass_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClinePassGlm53 {
            model_name: "cline-pass/glm-5.3",
            constructor_name: cline_pass_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClinePassGlm53Flash {
            model_name: "cline-pass/glm-5.3-flash",
            constructor_name: cline_pass_glm_5_3_flash,
            display_name: "cline-pass/glm-5.3-flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClinePassKimiK3 {
            model_name: "cline-pass/kimi-k3",
            constructor_name: cline_pass_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClinePassKimiK31Pro {
            model_name: "cline-pass/kimi-k3.1-pro",
            constructor_name: cline_pass_kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClinePassMimoV26Flash {
            model_name: "cline-pass/mimo-v2.6-flash",
            constructor_name: cline_pass_mimo_v2_6_flash,
            display_name: "MiMo-V2.6-Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClinePassMimoV26Pro {
            model_name: "cline-pass/mimo-v2.6-pro",
            constructor_name: cline_pass_mimo_v2_6_pro,
            display_name: "MiMo-V2.6-Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClinePassMinimaxM3 {
            model_name: "cline-pass/minimax-m3",
            constructor_name: cline_pass_minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClinePassMinimaxM31Flash {
            model_name: "cline-pass/minimax-m3.1-flash",
            constructor_name: cline_pass_minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClinePassMuseSpark13Contributor {
            model_name: "cline-pass/muse-spark-1.3-contributor",
            constructor_name: cline_pass_muse_spark_1_3_contributor,
            display_name: "Muse Spark 1.3 Contributor",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
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
        ClinePassQwen38Max {
            model_name: "cline-pass/qwen3.8-max",
            constructor_name: cline_pass_qwen3_8_max,
            display_name: "Qwen3.8 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
