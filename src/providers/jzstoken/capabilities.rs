//! Capabilities for jzstoken models.
//!
//! This module defines model types and their capabilities for jzstoken providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::jzstoken::Jzstoken;

model_capabilities! {
    provider: Jzstoken,
    models: {
        ClaudeHaiku4520251001 {
            model_name: "claude-haiku-4-5-20251001",
            constructor_name: claude_haiku_4_5_20251001,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus47 {
            model_name: "claude-opus-4-7",
            constructor_name: claude_opus_4_7,
            display_name: "Claude Opus 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus48 {
            model_name: "claude-opus-4-8",
            constructor_name: claude_opus_4_8,
            display_name: "Claude Opus 4.8",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus5 {
            model_name: "claude-opus-5",
            constructor_name: claude_opus_5,
            display_name: "Claude Opus 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus55 {
            model_name: "claude-opus-5.5",
            constructor_name: claude_opus_5_5,
            display_name: "Claude Opus 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet5 {
            model_name: "claude-sonnet-5",
            constructor_name: claude_sonnet_5,
            display_name: "Claude Sonnet 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet55 {
            model_name: "claude-sonnet-5.5",
            constructor_name: claude_sonnet_5_5,
            display_name: "Claude Sonnet 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Flash {
            model_name: "deepseek-v4-flash",
            constructor_name: deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Pro {
            model_name: "deepseek-v4-pro",
            constructor_name: deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash {
            model_name: "deepseek-v4.1-flash",
            constructor_name: deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53 {
            model_name: "glm-5.3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt55 {
            model_name: "gpt-5.5",
            constructor_name: gpt_5_5,
            display_name: "GPT-5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Sol {
            model_name: "gpt-5.6-sol",
            constructor_name: gpt_5_6_sol,
            display_name: "GPT-5.6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Terra {
            model_name: "gpt-5.6-terra",
            constructor_name: gpt_5_6_terra,
            display_name: "GPT-5.6 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Astra {
            model_name: "gpt-6-astra",
            constructor_name: gpt_6_astra,
            display_name: "GPT-6 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt61Sol {
            model_name: "gpt-6.1-sol",
            constructor_name: gpt_6_1_sol,
            display_name: "GPT-6.1 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt61Terra {
            model_name: "gpt-6.1-terra",
            constructor_name: gpt_6_1_terra,
            display_name: "GPT-6.1 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok47 {
            model_name: "grok-4.7",
            constructor_name: grok_4_7,
            display_name: "Grok 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK26 {
            model_name: "kimi-k2.6",
            constructor_name: kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK3 {
            model_name: "kimi-k3",
            constructor_name: kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK31Pro {
            model_name: "kimi-k3.1-pro",
            constructor_name: kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen38Max {
            model_name: "qwen3.8-max",
            constructor_name: qwen3_8_max,
            display_name: "Qwen3.8 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
