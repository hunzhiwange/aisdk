//! Capabilities for hyper models.
//!
//! This module defines model types and their capabilities for hyper providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::hyper::Hyper;

model_capabilities! {
    provider: Hyper,
    models: {
        DeepseekV4Pro0813 {
            model_name: "deepseek-v4-pro-0813",
            constructor_name: deepseek_v4_pro_0813,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash1017 {
            model_name: "deepseek-v4.1-flash-1017",
            constructor_name: deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash1017Economy {
            model_name: "deepseek-v4.1-flash-1017-economy",
            constructor_name: deepseek_v4_1_flash_1017_economy,
            display_name: "DeepSeek V4.1 Flash 1017 (Economy)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma426bA4bIt {
            model_name: "gemma-4-26b-a4b-it",
            constructor_name: gemma_4_26b_a4b_it,
            display_name: "Gemma 4 26B A4B IT",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53 {
            model_name: "glm-5.3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Flash {
            model_name: "glm-5.3-flash",
            constructor_name: glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK3 {
            model_name: "kimi-k3",
            constructor_name: kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK31Pro {
            model_name: "kimi-k3.1-pro",
            constructor_name: kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM3 {
            model_name: "minimax-m3",
            constructor_name: minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MuseSparkOpen {
            model_name: "muse-spark-open",
            constructor_name: muse_spark_open,
            display_name: "Muse Spark Open",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Prism {
            model_name: "prism",
            constructor_name: prism,
            display_name: "Prism",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PrismCore {
            model_name: "prism-core",
            constructor_name: prism_core,
            display_name: "Prism Core",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PrismFlash {
            model_name: "prism-flash",
            constructor_name: prism_flash,
            display_name: "Prism Flash",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3824tA95b {
            model_name: "qwen3.8-2.4t-a95b",
            constructor_name: qwen3_8_2_4t_a95b,
            display_name: "Qwen3.8 2.4T A95B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3827b {
            model_name: "qwen3.8-27b",
            constructor_name: qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen38Flash {
            model_name: "qwen3.8-flash",
            constructor_name: qwen3_8_flash,
            display_name: "Qwen3.8 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen38Max {
            model_name: "qwen3.8-max",
            constructor_name: qwen3_8_max,
            display_name: "Qwen3.8 Max Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen4Max {
            model_name: "qwen4-max",
            constructor_name: qwen4_max,
            display_name: "Qwen4 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
