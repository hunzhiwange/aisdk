//! Capabilities for abliteratedllm models.
//!
//! This module defines model types and their capabilities for abliteratedllm providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::abliteratedllm::Abliteratedllm;

model_capabilities! {
    provider: Abliteratedllm,
    models: {
        DeepseekV41Flash1017 {
            model_name: "deepseek-v4.1-flash-1017",
            constructor_name: deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash1017Ultrafast {
            model_name: "deepseek-v4.1-flash-1017-ultrafast",
            constructor_name: deepseek_v4_1_flash_1017_ultrafast,
            display_name: "DeepSeek V4.1 Flash 1017 Ultrafast",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41FlashOriginal {
            model_name: "deepseek-v4.1-flash-original",
            constructor_name: deepseek_v4_1_flash_original,
            display_name: "DeepSeek V4.1 Flash (original)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ember1 {
            model_name: "ember-1",
            constructor_name: ember_1,
            display_name: "Ember-1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53 {
            model_name: "glm-5.3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Fast {
            model_name: "glm-5.3-fast",
            constructor_name: glm_5_3_fast,
            display_name: "GLM-5.3 Fast",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Flash {
            model_name: "glm-5.3-flash",
            constructor_name: glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm53FlashAbliterated {
            model_name: "glm-5.3-flash-abliterated",
            constructor_name: glm_5_3_flash_abliterated,
            display_name: "GLM-5.3 Flash Abliterated",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK3 {
            model_name: "kimi-k3",
            constructor_name: kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK3Fast {
            model_name: "kimi-k3-fast",
            constructor_name: kimi_k3_fast,
            display_name: "Kimi K3 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK31Pro {
            model_name: "kimi-k3.1-pro",
            constructor_name: kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM3 {
            model_name: "minimax-m3",
            constructor_name: minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MuseSparkOpen {
            model_name: "muse-spark-open",
            constructor_name: muse_spark_open,
            display_name: "Muse Spark Open",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3824tA95b {
            model_name: "qwen3.8-2.4t-a95b",
            constructor_name: qwen3_8_2_4t_a95b,
            display_name: "Qwen3.8 2.4T A95B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen38Max {
            model_name: "qwen3.8-max",
            constructor_name: qwen3_8_max,
            display_name: "Qwen3.8 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
