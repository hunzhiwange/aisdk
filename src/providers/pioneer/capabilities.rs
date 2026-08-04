//! Capabilities for pioneer models.
//!
//! This module defines model types and their capabilities for pioneer providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::pioneer::Pioneer;

model_capabilities! {
    provider: Pioneer,
    models: {
        HuggingfacetbSmollm33bBase {
            model_name: "HuggingFaceTB/SmolLM3-3B-Base",
            constructor_name: huggingfacetb_smollm3_3b_base,
            display_name: "SmolLM3 3B Base",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        LiquidaiLfm224bA2b {
            model_name: "LiquidAI/LFM2-24B-A2B",
            constructor_name: liquidai_lfm2_24b_a2b,
            display_name: "LFM2 24B A2B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        MinimaxaiMinimaxM27 {
            model_name: "MiniMaxAI/MiniMax-M2.7",
            constructor_name: minimaxai_minimax_m2_7,
            display_name: "MiniMax-M2.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxaiMinimaxM3 {
            model_name: "MiniMaxAI/MiniMax-M3",
            constructor_name: minimaxai_minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen317bBase {
            model_name: "Qwen/Qwen3-1.7B-Base",
            constructor_name: qwen_qwen3_1_7b_base,
            display_name: "Qwen3 1.7B Base",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen332b {
            model_name: "Qwen/Qwen3-32B",
            constructor_name: qwen_qwen3_32b,
            display_name: "Qwen3 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen34bBase {
            model_name: "Qwen/Qwen3-4B-Base",
            constructor_name: qwen_qwen3_4b_base,
            display_name: "Qwen3 4B Base",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen34bInstruct2507 {
            model_name: "Qwen/Qwen3-4B-Instruct-2507",
            constructor_name: qwen_qwen3_4b_instruct_2507,
            display_name: "Qwen3 4B Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen38b {
            model_name: "Qwen/Qwen3-8B",
            constructor_name: qwen_qwen3_8b,
            display_name: "Qwen3 8B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen359b {
            model_name: "Qwen/Qwen3.5-9B",
            constructor_name: qwen_qwen3_5_9b,
            display_name: "Qwen3.5 9B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3627b {
            model_name: "Qwen/Qwen3.6-27B",
            constructor_name: qwen_qwen3_6_27b,
            display_name: "Qwen3.6 27B",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3635bA3b {
            model_name: "Qwen/Qwen3.6-35B-A3B",
            constructor_name: qwen_qwen3_6_35b_a3b,
            display_name: "Qwen3.6 35B-A3B",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XiaomimimoMimoV25 {
            model_name: "XiaomiMiMo/MiMo-V2.5",
            constructor_name: xiaomimimo_mimo_v2_5,
            display_name: "MiMo-V2.5",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XiaomimimoMimoV25Pro {
            model_name: "XiaomiMiMo/MiMo-V2.5-Pro",
            constructor_name: xiaomimimo_mimo_v2_5_pro,
            display_name: "MiMo-V2.5-Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku45 {
            model_name: "claude-haiku-4-5",
            constructor_name: claude_haiku_4_5,
            display_name: "Claude Haiku 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus41 {
            model_name: "claude-opus-4-1",
            constructor_name: claude_opus_4_1,
            display_name: "Claude Opus 4.1 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus45 {
            model_name: "claude-opus-4-5",
            constructor_name: claude_opus_4_5,
            display_name: "Claude Opus 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus46 {
            model_name: "claude-opus-4-6",
            constructor_name: claude_opus_4_6,
            display_name: "Claude Opus 4.6",
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
        ClaudeSonnet45 {
            model_name: "claude-sonnet-4-5",
            constructor_name: claude_sonnet_4_5,
            display_name: "Claude Sonnet 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet46 {
            model_name: "claude-sonnet-4-6",
            constructor_name: claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV4Flash {
            model_name: "deepseek-ai/DeepSeek-V4-Flash",
            constructor_name: deepseek_ai_deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV4Pro {
            model_name: "deepseek-ai/DeepSeek-V4-Pro",
            constructor_name: deepseek_ai_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        FastinoGliguardLlmguardrails300m {
            model_name: "fastino/gliguard-LLMGuardrails-300M",
            constructor_name: fastino_gliguard_llmguardrails_300m,
            display_name: "GLiGuard LLM Guardrails 300M",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        FastinoGliner2BaseV1 {
            model_name: "fastino/gliner2-base-v1",
            constructor_name: fastino_gliner2_base_v1,
            display_name: "GLiNER2 Base",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        FastinoGliner2LargeV1 {
            model_name: "fastino/gliner2-large-v1",
            constructor_name: fastino_gliner2_large_v1,
            display_name: "GLiNER2 Large",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        FastinoGliner2MultiLargeV1 {
            model_name: "fastino/gliner2-multi-large-v1",
            constructor_name: fastino_gliner2_multi_large_v1,
            display_name: "GLiNER2 Multi Large",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        FastinoGliner2MultiV1 {
            model_name: "fastino/gliner2-multi-v1",
            constructor_name: fastino_gliner2_multi_v1,
            display_name: "GLiNER2 Multi",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        FastinoGliner2PrivacyFilterPiiMulti {
            model_name: "fastino/gliner2-privacy-filter-PII-multi",
            constructor_name: fastino_gliner2_privacy_filter_pii_multi,
            display_name: "GLiNER2 Privacy Filter PII (Multi)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini3Flash {
            model_name: "gemini-3-flash",
            constructor_name: gemini_3_flash,
            display_name: "Gemini 3 Flash Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini31Pro {
            model_name: "gemini-3.1-pro",
            constructor_name: gemini_3_1_pro,
            display_name: "Gemini 3.1 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini35Flash {
            model_name: "gemini-3.5-flash",
            constructor_name: gemini_3_5_flash,
            display_name: "Gemini 3.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleDiffusiongemma26bA4bIt {
            model_name: "google/diffusiongemma-26B-A4B-it",
            constructor_name: google_diffusiongemma_26b_a4b_it,
            display_name: "DiffusionGemma 26B-A4B IT",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma34bPt {
            model_name: "google/gemma-3-4b-pt",
            constructor_name: google_gemma_3_4b_pt,
            display_name: "Gemma 3 4B (Pretrained)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma412bIt {
            model_name: "google/gemma-4-12B-it",
            constructor_name: google_gemma_4_12b_it,
            display_name: "Gemma 4 12B IT",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31B-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma 4 31B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma4E2bIt {
            model_name: "google/gemma-4-E2B-it",
            constructor_name: google_gemma_4_e2b_it,
            display_name: "Gemma 4 E2B IT",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma4E4bIt {
            model_name: "google/gemma-4-E4B-it",
            constructor_name: google_gemma_4_e4b_it,
            display_name: "Gemma 4 E4B IT",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41 {
            model_name: "gpt-4.1",
            constructor_name: gpt_4_1,
            display_name: "GPT-4.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Mini {
            model_name: "gpt-4.1-mini",
            constructor_name: gpt_4_1_mini,
            display_name: "GPT-4.1 mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt41Nano {
            model_name: "gpt-4.1-nano",
            constructor_name: gpt_4_1_nano,
            display_name: "GPT-4.1 nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4o {
            model_name: "gpt-4o",
            constructor_name: gpt_4o,
            display_name: "GPT-4o",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt4oMini {
            model_name: "gpt-4o-mini",
            constructor_name: gpt_4o_mini,
            display_name: "GPT-4o mini",
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
        Gpt51 {
            model_name: "gpt-5.1",
            constructor_name: gpt_5_1,
            display_name: "GPT-5.1",
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
        Gpt55 {
            model_name: "gpt-5.5",
            constructor_name: gpt_5_5,
            display_name: "GPT-5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama318bInstruct {
            model_name: "meta-llama/Llama-3.1-8B-Instruct",
            constructor_name: meta_llama_llama_3_1_8b_instruct,
            display_name: "Llama 3.1 8B Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama321bInstruct {
            model_name: "meta-llama/Llama-3.2-1B-Instruct",
            constructor_name: meta_llama_llama_3_2_1b_instruct,
            display_name: "Llama 3.2 1B Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama323bInstruct {
            model_name: "meta-llama/Llama-3.2-3B-Instruct",
            constructor_name: meta_llama_llama_3_2_3b_instruct,
            display_name: "Llama 3.2 3B Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama3370bInstruct {
            model_name: "meta-llama/Llama-3.3-70B-Instruct",
            constructor_name: meta_llama_llama_3_3_70b_instruct,
            display_name: "Llama-3.3-70B-Instruct",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMedium35 {
            model_name: "mistral-medium-3.5",
            constructor_name: mistral_medium_3_5,
            display_name: "Mistral Medium 3.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistral7bInstructV03 {
            model_name: "mistralai/Mistral-7B-Instruct-v0.3",
            constructor_name: mistralai_mistral_7b_instruct_v0_3,
            display_name: "Mistral 7B Instruct v0.3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralNemoInstruct2407 {
            model_name: "mistralai/Mistral-Nemo-Instruct-2407",
            constructor_name: mistralai_mistral_nemo_instruct_2407,
            display_name: "Mistral Nemo",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall4119b2603 {
            model_name: "mistralai/Mistral-Small-4-119B-2603",
            constructor_name: mistralai_mistral_small_4_119b_2603,
            display_name: "Mistral Small 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK26 {
            model_name: "moonshotai/Kimi-K2.6",
            constructor_name: moonshotai_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiK27Code {
            model_name: "moonshotai/Kimi-K2.7-Code",
            constructor_name: moonshotai_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NvidiaNvidiaNemotron3Nano30bA3bBf16 {
            model_name: "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B-BF16",
            constructor_name: nvidia_nvidia_nemotron_3_nano_30b_a3b_bf16,
            display_name: "Nemotron 3 Nano 30B A3B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNvidiaNemotron3Super120bA12bFp8 {
            model_name: "nvidia/NVIDIA-Nemotron-3-Super-120B-A12B-FP8",
            constructor_name: nvidia_nvidia_nemotron_3_super_120b_a12b_fp8,
            display_name: "Nemotron 3 Super 120B A12B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNvidiaNemotron3Ultra550bA55bBf16 {
            model_name: "nvidia/NVIDIA-Nemotron-3-Ultra-550B-A55B-BF16",
            constructor_name: nvidia_nvidia_nemotron_3_ultra_550b_a55b_bf16,
            display_name: "Nemotron 3 Ultra 550B A55B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20b {
            model_name: "openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PioneerAuto {
            model_name: "pioneer/auto",
            constructor_name: pioneer_auto,
            display_name: "Pioneer Auto",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen36Flash {
            model_name: "qwen3.6-flash",
            constructor_name: qwen3_6_flash,
            display_name: "Qwen3.6 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen36MaxPreview {
            model_name: "qwen3.6-max-preview",
            constructor_name: qwen3_6_max_preview,
            display_name: "Qwen3.6 Max Preview",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen36Plus {
            model_name: "qwen3.6-plus",
            constructor_name: qwen3_6_plus,
            display_name: "Qwen3.6 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen37Max {
            model_name: "qwen3.7-max",
            constructor_name: qwen3_7_max,
            display_name: "Qwen3.7 Max",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen37Plus {
            model_name: "qwen3.7-plus",
            constructor_name: qwen3_7_plus,
            display_name: "Qwen3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        SakanaFuguUltra {
            model_name: "sakana/fugu-ultra",
            constructor_name: sakana_fugu_ultra,
            display_name: "Fugu Ultra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm51 {
            model_name: "zai-org/GLM-5.1",
            constructor_name: zai_org_glm_5_1,
            display_name: "GLM-5.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm52 {
            model_name: "zai-org/GLM-5.2",
            constructor_name: zai_org_glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
