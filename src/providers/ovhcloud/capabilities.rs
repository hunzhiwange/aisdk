//! Capabilities for ovhcloud models.
//!
//! This module defines model types and their capabilities for ovhcloud providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::ovhcloud::Ovhcloud;

model_capabilities! {
    provider: Ovhcloud,
    models: {
        DeepseekV41Flash1017 {
            model_name: "deepseek-v4.1-flash-1017",
            constructor_name: deepseek_v4_1_flash_1017,
            display_name: "DeepSeek-V4.1-Flash-1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma426bA4b {
            model_name: "gemma-4-26b-a4b",
            constructor_name: gemma_4_26b_a4b,
            display_name: "gemma-4-26b-a4b",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "gpt-oss-120b",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss20b {
            model_name: "gpt-oss-20b",
            constructor_name: gpt_oss_20b,
            display_name: "gpt-oss-20b",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LagunaXs21 {
            model_name: "laguna-xs-2.1",
            constructor_name: laguna_xs_2_1,
            display_name: "laguna-xs-2.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3370bInstruct {
            model_name: "meta-llama-3_3-70b-instruct",
            constructor_name: meta_llama_3_3_70b_instruct,
            display_name: "Meta-Llama-3_3-70B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralSmall3224bInstruct2506 {
            model_name: "mistral-small-3.2-24b-instruct-2506",
            constructor_name: mistral_small_3_2_24b_instruct_2506,
            display_name: "Mistral-Small-3.2-24B-Instruct-2506",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Nemotron3Nano30b {
            model_name: "nemotron3-nano-30b",
            constructor_name: nemotron3_nano_30b,
            display_name: "nemotron3-nano-30b",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Nemotron3Omni30b {
            model_name: "nemotron3-omni-30b",
            constructor_name: nemotron3_omni_30b,
            display_name: "nemotron3-omni-30b",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OlalaV10Translate {
            model_name: "olala-v1.0-translate",
            constructor_name: olala_v1_0_translate,
            display_name: "Olala-v1.0-translate",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwen25Vl72bInstruct {
            model_name: "qwen2.5-vl-72b-instruct",
            constructor_name: qwen2_5_vl_72b_instruct,
            display_name: "Qwen2.5-VL-72B-Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3Vl30bA3b {
            model_name: "qwen3-vl-30b-a3b",
            constructor_name: qwen3_vl_30b_a3b,
            display_name: "qwen3-vl-30b-a3b",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen35397bA17b {
            model_name: "qwen3.5-397b-a17b",
            constructor_name: qwen3_5_397b_a17b,
            display_name: "Qwen3.5-397B-A17B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen359b {
            model_name: "qwen3.5-9b",
            constructor_name: qwen3_5_9b,
            display_name: "Qwen3.5-9B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3627b {
            model_name: "qwen3.6-27b",
            constructor_name: qwen3_6_27b,
            display_name: "Qwen3.6-27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3635bA3b {
            model_name: "qwen3.6-35b-a3b",
            constructor_name: qwen3_6_35b_a3b,
            display_name: "qwen3.6-35b-a3b",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3827b {
            model_name: "qwen3.8-27b",
            constructor_name: qwen3_8_27b,
            display_name: "Qwen3.8-27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3guardGen06b {
            model_name: "qwen3guard-gen-0.6b",
            constructor_name: qwen3guard_gen_0_6b,
            display_name: "Qwen3Guard-Gen-0.6B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwen3guardGen8b {
            model_name: "qwen3guard-gen-8b",
            constructor_name: qwen3guard_gen_8b,
            display_name: "Qwen3Guard-Gen-8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
    }
}
