//! Capabilities for aki_io models.
//!
//! This module defines model types and their capabilities for aki_io providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::aki_io::AkiIo;

model_capabilities! {
    provider: AkiIo,
    models: {
        Gemma426b {
            model_name: "gemma4-26b",
            constructor_name: gemma4_26b,
            display_name: "Gemma 4 26B A4B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK27Code1100b {
            model_name: "kimi-k2.7-code-1100b",
            constructor_name: kimi_k2_7_code_1100b,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM25230b {
            model_name: "minimax-m2.5-230b",
            constructor_name: minimax_m2_5_230b,
            display_name: "MiniMax-M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Mistral4119b {
            model_name: "mistral4-119b",
            constructor_name: mistral4_119b,
            display_name: "Mistral Small 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3635b {
            model_name: "qwen3.6-35b",
            constructor_name: qwen3_6_35b,
            display_name: "Qwen3.6 35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
