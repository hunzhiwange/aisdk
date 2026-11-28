//! Capabilities for edenai models.
//!
//! This module defines model types and their capabilities for edenai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::edenai::Edenai;

model_capabilities! {
    provider: Edenai,
    models: {
        Beam {
            model_name: "beam",
            constructor_name: beam,
            display_name: "Beam 1.0 501B A23B (beam)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        C4aiAyaExpanse32b {
            model_name: "c4ai-aya-expanse-32b",
            constructor_name: c4ai_aya_expanse_32b,
            display_name: "Aya Expanse 32B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ClaudeFable5 {
            model_name: "claude-fable-5",
            constructor_name: claude_fable_5,
            display_name: "Claude Fable 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeFable52 {
            model_name: "claude-fable-5-2",
            constructor_name: claude_fable_5_2,
            display_name: "Claude Fable 5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeFable51 {
            model_name: "claude-fable-5.1",
            constructor_name: claude_fable_5_1,
            display_name: "Claude Fable 5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku55 {
            model_name: "claude-haiku-5.5",
            constructor_name: claude_haiku_5_5,
            display_name: "Claude Haiku 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus45 {
            model_name: "claude-opus-4.5",
            constructor_name: claude_opus_4_5,
            display_name: "Claude Opus 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus46 {
            model_name: "claude-opus-4.6",
            constructor_name: claude_opus_4_6,
            display_name: "Claude Opus 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus47 {
            model_name: "claude-opus-4.7",
            constructor_name: claude_opus_4_7,
            display_name: "Claude Opus 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus48 {
            model_name: "claude-opus-4.8",
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
        ClaudeSonnet46 {
            model_name: "claude-sonnet-4.6",
            constructor_name: claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6",
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
        Codestral2508 {
            model_name: "codestral-2508",
            constructor_name: codestral_2508,
            display_name: "Codestral 25.08",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CodestralLatest {
            model_name: "codestral-latest",
            constructor_name: codestral_latest,
            display_name: "Codestral (latest)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CommandA032025 {
            model_name: "command-a-03-2025",
            constructor_name: command_a_03_2025,
            display_name: "Command A",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CommandATranslate082025 {
            model_name: "command-a-translate-08-2025",
            constructor_name: command_a_translate_08_2025,
            display_name: "Command A Translate",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CommandAVision072025 {
            model_name: "command-a-vision-07-2025",
            constructor_name: command_a_vision_07_2025,
            display_name: "Command A Vision",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CommandR082024 {
            model_name: "command-r-08-2024",
            constructor_name: command_r_08_2024,
            display_name: "Command R",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CommandRPlus082024 {
            model_name: "command-r-plus-08-2024",
            constructor_name: command_r_plus_08_2024,
            display_name: "Command R+",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CommandR7b122024 {
            model_name: "command-r7b-12-2024",
            constructor_name: command_r7b_12_2024,
            display_name: "Command R7B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepResearchMaxPreview042026 {
            model_name: "deep-research-max-preview-04-2026",
            constructor_name: deep_research_max_preview_04_2026,
            display_name: "Deep Research Max Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepResearchPreview042026 {
            model_name: "deep-research-preview-04-2026",
            constructor_name: deep_research_preview_04_2026,
            display_name: "Gemini Deep Research Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekChat {
            model_name: "deepseek-chat",
            constructor_name: deepseek_chat,
            display_name: "DeepSeek Chat",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR1 {
            model_name: "deepseek-r1",
            constructor_name: deepseek_r1,
            display_name: "DeepSeek-R1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR10528 {
            model_name: "deepseek-r1-0528",
            constructor_name: deepseek_r1_0528,
            display_name: "DeepSeek R1 0528",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV3 {
            model_name: "deepseek-v3",
            constructor_name: deepseek_v3,
            display_name: "DeepSeek-V3",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV30324 {
            model_name: "deepseek-v3-0324",
            constructor_name: deepseek_v3_0324,
            display_name: "DeepSeek V3 0324",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Flash {
            model_name: "deepseek-v4-flash",
            constructor_name: deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Flash0731 {
            model_name: "deepseek-v4-flash-0731",
            constructor_name: deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4FlashVisionExp {
            model_name: "deepseek-v4-flash-vision-exp",
            constructor_name: deepseek_v4_flash_vision_exp,
            display_name: "DeepSeek V4 Flash Vision Exp",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Pro {
            model_name: "deepseek-v4-pro",
            constructor_name: deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Pro0813 {
            model_name: "deepseek-v4-pro-0813",
            constructor_name: deepseek_v4_pro_0813,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash {
            model_name: "deepseek-v4.1-flash",
            constructor_name: deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash1017 {
            model_name: "deepseek-v4.1-flash-1017",
            constructor_name: deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Devstral2512 {
            model_name: "devstral-2512",
            constructor_name: devstral_2512,
            display_name: "Devstral 2",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DevstralMediumLatest {
            model_name: "devstral-medium-latest",
            constructor_name: devstral_medium_latest,
            display_name: "Devstral 2 (latest)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ember1 {
            model_name: "ember-1",
            constructor_name: ember_1,
            display_name: "Ember-1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashImage {
            model_name: "gemini-2.5-flash-image",
            constructor_name: gemini_2_5_flash_image,
            display_name: "Nano Banana",
            capabilities: [AudioInputSupport, ImageInputSupport, ImageOutputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Gemini3FlashPreview {
            model_name: "gemini-3-flash-preview",
            constructor_name: gemini_3_flash_preview,
            display_name: "Gemini 3 Flash Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini3ProImage {
            model_name: "gemini-3-pro-image",
            constructor_name: gemini_3_pro_image,
            display_name: "Nano Banana Pro",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini3ProImagePreview {
            model_name: "gemini-3-pro-image-preview",
            constructor_name: gemini_3_pro_image_preview,
            display_name: "Nano Banana Pro Preview",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini31FlashImage {
            model_name: "gemini-3.1-flash-image",
            constructor_name: gemini_3_1_flash_image,
            display_name: "Nano Banana 2",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini31FlashImagePreview {
            model_name: "gemini-3.1-flash-image-preview",
            constructor_name: gemini_3_1_flash_image_preview,
            display_name: "Nano Banana 2 Preview",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini31FlashLite {
            model_name: "gemini-3.1-flash-lite",
            constructor_name: gemini_3_1_flash_lite,
            display_name: "Gemini 3.1 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini31FlashLiteImage {
            model_name: "gemini-3.1-flash-lite-image",
            constructor_name: gemini_3_1_flash_lite_image,
            display_name: "Nano Banana 2 Lite",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini31FlashLitePreview {
            model_name: "gemini-3.1-flash-lite-preview",
            constructor_name: gemini_3_1_flash_lite_preview,
            display_name: "Gemini 3.1 Flash Lite Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini31ProPreview {
            model_name: "gemini-3.1-pro-preview",
            constructor_name: gemini_3_1_pro_preview,
            display_name: "Gemini 3.1 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini31ProPreviewCustomtools {
            model_name: "gemini-3.1-pro-preview-customtools",
            constructor_name: gemini_3_1_pro_preview_customtools,
            display_name: "Gemini 3.1 Pro Preview Custom Tools",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini35Flash {
            model_name: "gemini-3.5-flash",
            constructor_name: gemini_3_5_flash,
            display_name: "Gemini 3.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini35FlashLite {
            model_name: "gemini-3.5-flash-lite",
            constructor_name: gemini_3_5_flash_lite,
            display_name: "Gemini 3.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini36Flash {
            model_name: "gemini-3.6-flash",
            constructor_name: gemini_3_6_flash,
            display_name: "Gemini 3.6 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini37Flash {
            model_name: "gemini-3.7-flash",
            constructor_name: gemini_3_7_flash,
            display_name: "Gemini 3.7 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini38Flash {
            model_name: "gemini-3.8-flash",
            constructor_name: gemini_3_8_flash,
            display_name: "Gemini 3.8 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini4Argon {
            model_name: "gemini-4-argon",
            constructor_name: gemini_4_argon,
            display_name: "Gemini 4 Argon",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini4Carbon {
            model_name: "gemini-4-carbon",
            constructor_name: gemini_4_carbon,
            display_name: "Gemini 4 Carbon",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini4Hydrogen {
            model_name: "gemini-4-hydrogen",
            constructor_name: gemini_4_hydrogen,
            display_name: "Gemini 4 Hydrogen",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini4Neon {
            model_name: "gemini-4-neon",
            constructor_name: gemini_4_neon,
            display_name: "Gemini 4 Neon",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GeminiNanoBanana21 {
            model_name: "gemini-nano-banana-2.1",
            constructor_name: gemini_nano_banana_2_1,
            display_name: "Nano Banana 2.1",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GeminiRoboticsEr2Preview {
            model_name: "gemini-robotics-er-2-preview",
            constructor_name: gemini_robotics_er_2_preview,
            display_name: "Gemini Robotics-ER 2 Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemma312bIt {
            model_name: "gemma-3-12b-it",
            constructor_name: gemma_3_12b_it,
            display_name: "Gemma 3 12B IT",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma327bIt {
            model_name: "gemma-3-27b-it",
            constructor_name: gemma_3_27b_it,
            display_name: "Gemma 3 27B IT",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma34bIt {
            model_name: "gemma-3-4b-it",
            constructor_name: gemma_3_4b_it,
            display_name: "Gemma 3 4B IT",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GemmaSeaLionV427bIt {
            model_name: "gemma-sea-lion-v4-27b-it",
            constructor_name: gemma_sea_lion_v4_27b_it,
            display_name: "Gemma-SEA-LION-v4-27B-IT",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm46 {
            model_name: "glm-4.6",
            constructor_name: glm_4_6,
            display_name: "GLM-4.6",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm47Flash {
            model_name: "glm-4.7-flash",
            constructor_name: glm_4_7_flash,
            display_name: "GLM-4.7-Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm5 {
            model_name: "glm-5",
            constructor_name: glm_5,
            display_name: "GLM-5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm5Turbo {
            model_name: "glm-5-turbo",
            constructor_name: glm_5_turbo,
            display_name: "GLM-5-Turbo",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        Glm53 {
            model_name: "glm-5.3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Fast {
            model_name: "glm-5.3-fast",
            constructor_name: glm_5_3_fast,
            display_name: "GLM-5.3 (glm-5.3-fast)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Flash {
            model_name: "glm-5.3-flash",
            constructor_name: glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm53FlashxLegal {
            model_name: "glm-5.3-flashx-legal",
            constructor_name: glm_5_3_flashx_legal,
            display_name: "GLM-5.3-FlashX-Legal",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm5vTurbo {
            model_name: "glm-5v-turbo",
            constructor_name: glm_5v_turbo,
            display_name: "GLM-5V-Turbo",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gpt35Turbo {
            model_name: "gpt-3.5-turbo",
            constructor_name: gpt_3_5_turbo,
            display_name: "GPT-3.5-turbo",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4 {
            model_name: "gpt-4",
            constructor_name: gpt_4,
            display_name: "GPT-4",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41 {
            model_name: "gpt-4.1",
            constructor_name: gpt_4_1,
            display_name: "GPT-4.1",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Mini {
            model_name: "gpt-4.1-mini",
            constructor_name: gpt_4_1_mini,
            display_name: "GPT-4.1 mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Nano {
            model_name: "gpt-4.1-nano",
            constructor_name: gpt_4_1_nano,
            display_name: "GPT-4.1 nano",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4o {
            model_name: "gpt-4o",
            constructor_name: gpt_4o,
            display_name: "GPT-4o",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4o20240806 {
            model_name: "gpt-4o-2024-08-06",
            constructor_name: gpt_4o_2024_08_06,
            display_name: "GPT-4o (2024-08-06)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4o20241120 {
            model_name: "gpt-4o-2024-11-20",
            constructor_name: gpt_4o_2024_11_20,
            display_name: "GPT-4o (2024-11-20)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4oMini {
            model_name: "gpt-4o-mini",
            constructor_name: gpt_4o_mini,
            display_name: "GPT-4o mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5 {
            model_name: "gpt-5",
            constructor_name: gpt_5,
            display_name: "GPT-5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Mini {
            model_name: "gpt-5-mini",
            constructor_name: gpt_5_mini,
            display_name: "GPT-5 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Nano {
            model_name: "gpt-5-nano",
            constructor_name: gpt_5_nano,
            display_name: "GPT-5 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5Pro {
            model_name: "gpt-5-pro",
            constructor_name: gpt_5_pro,
            display_name: "GPT-5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt5SearchApi {
            model_name: "gpt-5-search-api",
            constructor_name: gpt_5_search_api,
            display_name: "GPT-5 Search API",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Gpt51 {
            model_name: "gpt-5.1",
            constructor_name: gpt_5_1,
            display_name: "GPT-5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51Codex {
            model_name: "gpt-5.1-codex",
            constructor_name: gpt_5_1_codex,
            display_name: "GPT-5.1 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51CodexMax {
            model_name: "gpt-5.1-codex-max",
            constructor_name: gpt_5_1_codex_max,
            display_name: "GPT-5.1 Codex Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt51CodexMini {
            model_name: "gpt-5.1-codex-mini",
            constructor_name: gpt_5_1_codex_mini,
            display_name: "GPT-5.1 Codex mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52 {
            model_name: "gpt-5.2",
            constructor_name: gpt_5_2,
            display_name: "GPT-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52Codex {
            model_name: "gpt-5.2-codex",
            constructor_name: gpt_5_2_codex,
            display_name: "GPT-5.2 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52Pro {
            model_name: "gpt-5.2-pro",
            constructor_name: gpt_5_2_pro,
            display_name: "GPT-5.2 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt53Codex {
            model_name: "gpt-5.3-codex",
            constructor_name: gpt_5_3_codex,
            display_name: "GPT-5.3 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54 {
            model_name: "gpt-5.4",
            constructor_name: gpt_5_4,
            display_name: "GPT-5.4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54Mini {
            model_name: "gpt-5.4-mini",
            constructor_name: gpt_5_4_mini,
            display_name: "GPT-5.4 mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54Nano {
            model_name: "gpt-5.4-nano",
            constructor_name: gpt_5_4_nano,
            display_name: "GPT-5.4 nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54Pro {
            model_name: "gpt-5.4-pro",
            constructor_name: gpt_5_4_pro,
            display_name: "GPT-5.4 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt55 {
            model_name: "gpt-5.5",
            constructor_name: gpt_5_5,
            display_name: "GPT-5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt55Pro {
            model_name: "gpt-5.5-pro",
            constructor_name: gpt_5_5_pro,
            display_name: "GPT-5.5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Luna {
            model_name: "gpt-5.6-luna",
            constructor_name: gpt_5_6_luna,
            display_name: "GPT-5.6 Luna",
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
        Gpt6AstraHealthcare {
            model_name: "gpt-6-astra-healthcare",
            constructor_name: gpt_6_astra_healthcare,
            display_name: "GPT-6 Astra Healthcare",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Luna {
            model_name: "gpt-6-luna",
            constructor_name: gpt_6_luna,
            display_name: "GPT-6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Sol {
            model_name: "gpt-6-sol",
            constructor_name: gpt_6_sol,
            display_name: "GPT-6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt61Sol {
            model_name: "gpt-6.1-sol",
            constructor_name: gpt_6_1_sol,
            display_name: "GPT-6.1 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt61SolHealthcare {
            model_name: "gpt-6.1-sol-healthcare",
            constructor_name: gpt_6_1_sol_healthcare,
            display_name: "GPT-6.1 Sol Healthcare",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt61Terra {
            model_name: "gpt-6.1-terra",
            constructor_name: gpt_6_1_terra,
            display_name: "GPT-6.1 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt62Astra {
            model_name: "gpt-6.2-astra",
            constructor_name: gpt_6_2_astra,
            display_name: "GPT-6.2 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt62AstraHealthcare {
            model_name: "gpt-6.2-astra-healthcare",
            constructor_name: gpt_6_2_astra_healthcare,
            display_name: "GPT-6.2 Astra Healthcare",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt62Sol {
            model_name: "gpt-6.2-sol",
            constructor_name: gpt_6_2_sol,
            display_name: "GPT-6.2 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt62SolHealthcare {
            model_name: "gpt-6.2-sol-healthcare",
            constructor_name: gpt_6_2_sol_healthcare,
            display_name: "GPT-6.2 Sol Healthcare",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss20b {
            model_name: "gpt-oss-20b",
            constructor_name: gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOssSafeguard20b {
            model_name: "gpt-oss-safeguard-20b",
            constructor_name: gpt_oss_safeguard_20b,
            display_name: "GPT OSS Safeguard 20B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Granite428b {
            model_name: "granite-4.2-8b",
            constructor_name: granite_4_2_8b,
            display_name: "Granite 4.2 8B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4FastNonReasoning {
            model_name: "grok-4-fast-non-reasoning",
            constructor_name: grok_4_fast_non_reasoning,
            display_name: "Grok 4 Fast (Non-Reasoning)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4200309NonReasoning {
            model_name: "grok-4.20-0309-non-reasoning",
            constructor_name: grok_4_20_0309_non_reasoning,
            display_name: "Grok 4.20 (Non-Reasoning)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4200309Reasoning {
            model_name: "grok-4.20-0309-reasoning",
            constructor_name: grok_4_20_0309_reasoning,
            display_name: "Grok 4.20 (Reasoning)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok420MultiAgent0309 {
            model_name: "grok-4.20-multi-agent-0309",
            constructor_name: grok_4_20_multi_agent_0309,
            display_name: "Grok 4.20 Multi-Agent",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok43 {
            model_name: "grok-4.3",
            constructor_name: grok_4_3,
            display_name: "Grok 4.3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok45 {
            model_name: "grok-4.5",
            constructor_name: grok_4_5,
            display_name: "Grok 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok46 {
            model_name: "grok-4.6",
            constructor_name: grok_4_6,
            display_name: "Grok 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok47 {
            model_name: "grok-4.7",
            constructor_name: grok_4_7,
            display_name: "Grok 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hermes3Llama31405b {
            model_name: "hermes-3-llama-3.1-405b",
            constructor_name: hermes_3_llama_3_1_405b,
            display_name: "Hermes 3 405B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hermes3Llama3170b {
            model_name: "hermes-3-llama-3.1-70b",
            constructor_name: hermes_3_llama_3_1_70b,
            display_name: "Hermes 3 70B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hy3 {
            model_name: "hy3",
            constructor_name: hy3,
            display_name: "Hy3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hy4 {
            model_name: "hy4",
            constructor_name: hy4,
            display_name: "Hy4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hy4Preview {
            model_name: "hy4-preview",
            constructor_name: hy4_preview,
            display_name: "Hy4 preview",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Inkling {
            model_name: "inkling",
            constructor_name: inkling,
            display_name: "Inkling",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Inkling2Mini {
            model_name: "inkling-2-mini",
            constructor_name: inkling_2_mini,
            display_name: "Inkling 2 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Inkling2Small {
            model_name: "inkling-2-small",
            constructor_name: inkling_2_small,
            display_name: "Inkling 2 Small",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InklingSmall {
            model_name: "inkling-small",
            constructor_name: inkling_small,
            display_name: "Inkling Small",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2Instruct {
            model_name: "kimi-k2-instruct",
            constructor_name: kimi_k2_instruct,
            display_name: "Kimi K2 Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2Thinking {
            model_name: "kimi-k2-thinking",
            constructor_name: kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK25 {
            model_name: "kimi-k2.5",
            constructor_name: kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK26 {
            model_name: "kimi-k2.6",
            constructor_name: kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK27Code {
            model_name: "kimi-k2.7-code",
            constructor_name: kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK27CodeHighspeed {
            model_name: "kimi-k2.7-code-highspeed",
            constructor_name: kimi_k2_7_code_highspeed,
            display_name: "Kimi K2.7 Code Highspeed",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK3 {
            model_name: "kimi-k3",
            constructor_name: kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK3Fast {
            model_name: "kimi-k3-fast",
            constructor_name: kimi_k3_fast,
            display_name: "Kimi K3 (kimi-k3-fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK31Pro {
            model_name: "kimi-k3.1-pro",
            constructor_name: kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK31ProHighspeed {
            model_name: "kimi-k3.1-pro-highspeed",
            constructor_name: kimi_k3_1_pro_highspeed,
            display_name: "Kimi K3.1 Pro Highspeed",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Llama3170bInstruct {
            model_name: "llama-3.1-70b-instruct",
            constructor_name: llama_3_1_70b_instruct,
            display_name: "Llama-3.1-70B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama318bInstruct {
            model_name: "llama-3.1-8b-instruct",
            constructor_name: llama_3_1_8b_instruct,
            display_name: "Llama-3.1-8B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama31Nemotron70bInstruct {
            model_name: "llama-3.1-nemotron-70b-instruct",
            constructor_name: llama_3_1_nemotron_70b_instruct,
            display_name: "Llama 3.1 Nemotron 70B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama3211bVisionInstruct {
            model_name: "llama-3.2-11b-vision-instruct",
            constructor_name: llama_3_2_11b_vision_instruct,
            display_name: "Llama-3.2-11B-Vision-Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama323bInstruct {
            model_name: "llama-3.2-3b-instruct",
            constructor_name: llama_3_2_3b_instruct,
            display_name: "Llama 3.2 3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama3370bInstruct {
            model_name: "llama-3.3-70b-instruct",
            constructor_name: llama_3_3_70b_instruct,
            display_name: "Llama-3.3-70B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LlamaGuard38b {
            model_name: "llama-guard-3-8b",
            constructor_name: llama_guard_3_8b,
            display_name: "Llama-Guard-3-8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        LlamaGuard412b {
            model_name: "llama-guard-4-12b",
            constructor_name: llama_guard_4_12b,
            display_name: "Llama Guard 4 12B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        LlamaPromptGuard222m {
            model_name: "llama-prompt-guard-2-22m",
            constructor_name: llama_prompt_guard_2_22m,
            display_name: "Llama Prompt Guard 2 22M",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        LlamaPromptGuard286m {
            model_name: "llama-prompt-guard-2-86m",
            constructor_name: llama_prompt_guard_2_86m,
            display_name: "Llama Prompt Guard 2 86M",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MagistralMediumLatest {
            model_name: "magistral-medium-latest",
            constructor_name: magistral_medium_latest,
            display_name: "Magistral Medium (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM2 {
            model_name: "minimax-m2",
            constructor_name: minimax_m2,
            display_name: "MiniMax-M2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM21 {
            model_name: "minimax-m2.1",
            constructor_name: minimax_m2_1,
            display_name: "MiniMax-M2.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM25 {
            model_name: "minimax-m2.5",
            constructor_name: minimax_m2_5,
            display_name: "MiniMax-M2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM27 {
            model_name: "minimax-m2.7",
            constructor_name: minimax_m2_7,
            display_name: "MiniMax-M2.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM3 {
            model_name: "minimax-m3",
            constructor_name: minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MinimaxM31Flash {
            model_name: "minimax-m3.1-flash",
            constructor_name: minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Ministral14b {
            model_name: "ministral-14b",
            constructor_name: ministral_14b,
            display_name: "Ministral 14B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Ministral14b2512 {
            model_name: "ministral-14b-2512",
            constructor_name: ministral_14b_2512,
            display_name: "Ministral 3 14B 2512",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ministral314bInstruct2512 {
            model_name: "ministral-3-14b-instruct-2512",
            constructor_name: ministral_3_14b_instruct_2512,
            display_name: "Ministral 3 14B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ministral3b2512 {
            model_name: "ministral-3b-2512",
            constructor_name: ministral_3b_2512,
            display_name: "Ministral 3 3B 2512",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Ministral8b2512 {
            model_name: "ministral-8b-2512",
            constructor_name: ministral_8b_2512,
            display_name: "Ministral 3 8B 2512",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralLarge2407 {
            model_name: "mistral-large-2407",
            constructor_name: mistral_large_2407,
            display_name: "Mistral Large 2407",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralLarge2512 {
            model_name: "mistral-large-2512",
            constructor_name: mistral_large_2512,
            display_name: "Mistral Large 3",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralLarge4 {
            model_name: "mistral-large-4",
            constructor_name: mistral_large_4,
            display_name: "Mistral Large 4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralLargeLatest {
            model_name: "mistral-large-latest",
            constructor_name: mistral_large_latest,
            display_name: "Mistral Large (latest)",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMedium2604 {
            model_name: "mistral-medium-2604",
            constructor_name: mistral_medium_2604,
            display_name: "Mistral Medium 3.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMedium3 {
            model_name: "mistral-medium-3",
            constructor_name: mistral_medium_3,
            display_name: "Mistral Medium 3",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMedium35 {
            model_name: "mistral-medium-3.5",
            constructor_name: mistral_medium_3_5,
            display_name: "Mistral Medium 3.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMediumLatest {
            model_name: "mistral-medium-latest",
            constructor_name: mistral_medium_latest,
            display_name: "Mistral Medium (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralSmall24bInstruct2501 {
            model_name: "mistral-small-24b-instruct-2501",
            constructor_name: mistral_small_24b_instruct_2501,
            display_name: "Mistral Small 3",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralSmall2603 {
            model_name: "mistral-small-2603",
            constructor_name: mistral_small_2603,
            display_name: "Mistral Small 4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralSmall3124bInstruct {
            model_name: "mistral-small-3.1-24b-instruct",
            constructor_name: mistral_small_3_1_24b_instruct,
            display_name: "Mistral Small 3.1 24B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralSmallLatest {
            model_name: "mistral-small-latest",
            constructor_name: mistral_small_latest,
            display_name: "Mistral Small (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Mixtral8x7bInstructV01 {
            model_name: "mixtral-8x7b-instruct-v0.1",
            constructor_name: mixtral_8x7b_instruct_v0_1,
            display_name: "Mixtral 8x7B Instruct v0.1",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        MuseGlimmer30b {
            model_name: "muse-glimmer-30b",
            constructor_name: muse_glimmer_30b,
            display_name: "Muse Glimmer 30B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MuseSpark11 {
            model_name: "muse-spark-1.1",
            constructor_name: muse_spark_1_1,
            display_name: "Muse Spark 1.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MuseSpark12 {
            model_name: "muse-spark-1.2",
            constructor_name: muse_spark_1_2,
            display_name: "Muse Spark 1.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MuseSpark13 {
            model_name: "muse-spark-1.3",
            constructor_name: muse_spark_1_3,
            display_name: "Muse Spark 1.3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MuseSparkOpen {
            model_name: "muse-spark-open",
            constructor_name: muse_spark_open,
            display_name: "Muse Spark Open",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MuseWonder10 {
            model_name: "muse-wonder-1.0",
            constructor_name: muse_wonder_1_0,
            display_name: "Muse Wonder",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MythomaxL213b {
            model_name: "mythomax-l2-13b",
            constructor_name: mythomax_l2_13b,
            display_name: "MythoMax L2 13B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Nemotron3Nano30bA3b {
            model_name: "nemotron-3-nano-30b-a3b",
            constructor_name: nemotron_3_nano_30b_a3b,
            display_name: "Nemotron 3 Nano 30B A3B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Nemotron3Super120bA12b {
            model_name: "nemotron-3-super-120b-a12b",
            constructor_name: nemotron_3_super_120b_a12b,
            display_name: "Nemotron 3 Super 120B A12B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Nemotron3Ultra550bA55b {
            model_name: "nemotron-3-ultra-550b-a55b",
            constructor_name: nemotron_3_ultra_550b_a55b,
            display_name: "Nemotron 3 Ultra 550B A55B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NovaLite {
            model_name: "nova-lite",
            constructor_name: nova_lite,
            display_name: "Nova Lite",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NovaMicro {
            model_name: "nova-micro",
            constructor_name: nova_micro,
            display_name: "Nova Micro",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NovaPro {
            model_name: "nova-pro",
            constructor_name: nova_pro,
            display_name: "Nova Pro",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        O3 {
            model_name: "o3",
            constructor_name: o3,
            display_name: "o3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        O3Pro {
            model_name: "o3-pro",
            constructor_name: o3_pro,
            display_name: "o3-pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PalmyraVision7b {
            model_name: "palmyra-vision-7b",
            constructor_name: palmyra_vision_7b,
            display_name: "Palmyra Vision 7B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        PixtralLarge2502 {
            model_name: "pixtral-large-2502",
            constructor_name: pixtral_large_2502,
            display_name: "Pixtral Large (25.02)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenMax {
            model_name: "qwen-max",
            constructor_name: qwen_max,
            display_name: "Qwen Max",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenMtPlus {
            model_name: "qwen-mt-plus",
            constructor_name: qwen_mt_plus,
            display_name: "Qwen MT Plus",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwen257bInstruct {
            model_name: "qwen2.5-7b-instruct",
            constructor_name: qwen2_5_7b_instruct,
            display_name: "Qwen2.5 7B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen25Coder32bInstruct {
            model_name: "qwen2.5-coder-32b-instruct",
            constructor_name: qwen2_5_coder_32b_instruct,
            display_name: "Qwen2.5-Coder-32B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3235bA22bInstruct2507 {
            model_name: "qwen3-235b-a22b-instruct-2507",
            constructor_name: qwen3_235b_a22b_instruct_2507,
            display_name: "Qwen3 235B-A22B Instruct 2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3235bA22bThinking2507 {
            model_name: "qwen3-235b-a22b-thinking-2507",
            constructor_name: qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen3 235B-A22B Thinking 2507",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen330bA3bInstruct2507 {
            model_name: "qwen3-30b-a3b-instruct-2507",
            constructor_name: qwen3_30b_a3b_instruct_2507,
            display_name: "Qwen3 30B-A3B Instruct 2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Coder30bA3bInstruct {
            model_name: "qwen3-coder-30b-a3b-instruct",
            constructor_name: qwen3_coder_30b_a3b_instruct,
            display_name: "Qwen3-Coder 30B-A3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Coder480bA35bInstruct {
            model_name: "qwen3-coder-480b-a35b-instruct",
            constructor_name: qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3-Coder 480B-A35B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3CoderFlash {
            model_name: "qwen3-coder-flash",
            constructor_name: qwen3_coder_flash,
            display_name: "Qwen3 Coder Flash",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3CoderNext {
            model_name: "qwen3-coder-next",
            constructor_name: qwen3_coder_next,
            display_name: "Qwen3 Coder Next",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Max {
            model_name: "qwen3-max",
            constructor_name: qwen3_max,
            display_name: "Qwen3 Max",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Next80bA3bInstruct {
            model_name: "qwen3-next-80b-a3b-instruct",
            constructor_name: qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3-Next 80B-A3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Vl235bA22bInstruct {
            model_name: "qwen3-vl-235b-a22b-instruct",
            constructor_name: qwen3_vl_235b_a22b_instruct,
            display_name: "Qwen3 VL 235B A22B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Vl30bA3bInstruct {
            model_name: "qwen3-vl-30b-a3b-instruct",
            constructor_name: qwen3_vl_30b_a3b_instruct,
            display_name: "Qwen3 VL 30B A3B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3824tA95b {
            model_name: "qwen3.8-2.4t-a95b",
            constructor_name: qwen3_8_2_4t_a95b,
            display_name: "Qwen3.8 2.4T A95B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3827b {
            model_name: "qwen3.8-27b",
            constructor_name: qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen38Flash {
            model_name: "qwen3.8-flash",
            constructor_name: qwen3_8_flash,
            display_name: "Qwen3.8 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen38Max {
            model_name: "qwen3.8-max",
            constructor_name: qwen3_8_max,
            display_name: "Qwen3.8 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen38Max0902 {
            model_name: "qwen3.8-max-0902",
            constructor_name: qwen3_8_max_0902,
            display_name: "Qwen3.8 Max 0902",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen38OmniFlash {
            model_name: "qwen3.8-omni-flash",
            constructor_name: qwen3_8_omni_flash,
            display_name: "Qwen3.8 Omni Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen4Max {
            model_name: "qwen4-max",
            constructor_name: qwen4_max,
            display_name: "Qwen4 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Seed20Code {
            model_name: "seed-2.0-code",
            constructor_name: seed_2_0_code,
            display_name: "Seed 2.0 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Sonar {
            model_name: "sonar",
            constructor_name: sonar,
            display_name: "Sonar",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        SonarDeepResearch {
            model_name: "sonar-deep-research",
            constructor_name: sonar_deep_research,
            display_name: "Sonar Deep Research",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        SonarPro {
            model_name: "sonar-pro",
            constructor_name: sonar_pro,
            display_name: "Sonar Pro",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        SonarReasoningPro {
            model_name: "sonar-reasoning-pro",
            constructor_name: sonar_reasoning_pro,
            display_name: "Sonar Reasoning Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Step35Flash {
            model_name: "step-3.5-flash",
            constructor_name: step_3_5_flash,
            display_name: "Step 3.5 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Step37Flash {
            model_name: "step-3.7-flash",
            constructor_name: step_3_7_flash,
            display_name: "Step 3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Step5Preview {
            model_name: "step-5-preview",
            constructor_name: step_5_preview,
            display_name: "Step 5 Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        VoxtralMini3b2507 {
            model_name: "voxtral-mini-3b-2507",
            constructor_name: voxtral_mini_3b_2507,
            display_name: "Voxtral Mini 3B 2507",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        VoxtralSmall24b2507 {
            model_name: "voxtral-small-24b-2507",
            constructor_name: voxtral_small_24b_2507,
            display_name: "Voxtral Small 24B 2507",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        VoxtralSmallLatest {
            model_name: "voxtral-small-latest",
            constructor_name: voxtral_small_latest,
            display_name: "Voxtral Small (latest)",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Wizardlm28x22b {
            model_name: "wizardlm-2-8x22b",
            constructor_name: wizardlm_2_8x22b,
            display_name: "WizardLM 2 8x22B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
