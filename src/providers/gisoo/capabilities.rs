//! Capabilities for gisoo models.
//!
//! This module defines model types and their capabilities for gisoo providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::gisoo::Gisoo;

model_capabilities! {
    provider: Gisoo,
    models: {
        AlibabaGlm53 {
            model_name: "alibaba/glm-5.3",
            constructor_name: alibaba_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeFable51 {
            model_name: "claude-fable-5-1",
            constructor_name: claude_fable_5_1,
            display_name: "Claude Fable 5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku55 {
            model_name: "claude-haiku-5-5",
            constructor_name: claude_haiku_5_5,
            display_name: "Claude Haiku 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus55 {
            model_name: "claude-opus-5-5",
            constructor_name: claude_opus_5_5,
            display_name: "Claude Opus 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet55 {
            model_name: "claude-sonnet-5-5",
            constructor_name: claude_sonnet_5_5,
            display_name: "Claude Sonnet 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Pro {
            model_name: "deepseek-v4-pro",
            constructor_name: deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini31ProPreview {
            model_name: "gemini-3.1-pro-preview",
            constructor_name: gemini_3_1_pro_preview,
            display_name: "Gemini 3.1 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini35FlashLite {
            model_name: "gemini-3.5-flash-lite",
            constructor_name: gemini_3_5_flash_lite,
            display_name: "Gemini 3.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini38Flash {
            model_name: "gemini-3.8-flash",
            constructor_name: gemini_3_8_flash,
            display_name: "Gemini 3.8 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini4Argon {
            model_name: "gemini-4-argon",
            constructor_name: gemini_4_argon,
            display_name: "Gemini 4 Argon",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gpt41 {
            model_name: "gpt-4.1",
            constructor_name: gpt_4_1,
            display_name: "GPT-4.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Mini {
            model_name: "gpt-5-mini",
            constructor_name: gpt_5_mini,
            display_name: "GPT-5 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Luna {
            model_name: "gpt-5.6-luna",
            constructor_name: gpt_5_6_luna,
            display_name: "GPT-5.6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Sol {
            model_name: "gpt-5.6-sol",
            constructor_name: gpt_5_6_sol,
            display_name: "GPT-5.6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Terra {
            model_name: "gpt-5.6-terra",
            constructor_name: gpt_5_6_terra,
            display_name: "GPT-5.6 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Astra {
            model_name: "gpt-6-astra",
            constructor_name: gpt_6_astra,
            display_name: "GPT-6 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
