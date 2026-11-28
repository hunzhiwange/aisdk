//! Capabilities for sparrowcodes models.
//!
//! This module defines model types and their capabilities for sparrowcodes providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::sparrowcodes::Sparrowcodes;

model_capabilities! {
    provider: Sparrowcodes,
    models: {
        AnthropicClaudeFable51 {
            model_name: "anthropic/claude-fable-5.1",
            constructor_name: anthropic_claude_fable_5_1,
            display_name: "Claude Fable 5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku55 {
            model_name: "anthropic/claude-haiku-5.5",
            constructor_name: anthropic_claude_haiku_5_5,
            display_name: "Claude Haiku 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus55 {
            model_name: "anthropic/claude-opus-5.5",
            constructor_name: anthropic_claude_opus_5_5,
            display_name: "Claude Opus 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet55 {
            model_name: "anthropic/claude-sonnet-5.5",
            constructor_name: anthropic_claude_sonnet_5_5,
            display_name: "Claude Sonnet 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash {
            model_name: "deepseek/deepseek-v4.1-flash",
            constructor_name: deepseek_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini31FlashLite {
            model_name: "google/gemini-3.1-flash-lite",
            constructor_name: google_gemini_3_1_flash_lite,
            display_name: "Gemini 3.1 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini35FlashLite {
            model_name: "google/gemini-3.5-flash-lite",
            constructor_name: google_gemini_3_5_flash_lite,
            display_name: "Gemini 3.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini38Flash {
            model_name: "google/gemini-3.8-flash",
            constructor_name: google_gemini_3_8_flash,
            display_name: "Gemini 3.8 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini4Argon {
            model_name: "google/gemini-4-argon",
            constructor_name: google_gemini_4_argon,
            display_name: "Gemini 4 Argon",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        OpenaiGpt6Astra {
            model_name: "openai/gpt-6-astra",
            constructor_name: openai_gpt_6_astra,
            display_name: "GPT-6 Astra",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt6Luna {
            model_name: "openai/gpt-6-luna",
            constructor_name: openai_gpt_6_luna,
            display_name: "GPT-6 Luna",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61Sol {
            model_name: "openai/gpt-6.1-sol",
            constructor_name: openai_gpt_6_1_sol,
            display_name: "GPT-6.1 Sol",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt61Terra {
            model_name: "openai/gpt-6.1-terra",
            constructor_name: openai_gpt_6_1_terra,
            display_name: "GPT-6.1 Terra",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen37Flash {
            model_name: "qwen/qwen3.7-flash",
            constructor_name: qwen_qwen3_7_flash,
            display_name: "Qwen3.7 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen37Plus {
            model_name: "qwen/qwen3.7-plus",
            constructor_name: qwen_qwen3_7_plus,
            display_name: "Qwen3.7 Plus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SparrowAuto {
            model_name: "sparrow/auto",
            constructor_name: sparrow_auto,
            display_name: "Sparrow Auto",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV26Flash {
            model_name: "xiaomi/mimo-v2.6-flash",
            constructor_name: xiaomi_mimo_v2_6_flash,
            display_name: "MiMo-V2.6-Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm47Flash {
            model_name: "z-ai/glm-4.7-flash",
            constructor_name: z_ai_glm_4_7_flash,
            display_name: "GLM-4.7-Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53Flash {
            model_name: "z-ai/glm-5.3-flash",
            constructor_name: z_ai_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
