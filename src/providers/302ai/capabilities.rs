//! Capabilities for ai_302 models.
//!
//! This module defines model types and their capabilities for ai_302 providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::ai_302::Ai302;

model_capabilities! {
    provider: Ai302,
    models: {
        MinimaxM1 {
            model_name: "MiniMax-M1",
            constructor_name: minimax_m1,
            display_name: "MiniMax-M1",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM2 {
            model_name: "MiniMax-M2",
            constructor_name: minimax_m2,
            display_name: "MiniMax-M2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM21 {
            model_name: "MiniMax-M2.1",
            constructor_name: minimax_m2_1,
            display_name: "MiniMax-M2.1",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM27 {
            model_name: "MiniMax-M2.7",
            constructor_name: minimax_m2_7,
            display_name: "MiniMax-M2.7",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM27Highspeed {
            model_name: "MiniMax-M2.7-highspeed",
            constructor_name: minimax_m2_7_highspeed,
            display_name: "MiniMax-M2.7-highspeed",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Chatgpt4oLatest {
            model_name: "chatgpt-4o-latest",
            constructor_name: chatgpt_4o_latest,
            display_name: "chatgpt-4o-latest",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Claude35Haiku20241022 {
            model_name: "claude-3-5-haiku-20241022",
            constructor_name: claude_3_5_haiku_20241022,
            display_name: "claude-3-5-haiku-20241022",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude35HaikuLatest {
            model_name: "claude-3-5-haiku-latest",
            constructor_name: claude_3_5_haiku_latest,
            display_name: "claude-3-5-haiku-latest",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku45 {
            model_name: "claude-haiku-4-5",
            constructor_name: claude_haiku_4_5,
            display_name: "claude-haiku-4-5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku4520251001 {
            model_name: "claude-haiku-4-5-20251001",
            constructor_name: claude_haiku_4_5_20251001,
            display_name: "claude-haiku-4-5-20251001",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4120250805 {
            model_name: "claude-opus-4-1-20250805",
            constructor_name: claude_opus_4_1_20250805,
            display_name: "claude-opus-4-1-20250805",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4120250805Thinking {
            model_name: "claude-opus-4-1-20250805-thinking",
            constructor_name: claude_opus_4_1_20250805_thinking,
            display_name: "claude-opus-4-1-20250805-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus420250514 {
            model_name: "claude-opus-4-20250514",
            constructor_name: claude_opus_4_20250514,
            display_name: "claude-opus-4-20250514",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus45 {
            model_name: "claude-opus-4-5",
            constructor_name: claude_opus_4_5,
            display_name: "claude-opus-4-5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4520251101 {
            model_name: "claude-opus-4-5-20251101",
            constructor_name: claude_opus_4_5_20251101,
            display_name: "claude-opus-4-5-20251101",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4520251101Thinking {
            model_name: "claude-opus-4-5-20251101-thinking",
            constructor_name: claude_opus_4_5_20251101_thinking,
            display_name: "claude-opus-4-5-20251101-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus46 {
            model_name: "claude-opus-4-6",
            constructor_name: claude_opus_4_6,
            display_name: "claude-opus-4-6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus46Thinking {
            model_name: "claude-opus-4-6-thinking",
            constructor_name: claude_opus_4_6_thinking,
            display_name: "claude-opus-4-6-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus47 {
            model_name: "claude-opus-4-7",
            constructor_name: claude_opus_4_7,
            display_name: "claude-opus-4-7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet420250514 {
            model_name: "claude-sonnet-4-20250514",
            constructor_name: claude_sonnet_4_20250514,
            display_name: "claude-sonnet-4-20250514",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet45 {
            model_name: "claude-sonnet-4-5",
            constructor_name: claude_sonnet_4_5,
            display_name: "claude-sonnet-4-5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4520250929 {
            model_name: "claude-sonnet-4-5-20250929",
            constructor_name: claude_sonnet_4_5_20250929,
            display_name: "claude-sonnet-4-5-20250929",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4520250929Thinking {
            model_name: "claude-sonnet-4-5-20250929-thinking",
            constructor_name: claude_sonnet_4_5_20250929_thinking,
            display_name: "claude-sonnet-4-5-20250929-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet46 {
            model_name: "claude-sonnet-4-6",
            constructor_name: claude_sonnet_4_6,
            display_name: "claude-sonnet-4-6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet46Thinking {
            model_name: "claude-sonnet-4-6-thinking",
            constructor_name: claude_sonnet_4_6_thinking,
            display_name: "claude-sonnet-4-6-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekChat {
            model_name: "deepseek-chat",
            constructor_name: deepseek_chat,
            display_name: "Deepseek-Chat",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekReasoner {
            model_name: "deepseek-reasoner",
            constructor_name: deepseek_reasoner,
            display_name: "Deepseek-Reasoner",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV32 {
            model_name: "deepseek-v3.2",
            constructor_name: deepseek_v3_2,
            display_name: "deepseek-v3.2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV32Thinking {
            model_name: "deepseek-v3.2-thinking",
            constructor_name: deepseek_v3_2_thinking,
            display_name: "DeepSeek-V3.2-Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DoubaoSeed16Thinking250715 {
            model_name: "doubao-seed-1-6-thinking-250715",
            constructor_name: doubao_seed_1_6_thinking_250715,
            display_name: "doubao-seed-1-6-thinking-250715",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DoubaoSeed16Vision250815 {
            model_name: "doubao-seed-1-6-vision-250815",
            constructor_name: doubao_seed_1_6_vision_250815,
            display_name: "doubao-seed-1-6-vision-250815",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DoubaoSeed18251215 {
            model_name: "doubao-seed-1-8-251215",
            constructor_name: doubao_seed_1_8_251215,
            display_name: "doubao-seed-1-8-251215",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DoubaoSeedCodePreview251028 {
            model_name: "doubao-seed-code-preview-251028",
            constructor_name: doubao_seed_code_preview_251028,
            display_name: "doubao-seed-code-preview-251028",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini20FlashLite {
            model_name: "gemini-2.0-flash-lite",
            constructor_name: gemini_2_0_flash_lite,
            display_name: "gemini-2.0-flash-lite",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25Flash {
            model_name: "gemini-2.5-flash",
            constructor_name: gemini_2_5_flash,
            display_name: "gemini-2.5-flash",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashImage {
            model_name: "gemini-2.5-flash-image",
            constructor_name: gemini_2_5_flash_image,
            display_name: "gemini-2.5-flash-image",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashLitePreview092025 {
            model_name: "gemini-2.5-flash-lite-preview-09-2025",
            constructor_name: gemini_2_5_flash_lite_preview_09_2025,
            display_name: "gemini-2.5-flash-lite-preview-09-2025",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashNothink {
            model_name: "gemini-2.5-flash-nothink",
            constructor_name: gemini_2_5_flash_nothink,
            display_name: "gemini-2.5-flash-nothink",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashPreview092025 {
            model_name: "gemini-2.5-flash-preview-09-2025",
            constructor_name: gemini_2_5_flash_preview_09_2025,
            display_name: "gemini-2.5-flash-preview-09-2025",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25Pro {
            model_name: "gemini-2.5-pro",
            constructor_name: gemini_2_5_pro,
            display_name: "gemini-2.5-pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini3FlashPreview {
            model_name: "gemini-3-flash-preview",
            constructor_name: gemini_3_flash_preview,
            display_name: "gemini-3-flash-preview",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini3ProImagePreview {
            model_name: "gemini-3-pro-image-preview",
            constructor_name: gemini_3_pro_image_preview,
            display_name: "gemini-3-pro-image-preview",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini3ProPreview {
            model_name: "gemini-3-pro-preview",
            constructor_name: gemini_3_pro_preview,
            display_name: "gemini-3-pro-preview",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini31FlashImagePreview {
            model_name: "gemini-3.1-flash-image-preview",
            constructor_name: gemini_3_1_flash_image_preview,
            display_name: "gemini-3.1-flash-image-preview",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Glm45 {
            model_name: "glm-4.5",
            constructor_name: glm_4_5,
            display_name: "GLM-4.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm45Air {
            model_name: "glm-4.5-air",
            constructor_name: glm_4_5_air,
            display_name: "glm-4.5-air",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm45Airx {
            model_name: "glm-4.5-airx",
            constructor_name: glm_4_5_airx,
            display_name: "glm-4.5-airx",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm45X {
            model_name: "glm-4.5-x",
            constructor_name: glm_4_5_x,
            display_name: "glm-4.5-x",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm45v {
            model_name: "glm-4.5v",
            constructor_name: glm_4_5v,
            display_name: "GLM-4.5V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm46 {
            model_name: "glm-4.6",
            constructor_name: glm_4_6,
            display_name: "glm-4.6",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm46v {
            model_name: "glm-4.6v",
            constructor_name: glm_4_6v,
            display_name: "GLM-4.6V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm47 {
            model_name: "glm-4.7",
            constructor_name: glm_4_7,
            display_name: "glm-4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm47Flashx {
            model_name: "glm-4.7-flashx",
            constructor_name: glm_4_7_flashx,
            display_name: "glm-4.7-flashx",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm5 {
            model_name: "glm-5",
            constructor_name: glm_5,
            display_name: "glm-5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm5Turbo {
            model_name: "glm-5-turbo",
            constructor_name: glm_5_turbo,
            display_name: "glm-5-turbo",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm51 {
            model_name: "glm-5.1",
            constructor_name: glm_5_1,
            display_name: "glm-5.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm5vTurbo {
            model_name: "glm-5v-turbo",
            constructor_name: glm_5v_turbo,
            display_name: "GLM-5V-Turbo",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GlmForCoding {
            model_name: "glm-for-coding",
            constructor_name: glm_for_coding,
            display_name: "glm-for-coding",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41 {
            model_name: "gpt-4.1",
            constructor_name: gpt_4_1,
            display_name: "gpt-4.1",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Mini {
            model_name: "gpt-4.1-mini",
            constructor_name: gpt_4_1_mini,
            display_name: "gpt-4.1-mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Nano {
            model_name: "gpt-4.1-nano",
            constructor_name: gpt_4_1_nano,
            display_name: "gpt-4.1-nano",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4o {
            model_name: "gpt-4o",
            constructor_name: gpt_4o,
            display_name: "gpt-4o",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5 {
            model_name: "gpt-5",
            constructor_name: gpt_5,
            display_name: "gpt-5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Mini {
            model_name: "gpt-5-mini",
            constructor_name: gpt_5_mini,
            display_name: "gpt-5-mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Pro {
            model_name: "gpt-5-pro",
            constructor_name: gpt_5_pro,
            display_name: "gpt-5-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Thinking {
            model_name: "gpt-5-thinking",
            constructor_name: gpt_5_thinking,
            display_name: "gpt-5-thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51 {
            model_name: "gpt-5.1",
            constructor_name: gpt_5_1,
            display_name: "gpt-5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51ChatLatest {
            model_name: "gpt-5.1-chat-latest",
            constructor_name: gpt_5_1_chat_latest,
            display_name: "gpt-5.1-chat-latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52 {
            model_name: "gpt-5.2",
            constructor_name: gpt_5_2,
            display_name: "gpt-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52ChatLatest {
            model_name: "gpt-5.2-chat-latest",
            constructor_name: gpt_5_2_chat_latest,
            display_name: "gpt-5.2-chat-latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54 {
            model_name: "gpt-5.4",
            constructor_name: gpt_5_4,
            display_name: "gpt-5.4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54Mini {
            model_name: "gpt-5.4-mini",
            constructor_name: gpt_5_4_mini,
            display_name: "gpt-5.4-mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54Mini20260317 {
            model_name: "gpt-5.4-mini-2026-03-17",
            constructor_name: gpt_5_4_mini_2026_03_17,
            display_name: "gpt-5.4-mini-2026-03-17",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54Nano {
            model_name: "gpt-5.4-nano",
            constructor_name: gpt_5_4_nano,
            display_name: "gpt-5.4-nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54Nano20260317 {
            model_name: "gpt-5.4-nano-2026-03-17",
            constructor_name: gpt_5_4_nano_2026_03_17,
            display_name: "gpt-5.4-nano-2026-03-17",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54Pro {
            model_name: "gpt-5.4-pro",
            constructor_name: gpt_5_4_pro,
            display_name: "gpt-5.4-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok41FastNonReasoning {
            model_name: "grok-4-1-fast-non-reasoning",
            constructor_name: grok_4_1_fast_non_reasoning,
            display_name: "grok-4-1-fast-non-reasoning",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok41FastReasoning {
            model_name: "grok-4-1-fast-reasoning",
            constructor_name: grok_4_1_fast_reasoning,
            display_name: "grok-4-1-fast-reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4FastNonReasoning {
            model_name: "grok-4-fast-non-reasoning",
            constructor_name: grok_4_fast_non_reasoning,
            display_name: "grok-4-fast-non-reasoning",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4FastReasoning {
            model_name: "grok-4-fast-reasoning",
            constructor_name: grok_4_fast_reasoning,
            display_name: "grok-4-fast-reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok41 {
            model_name: "grok-4.1",
            constructor_name: grok_4_1,
            display_name: "grok-4.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok420Beta0309NonReasoning {
            model_name: "grok-4.20-beta-0309-non-reasoning",
            constructor_name: grok_4_20_beta_0309_non_reasoning,
            display_name: "grok-4.20-beta-0309-non-reasoning",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok420Beta0309Reasoning {
            model_name: "grok-4.20-beta-0309-reasoning",
            constructor_name: grok_4_20_beta_0309_reasoning,
            display_name: "grok-4.20-beta-0309-reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok420MultiAgentBeta0309 {
            model_name: "grok-4.20-multi-agent-beta-0309",
            constructor_name: grok_4_20_multi_agent_beta_0309,
            display_name: "grok-4.20-multi-agent-beta-0309",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK20905Preview {
            model_name: "kimi-k2-0905-preview",
            constructor_name: kimi_k2_0905_preview,
            display_name: "kimi-k2-0905-preview",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2Thinking {
            model_name: "kimi-k2-thinking",
            constructor_name: kimi_k2_thinking,
            display_name: "kimi-k2-thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2ThinkingTurbo {
            model_name: "kimi-k2-thinking-turbo",
            constructor_name: kimi_k2_thinking_turbo,
            display_name: "kimi-k2-thinking-turbo",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ministral14b2512 {
            model_name: "ministral-14b-2512",
            constructor_name: ministral_14b_2512,
            display_name: "ministral-14b-2512",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralLarge2512 {
            model_name: "mistral-large-2512",
            constructor_name: mistral_large_2512,
            display_name: "mistral-large-2512",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenFlash {
            model_name: "qwen-flash",
            constructor_name: qwen_flash,
            display_name: "Qwen-Flash",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenMaxLatest {
            model_name: "qwen-max-latest",
            constructor_name: qwen_max_latest,
            display_name: "Qwen-Max-Latest",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenPlus {
            model_name: "qwen-plus",
            constructor_name: qwen_plus,
            display_name: "Qwen-Plus",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3235bA22b {
            model_name: "qwen3-235b-a22b",
            constructor_name: qwen3_235b_a22b,
            display_name: "Qwen3-235B-A22B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3235bA22bInstruct2507 {
            model_name: "qwen3-235b-a22b-instruct-2507",
            constructor_name: qwen3_235b_a22b_instruct_2507,
            display_name: "qwen3-235b-a22b-instruct-2507",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen330bA3b {
            model_name: "qwen3-30b-a3b",
            constructor_name: qwen3_30b_a3b,
            display_name: "Qwen3-30B-A3B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Coder480bA35bInstruct {
            model_name: "qwen3-coder-480b-a35b-instruct",
            constructor_name: qwen3_coder_480b_a35b_instruct,
            display_name: "qwen3-coder-480b-a35b-instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Max20250923 {
            model_name: "qwen3-max-2025-09-23",
            constructor_name: qwen3_max_2025_09_23,
            display_name: "qwen3-max-2025-09-23",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
