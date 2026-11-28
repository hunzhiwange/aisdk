//! Capabilities for dothoosh models.
//!
//! This module defines model types and their capabilities for dothoosh providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::dothoosh::Dothoosh;

model_capabilities! {
    provider: Dothoosh,
    models: {
        ClaudeFable51 {
            model_name: "claude-fable-5-1",
            constructor_name: claude_fable_5_1,
            display_name: "Claude Fable 5.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku55 {
            model_name: "claude-haiku-5-5",
            constructor_name: claude_haiku_5_5,
            display_name: "Claude Haiku 5.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus55 {
            model_name: "claude-opus-5-5",
            constructor_name: claude_opus_5_5,
            display_name: "Claude Opus 5.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet55 {
            model_name: "claude-sonnet-5-5",
            constructor_name: claude_sonnet_5_5,
            display_name: "Claude Sonnet 5.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekFlash {
            model_name: "deepseek-flash",
            constructor_name: deepseek_flash,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Pro {
            model_name: "deepseek-v4-pro",
            constructor_name: deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini35FlashLite {
            model_name: "gemini-3.5-flash-lite",
            constructor_name: gemini_3_5_flash_lite,
            display_name: "Gemini 3.5 Flash Lite",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini38Flash {
            model_name: "gemini-3.8-flash",
            constructor_name: gemini_3_8_flash,
            display_name: "Gemini 3.8 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini4Argon {
            model_name: "gemini-4-argon",
            constructor_name: gemini_4_argon,
            display_name: "Gemini 4 Argon",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Astra {
            model_name: "gpt-6-astra",
            constructor_name: gpt_6_astra,
            display_name: "GPT-6 Astra",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Luna {
            model_name: "gpt-6-luna",
            constructor_name: gpt_6_luna,
            display_name: "GPT-6 Luna",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt61Sol {
            model_name: "gpt-6.1-sol",
            constructor_name: gpt_6_1_sol,
            display_name: "GPT-6.1 Sol",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK26 {
            model_name: "kimi-k2.6",
            constructor_name: kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK27Code {
            model_name: "kimi-k2.7-code",
            constructor_name: kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK3 {
            model_name: "kimi-k3",
            constructor_name: kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK31Pro {
            model_name: "kimi-k3.1-pro",
            constructor_name: kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
