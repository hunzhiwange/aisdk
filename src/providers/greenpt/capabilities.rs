//! Capabilities for greenpt models.
//!
//! This module defines model types and their capabilities for greenpt providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::greenpt::Greenpt;

model_capabilities! {
    provider: Greenpt,
    models: {
        Devstral2123bInstruct2512 {
            model_name: "devstral-2-123b-instruct-2512",
            constructor_name: devstral_2_123b_instruct_2512,
            display_name: "Devstral 2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma327bIt {
            model_name: "gemma-3-27b-it",
            constructor_name: gemma_3_27b_it,
            display_name: "Gemma 3 27B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma4 {
            model_name: "gemma4",
            constructor_name: gemma4,
            display_name: "gemma4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm51 {
            model_name: "glm-5.1",
            constructor_name: glm_5_1,
            display_name: "GLM-5.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52 {
            model_name: "glm-5.2",
            constructor_name: glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GreenL {
            model_name: "green-l",
            constructor_name: green_l,
            display_name: "Green L",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GreenLRaw {
            model_name: "green-l-raw",
            constructor_name: green_l_raw,
            display_name: "Green L Raw",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GreenR {
            model_name: "green-r",
            constructor_name: green_r,
            display_name: "Green R",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GreenRRaw {
            model_name: "green-r-raw",
            constructor_name: green_r_raw,
            display_name: "Green R Raw",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GreenS {
            model_name: "green-s",
            constructor_name: green_s,
            display_name: "Green S",
            capabilities: [AudioInputSupport, TextOutputSupport]
        },
        GreenSPro {
            model_name: "green-s-pro",
            constructor_name: green_s_pro,
            display_name: "Green S Pro",
            capabilities: [AudioInputSupport, TextOutputSupport]
        },
        Holo230bA3b {
            model_name: "holo2-30b-a3b",
            constructor_name: holo2_30b_a3b,
            display_name: "Holo2 30B A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK26 {
            model_name: "kimi-k2.6",
            constructor_name: kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK26Fast {
            model_name: "kimi-k2.6-fast",
            constructor_name: kimi_k2_6_fast,
            display_name: "Kimi K2.6 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK27Code {
            model_name: "kimi-k2.7-code",
            constructor_name: kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama3370bInstruct {
            model_name: "llama-3.3-70b-instruct",
            constructor_name: llama_3_3_70b_instruct,
            display_name: "Llama-3.3-70B-Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM25 {
            model_name: "minimax-m2.5",
            constructor_name: minimax_m2_5,
            display_name: "MiniMax-M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMedium35128b {
            model_name: "mistral-medium-3.5-128b",
            constructor_name: mistral_medium_3_5_128b,
            display_name: "Mistral Medium 3.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralSmall3224bInstruct2506 {
            model_name: "mistral-small-3.2-24b-instruct-2506",
            constructor_name: mistral_small_3_2_24b_instruct_2506,
            display_name: "Mistral Small 3.2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Pixtral12b2409 {
            model_name: "pixtral-12b-2409",
            constructor_name: pixtral_12b_2409,
            display_name: "Pixtral 12B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3235bA22bInstruct2507 {
            model_name: "qwen3-235b-a22b-instruct-2507",
            constructor_name: qwen3_235b_a22b_instruct_2507,
            display_name: "Qwen3 235B A22B Instruct 2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Coder30bA3bInstruct {
            model_name: "qwen3-coder-30b-a3b-instruct",
            constructor_name: qwen3_coder_30b_a3b_instruct,
            display_name: "Qwen3-Coder 30B-A3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen35397bA17b {
            model_name: "qwen3.5-397b-a17b",
            constructor_name: qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B-A17B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3635bA3b {
            model_name: "qwen3.6-35b-a3b",
            constructor_name: qwen3_6_35b_a3b,
            display_name: "Qwen3.6 35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        VoxtralSmall24b2507 {
            model_name: "voxtral-small-24b-2507",
            constructor_name: voxtral_small_24b_2507,
            display_name: "Voxtral Small 24B",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
