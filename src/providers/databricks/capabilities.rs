//! Capabilities for databricks models.
//!
//! This module defines model types and their capabilities for databricks providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::databricks::Databricks;

model_capabilities! {
    provider: Databricks,
    models: {
        DatabricksClaudeFable5 {
            model_name: "databricks-claude-fable-5",
            constructor_name: databricks_claude_fable_5,
            display_name: "Claude Fable 5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeFable51 {
            model_name: "databricks-claude-fable-5-1",
            constructor_name: databricks_claude_fable_5_1,
            display_name: "Claude Fable 5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeHaiku45 {
            model_name: "databricks-claude-haiku-4-5",
            constructor_name: databricks_claude_haiku_4_5,
            display_name: "Claude Haiku 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeHaiku55 {
            model_name: "databricks-claude-haiku-5-5",
            constructor_name: databricks_claude_haiku_5_5,
            display_name: "Claude Haiku 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeOpus41 {
            model_name: "databricks-claude-opus-4-1",
            constructor_name: databricks_claude_opus_4_1,
            display_name: "Claude Opus 4.1 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeOpus45 {
            model_name: "databricks-claude-opus-4-5",
            constructor_name: databricks_claude_opus_4_5,
            display_name: "Claude Opus 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeOpus46 {
            model_name: "databricks-claude-opus-4-6",
            constructor_name: databricks_claude_opus_4_6,
            display_name: "Claude Opus 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeOpus47 {
            model_name: "databricks-claude-opus-4-7",
            constructor_name: databricks_claude_opus_4_7,
            display_name: "Claude Opus 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeOpus48 {
            model_name: "databricks-claude-opus-4-8",
            constructor_name: databricks_claude_opus_4_8,
            display_name: "Claude Opus 4.8",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeOpus5 {
            model_name: "databricks-claude-opus-5",
            constructor_name: databricks_claude_opus_5,
            display_name: "Claude Opus 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeOpus55 {
            model_name: "databricks-claude-opus-5-5",
            constructor_name: databricks_claude_opus_5_5,
            display_name: "Claude Opus 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeSonnet45 {
            model_name: "databricks-claude-sonnet-4-5",
            constructor_name: databricks_claude_sonnet_4_5,
            display_name: "Claude Sonnet 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeSonnet46 {
            model_name: "databricks-claude-sonnet-4-6",
            constructor_name: databricks_claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeSonnet5 {
            model_name: "databricks-claude-sonnet-5",
            constructor_name: databricks_claude_sonnet_5,
            display_name: "Claude Sonnet 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksClaudeSonnet55 {
            model_name: "databricks-claude-sonnet-5-5",
            constructor_name: databricks_claude_sonnet_5_5,
            display_name: "Claude Sonnet 5.5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksDeepseekV41Flash1017 {
            model_name: "databricks-deepseek-v4-1-flash-1017",
            constructor_name: databricks_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGemini31FlashLite {
            model_name: "databricks-gemini-3-1-flash-lite",
            constructor_name: databricks_gemini_3_1_flash_lite,
            display_name: "Gemini 3.1 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DatabricksGemini31Pro {
            model_name: "databricks-gemini-3-1-pro",
            constructor_name: databricks_gemini_3_1_pro,
            display_name: "Gemini 3.1 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DatabricksGemini35Flash {
            model_name: "databricks-gemini-3-5-flash",
            constructor_name: databricks_gemini_3_5_flash,
            display_name: "Gemini 3.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DatabricksGemini35FlashLite {
            model_name: "databricks-gemini-3-5-flash-lite",
            constructor_name: databricks_gemini_3_5_flash_lite,
            display_name: "Gemini 3.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DatabricksGemini37Flash {
            model_name: "databricks-gemini-3-7-flash",
            constructor_name: databricks_gemini_3_7_flash,
            display_name: "Gemini 3.7 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DatabricksGemini38Flash {
            model_name: "databricks-gemini-3-8-flash",
            constructor_name: databricks_gemini_3_8_flash,
            display_name: "Gemini 3.8 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DatabricksGemini3Flash {
            model_name: "databricks-gemini-3-flash",
            constructor_name: databricks_gemini_3_flash,
            display_name: "Gemini 3 Flash Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DatabricksGemini4Argon {
            model_name: "databricks-gemini-4-argon",
            constructor_name: databricks_gemini_4_argon,
            display_name: "Gemini 4 Argon",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DatabricksGemma312b {
            model_name: "databricks-gemma-3-12b",
            constructor_name: databricks_gemma_3_12b,
            display_name: "Gemma 3 12B IT",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGlm53 {
            model_name: "databricks-glm-5-3",
            constructor_name: databricks_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGlm53Flash {
            model_name: "databricks-glm-5-3-flash",
            constructor_name: databricks_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt51 {
            model_name: "databricks-gpt-5-1",
            constructor_name: databricks_gpt_5_1,
            display_name: "GPT-5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt52 {
            model_name: "databricks-gpt-5-2",
            constructor_name: databricks_gpt_5_2,
            display_name: "GPT-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt53Codex {
            model_name: "databricks-gpt-5-3-codex",
            constructor_name: databricks_gpt_5_3_codex,
            display_name: "GPT-5.3 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt54 {
            model_name: "databricks-gpt-5-4",
            constructor_name: databricks_gpt_5_4,
            display_name: "GPT-5.4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt54Mini {
            model_name: "databricks-gpt-5-4-mini",
            constructor_name: databricks_gpt_5_4_mini,
            display_name: "GPT-5.4 mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt54Nano {
            model_name: "databricks-gpt-5-4-nano",
            constructor_name: databricks_gpt_5_4_nano,
            display_name: "GPT-5.4 nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt55 {
            model_name: "databricks-gpt-5-5",
            constructor_name: databricks_gpt_5_5,
            display_name: "GPT-5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt55Pro {
            model_name: "databricks-gpt-5-5-pro",
            constructor_name: databricks_gpt_5_5_pro,
            display_name: "GPT-5.5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt56Luna {
            model_name: "databricks-gpt-5-6-luna",
            constructor_name: databricks_gpt_5_6_luna,
            display_name: "GPT-5.6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt56Sol {
            model_name: "databricks-gpt-5-6-sol",
            constructor_name: databricks_gpt_5_6_sol,
            display_name: "GPT-5.6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt56Terra {
            model_name: "databricks-gpt-5-6-terra",
            constructor_name: databricks_gpt_5_6_terra,
            display_name: "GPT-5.6 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt61Sol {
            model_name: "databricks-gpt-6-1-sol",
            constructor_name: databricks_gpt_6_1_sol,
            display_name: "GPT-6.1 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt61Terra {
            model_name: "databricks-gpt-6-1-terra",
            constructor_name: databricks_gpt_6_1_terra,
            display_name: "GPT-6.1 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt6Astra {
            model_name: "databricks-gpt-6-astra",
            constructor_name: databricks_gpt_6_astra,
            display_name: "GPT-6 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt6Luna {
            model_name: "databricks-gpt-6-luna",
            constructor_name: databricks_gpt_6_luna,
            display_name: "GPT-6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGpt6Sol {
            model_name: "databricks-gpt-6-sol",
            constructor_name: databricks_gpt_6_sol,
            display_name: "GPT-6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGptOss120b {
            model_name: "databricks-gpt-oss-120b",
            constructor_name: databricks_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGptOss20b {
            model_name: "databricks-gpt-oss-20b",
            constructor_name: databricks_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGrok46 {
            model_name: "databricks-grok-4-6",
            constructor_name: databricks_grok_4_6,
            display_name: "Grok 4.6",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksGrok47 {
            model_name: "databricks-grok-4-7",
            constructor_name: databricks_grok_4_7,
            display_name: "Grok 4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksKimiK3 {
            model_name: "databricks-kimi-k3",
            constructor_name: databricks_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksKimiK31Pro {
            model_name: "databricks-kimi-k3-1-pro",
            constructor_name: databricks_kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksLlama4Maverick {
            model_name: "databricks-llama-4-maverick",
            constructor_name: databricks_llama_4_maverick,
            display_name: "Llama 4 Maverick 17B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksMetaLlama318bInstruct {
            model_name: "databricks-meta-llama-3-1-8b-instruct",
            constructor_name: databricks_meta_llama_3_1_8b_instruct,
            display_name: "Llama-3.1-8B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DatabricksMetaLlama3370bInstruct {
            model_name: "databricks-meta-llama-3-3-70b-instruct",
            constructor_name: databricks_meta_llama_3_3_70b_instruct,
            display_name: "Llama-3.3-70B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SystemAiClaudeFable52 {
            model_name: "system.ai.claude-fable-5-2",
            constructor_name: system_ai_claude_fable_5_2,
            display_name: "Claude Fable 5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SystemAiGemini4Carbon {
            model_name: "system.ai.gemini-4-carbon",
            constructor_name: system_ai_gemini_4_carbon,
            display_name: "Gemini 4 Carbon",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        SystemAiGemini4Hydrogen {
            model_name: "system.ai.gemini-4-hydrogen",
            constructor_name: system_ai_gemini_4_hydrogen,
            display_name: "Gemini 4 Hydrogen",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SystemAiGpt62Astra {
            model_name: "system.ai.gpt-6-2-astra",
            constructor_name: system_ai_gpt_6_2_astra,
            display_name: "GPT-6.2 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SystemAiGpt62Sol {
            model_name: "system.ai.gpt-6-2-sol",
            constructor_name: system_ai_gpt_6_2_sol,
            display_name: "GPT-6.2 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
