//! Capabilities for kenari models.
//!
//! This module defines model types and their capabilities for kenari providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::kenari::Kenari;

model_capabilities! {
    provider: Kenari,
    models: {
        Beam10Free {
            model_name: "beam-1-0:free",
            constructor_name: beam_1_0_free,
            display_name: "Beam 1.0 (Free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeFable5 {
            model_name: "claude-fable-5",
            constructor_name: claude_fable_5,
            display_name: "Claude Fable 5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku55 {
            model_name: "claude-haiku-5-5",
            constructor_name: claude_haiku_5_5,
            display_name: "Claude Haiku 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus47 {
            model_name: "claude-opus-4-7",
            constructor_name: claude_opus_4_7,
            display_name: "Claude Opus 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus48 {
            model_name: "claude-opus-4-8",
            constructor_name: claude_opus_4_8,
            display_name: "Claude Opus 4.8",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus5 {
            model_name: "claude-opus-5",
            constructor_name: claude_opus_5,
            display_name: "Claude Opus 5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus55 {
            model_name: "claude-opus-5-5",
            constructor_name: claude_opus_5_5,
            display_name: "Claude Opus 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet46 {
            model_name: "claude-sonnet-4-6",
            constructor_name: claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet5 {
            model_name: "claude-sonnet-5",
            constructor_name: claude_sonnet_5,
            display_name: "Claude Sonnet 5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet55 {
            model_name: "claude-sonnet-5-5",
            constructor_name: claude_sonnet_5_5,
            display_name: "Claude Sonnet 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash {
            model_name: "deepseek-v4-1-flash",
            constructor_name: deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
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
        Gemini31FlashLite {
            model_name: "gemini-3-1-flash-lite",
            constructor_name: gemini_3_1_flash_lite,
            display_name: "Gemini 3.1 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini31Pro {
            model_name: "gemini-3-1-pro",
            constructor_name: gemini_3_1_pro,
            display_name: "Gemini 3.1 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini38Flash {
            model_name: "gemini-3-8-flash",
            constructor_name: gemini_3_8_flash,
            display_name: "Gemini 3.8 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini38FlashLiteTts {
            model_name: "gemini-3-8-flash-lite-tts",
            constructor_name: gemini_3_8_flash_lite_tts,
            display_name: "Gemini 3.8 Flash Lite TTS",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        Glm46vFlashFree {
            model_name: "glm-4-6v-flash:free",
            constructor_name: glm_4_6v_flash_free,
            display_name: "GLM-4.6V-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm47FlashFree {
            model_name: "glm-4-7-flash:free",
            constructor_name: glm_4_7_flash_free,
            display_name: "GLM-4.7-Flash (Free)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52 {
            model_name: "glm-5-2",
            constructor_name: glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53 {
            model_name: "glm-5-3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Flash {
            model_name: "glm-5-3-flash",
            constructor_name: glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gpt56Luna {
            model_name: "gpt-5-6-luna",
            constructor_name: gpt_5_6_luna,
            display_name: "GPT-5.6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Sol {
            model_name: "gpt-5-6-sol",
            constructor_name: gpt_5_6_sol,
            display_name: "GPT-5.6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt61Sol {
            model_name: "gpt-6-1-sol",
            constructor_name: gpt_6_1_sol,
            display_name: "GPT-6.1 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt61Terra {
            model_name: "gpt-6-1-terra",
            constructor_name: gpt_6_1_terra,
            display_name: "GPT-6.1 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt62Astra {
            model_name: "gpt-6-2-astra",
            constructor_name: gpt_6_2_astra,
            display_name: "GPT-6.2 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt62Sol {
            model_name: "gpt-6-2-sol",
            constructor_name: gpt_6_2_sol,
            display_name: "GPT-6.2 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Astra {
            model_name: "gpt-6-astra",
            constructor_name: gpt_6_astra,
            display_name: "GPT-6 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Luna {
            model_name: "gpt-6-luna",
            constructor_name: gpt_6_luna,
            display_name: "GPT-6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Sol {
            model_name: "gpt-6-sol",
            constructor_name: gpt_6_sol,
            display_name: "GPT-6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptImage2 {
            model_name: "gpt-image-2",
            constructor_name: gpt_image_2,
            display_name: "GPT-Image-2",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        GptImage25Flare {
            model_name: "gpt-image-2-5-flare",
            constructor_name: gpt_image_2_5_flare,
            display_name: "GPT Image 2.5 Flare",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        GptImage25Sunburst {
            model_name: "gpt-image-2-5-sunburst",
            constructor_name: gpt_image_2_5_sunburst,
            display_name: "GPT Image 2.5 Sunburst",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        GptImage26Flare {
            model_name: "gpt-image-2-6-flare",
            constructor_name: gpt_image_2_6_flare,
            display_name: "GPT Image 2.6 Flare",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        GptImage26Sunburst {
            model_name: "gpt-image-2-6-sunburst",
            constructor_name: gpt_image_2_6_sunburst,
            display_name: "GPT Image 2.6 Sunburst",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
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
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptTranscribe {
            model_name: "gpt-transcribe",
            constructor_name: gpt_transcribe,
            display_name: "GPT-Transcribe",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Grok45 {
            model_name: "grok-4-5",
            constructor_name: grok_4_5,
            display_name: "Grok 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok46 {
            model_name: "grok-4-6",
            constructor_name: grok_4_6,
            display_name: "Grok 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok47 {
            model_name: "grok-4-7",
            constructor_name: grok_4_7,
            display_name: "Grok 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GrokImagineImage {
            model_name: "grok-imagine-image",
            constructor_name: grok_imagine_image,
            display_name: "Grok Imagine Image",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        GrokImagineImage20 {
            model_name: "grok-imagine-image-2-0",
            constructor_name: grok_imagine_image_2_0,
            display_name: "Grok Imagine Image 2.0",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        GrokImagineImageQuality {
            model_name: "grok-imagine-image-quality",
            constructor_name: grok_imagine_image_quality,
            display_name: "Grok Imagine Image Quality",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        Hy3 {
            model_name: "hy3",
            constructor_name: hy3,
            display_name: "Hy3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hy4 {
            model_name: "hy4",
            constructor_name: hy4,
            display_name: "Hy4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hy4Flash {
            model_name: "hy4-flash",
            constructor_name: hy4_flash,
            display_name: "Hy4 flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hy4FlashFree {
            model_name: "hy4-flash:free",
            constructor_name: hy4_flash_free,
            display_name: "Hy4 flash (Free)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Inkling2MiniUncensored {
            model_name: "inkling-2-mini-uncensored",
            constructor_name: inkling_2_mini_uncensored,
            display_name: "Inkling 2 Mini Uncensored",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK26 {
            model_name: "kimi-k2-6",
            constructor_name: kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK27Code {
            model_name: "kimi-k2-7-code",
            constructor_name: kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK3 {
            model_name: "kimi-k3",
            constructor_name: kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK31Pro {
            model_name: "kimi-k3-1-pro",
            constructor_name: kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Longcat25Free {
            model_name: "longcat-2-5:free",
            constructor_name: longcat_2_5_free,
            display_name: "LongCat-2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Lyria35 {
            model_name: "lyria-3-5",
            constructor_name: lyria_3_5,
            display_name: "Lyria 3.5",
            capabilities: [AudioOutputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MinimaxM27 {
            model_name: "minimax-m2-7",
            constructor_name: minimax_m2_7,
            display_name: "MiniMax-M2.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM27Highspeed {
            model_name: "minimax-m2-7-highspeed",
            constructor_name: minimax_m2_7_highspeed,
            display_name: "MiniMax-M2.7-highspeed",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM3 {
            model_name: "minimax-m3",
            constructor_name: minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MinimaxM31Flash {
            model_name: "minimax-m3-1-flash",
            constructor_name: minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MuseEmber10 {
            model_name: "muse-ember-1-0",
            constructor_name: muse_ember_1_0,
            display_name: "Muse Ember",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MuseSpark12 {
            model_name: "muse-spark-1-2",
            constructor_name: muse_spark_1_2,
            display_name: "Muse Spark 1.2",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MuseSpark12Contributor {
            model_name: "muse-spark-1-2-contributor",
            constructor_name: muse_spark_1_2_contributor,
            display_name: "Muse Spark 1.2 Contributor",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MuseSpark13 {
            model_name: "muse-spark-1-3",
            constructor_name: muse_spark_1_3,
            display_name: "Muse Spark 1.3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MuseSpark13Contributor {
            model_name: "muse-spark-1-3-contributor",
            constructor_name: muse_spark_1_3_contributor,
            display_name: "Muse Spark 1.3 Contributor",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MuseWonder10 {
            model_name: "muse-wonder-1-0",
            constructor_name: muse_wonder_1_0,
            display_name: "Muse Wonder",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NanoBanana2 {
            model_name: "nano-banana-2",
            constructor_name: nano_banana_2,
            display_name: "Nano Banana 2",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        NanoBanana2Lite {
            model_name: "nano-banana-2-lite",
            constructor_name: nano_banana_2_lite,
            display_name: "Nano Banana 2 Lite",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NanoBananaPro {
            model_name: "nano-banana-pro",
            constructor_name: nano_banana_pro,
            display_name: "Nano Banana Pro",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Nemotron3Super120bA12b {
            model_name: "nemotron-3-super-120b-a12b",
            constructor_name: nemotron_3_super_120b_a12b,
            display_name: "Nemotron 3 Super 120B A12B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Nemotron3Ultra550bA55b {
            model_name: "nemotron-3-ultra-550b-a55b",
            constructor_name: nemotron_3_ultra_550b_a55b,
            display_name: "Nemotron 3 Ultra 550B A55B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Nemotron3Ultra550bA55bFree {
            model_name: "nemotron-3-ultra-550b-a55b:free",
            constructor_name: nemotron_3_ultra_550b_a55b_free,
            display_name: "Nemotron 3 Ultra 550B A55B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Particle1Free {
            model_name: "particle-1:free",
            constructor_name: particle_1_free,
            display_name: "Particle-1 (Free)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen37Plus {
            model_name: "qwen3-7-plus",
            constructor_name: qwen3_7_plus,
            display_name: "Qwen3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen38Max {
            model_name: "qwen3-8-max",
            constructor_name: qwen3_8_max,
            display_name: "Qwen3.8 Max",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SolarMini4Free {
            model_name: "solar-mini4:free",
            constructor_name: solar_mini4_free,
            display_name: "Solar Mini 4",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Step37Flash {
            model_name: "step-3-7-flash",
            constructor_name: step_3_7_flash,
            display_name: "Step 3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Step5Preview {
            model_name: "step-5-preview",
            constructor_name: step_5_preview,
            display_name: "Step 5 Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Whisper1 {
            model_name: "whisper-1",
            constructor_name: whisper_1,
            display_name: "Whisper",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        WhisperLargeV3Turbo {
            model_name: "whisper-large-v3-turbo",
            constructor_name: whisper_large_v3_turbo,
            display_name: "Whisper Large v3 Turbo",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
    }
}
