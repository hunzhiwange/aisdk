//! Capabilities for nvidia models.
//!
//! This module defines model types and their capabilities for nvidia providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::nvidia::Nvidia;

model_capabilities! {
    provider: Nvidia,
    models: {
        AbacusaiDracarysLlama3170bInstruct {
            model_name: "abacusai/dracarys-llama-3_1-70b-instruct",
            constructor_name: abacusai_dracarys_llama_3_1_70b_instruct,
            display_name: "dracarys-llama-3.1-70b-instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BaaiBgeM3 {
            model_name: "baai/bge-m3",
            constructor_name: baai_bge_m3,
            display_name: "BGE M3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BlackForestLabsFlux1Dev {
            model_name: "black-forest-labs/flux.1-dev",
            constructor_name: black_forest_labs_flux_1_dev,
            display_name: "FLUX.1-dev",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BlackForestLabsFlux1KontextDev {
            model_name: "black-forest-labs/flux_1-kontext-dev",
            constructor_name: black_forest_labs_flux_1_kontext_dev,
            display_name: "FLUX.1-Kontext-dev",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BlackForestLabsFlux1Schnell {
            model_name: "black-forest-labs/flux_1-schnell",
            constructor_name: black_forest_labs_flux_1_schnell,
            display_name: "FLUX.1-schnell",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BlackForestLabsFlux2Klein4b {
            model_name: "black-forest-labs/flux_2-klein-4b",
            constructor_name: black_forest_labs_flux_2_klein_4b,
            display_name: "FLUX.2 Klein 4B",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        BytedanceSeedOss36bInstruct {
            model_name: "bytedance/seed-oss-36b-instruct",
            constructor_name: bytedance_seed_oss_36b_instruct,
            display_name: "ByteDance-Seed/Seed-OSS-36B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31Terminus {
            model_name: "deepseek-ai/deepseek-v3.1-terminus",
            constructor_name: deepseek_ai_deepseek_v3_1_terminus,
            display_name: "DeepSeek V3.1 Terminus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV32 {
            model_name: "deepseek-ai/deepseek-v3.2",
            constructor_name: deepseek_ai_deepseek_v3_2,
            display_name: "DeepSeek V3.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV4Flash {
            model_name: "deepseek-ai/deepseek-v4-flash",
            constructor_name: deepseek_ai_deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV4Pro {
            model_name: "deepseek-ai/deepseek-v4-pro",
            constructor_name: deepseek_ai_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma22bIt {
            model_name: "google/gemma-2-2b-it",
            constructor_name: google_gemma_2_2b_it,
            display_name: "Gemma 2 2b It",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma327bIt {
            model_name: "google/gemma-3-27b-it",
            constructor_name: google_gemma_3_27b_it,
            display_name: "Gemma-3-27B-IT",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma3nE2bIt {
            model_name: "google/gemma-3n-e2b-it",
            constructor_name: google_gemma_3n_e2b_it,
            display_name: "Gemma 3n E2b It",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma3nE4bIt {
            model_name: "google/gemma-3n-e4b-it",
            constructor_name: google_gemma_3n_e4b_it,
            display_name: "Gemma 3n E4b It",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31b-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma-4-31B-IT",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGooglePaligemma {
            model_name: "google/google-paligemma",
            constructor_name: google_google_paligemma,
            display_name: "paligemma",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaEsm2650m {
            model_name: "meta/esm2-650m",
            constructor_name: meta_esm2_650m,
            display_name: "esm2-650m",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaEsmfold {
            model_name: "meta/esmfold",
            constructor_name: meta_esmfold,
            display_name: "esmfold",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama3170bInstruct {
            model_name: "meta/llama-3.1-70b-instruct",
            constructor_name: meta_llama_3_1_70b_instruct,
            display_name: "Llama 3.1 70b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama318bInstruct {
            model_name: "meta/llama-3.1-8b-instruct",
            constructor_name: meta_llama_3_1_8b_instruct,
            display_name: "Llama 3.1 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3211bVisionInstruct {
            model_name: "meta/llama-3.2-11b-vision-instruct",
            constructor_name: meta_llama_3_2_11b_vision_instruct,
            display_name: "Llama 3.2 11b Vision Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama321bInstruct {
            model_name: "meta/llama-3.2-1b-instruct",
            constructor_name: meta_llama_3_2_1b_instruct,
            display_name: "Llama 3.2 1b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama323bInstruct {
            model_name: "meta/llama-3.2-3b-instruct",
            constructor_name: meta_llama_3_2_3b_instruct,
            display_name: "Llama 3.2 3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlama3290bVisionInstruct {
            model_name: "meta/llama-3.2-90b-vision-instruct",
            constructor_name: meta_llama_3_2_90b_vision_instruct,
            display_name: "Llama-3.2-90B-Vision-Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3370bInstruct {
            model_name: "meta/llama-3.3-70b-instruct",
            constructor_name: meta_llama_3_3_70b_instruct,
            display_name: "Llama 3.3 70b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama4Maverick17b128eInstruct {
            model_name: "meta/llama-4-maverick-17b-128e-instruct",
            constructor_name: meta_llama_4_maverick_17b_128e_instruct,
            display_name: "Llama 4 Maverick 17b 128e Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaGuard412b {
            model_name: "meta/llama-guard-4-12b",
            constructor_name: meta_llama_guard_4_12b,
            display_name: "Llama Guard 4 12B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MicrosoftPhi4MiniInstruct {
            model_name: "microsoft/phi-4-mini-instruct",
            constructor_name: microsoft_phi_4_mini_instruct,
            display_name: "Phi-4-Mini",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftPhi4MultimodalInstruct {
            model_name: "microsoft/phi-4-multimodal-instruct",
            constructor_name: microsoft_phi_4_multimodal_instruct,
            display_name: "Phi 4 Multimodal",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MinimaxaiMinimaxM25 {
            model_name: "minimaxai/minimax-m2.5",
            constructor_name: minimaxai_minimax_m2_5,
            display_name: "MiniMax-M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxaiMinimaxM27 {
            model_name: "minimaxai/minimax-m2.7",
            constructor_name: minimaxai_minimax_m2_7,
            display_name: "MiniMax-M2.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiDevstral2123bInstruct2512 {
            model_name: "mistralai/devstral-2-123b-instruct-2512",
            constructor_name: mistralai_devstral_2_123b_instruct_2512,
            display_name: "Devstral-2-123B-Instruct-2512",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMagistralSmall2506 {
            model_name: "mistralai/magistral-small-2506",
            constructor_name: mistralai_magistral_small_2506,
            display_name: "Magistral Small 2506",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistral7bInstructV03 {
            model_name: "mistralai/mistral-7b-instruct-v03",
            constructor_name: mistralai_mistral_7b_instruct_v03,
            display_name: "Mistral-7B-Instruct-v0.3",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralLarge3675bInstruct2512 {
            model_name: "mistralai/mistral-large-3-675b-instruct-2512",
            constructor_name: mistralai_mistral_large_3_675b_instruct_2512,
            display_name: "Mistral Large 3 675B Instruct 2512",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralMedium3Instruct {
            model_name: "mistralai/mistral-medium-3-instruct",
            constructor_name: mistralai_mistral_medium_3_instruct,
            display_name: "Mistral Medium 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralNemotron {
            model_name: "mistralai/mistral-nemotron",
            constructor_name: mistralai_mistral_nemotron,
            display_name: "mistral-nemotron",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall4119b2603 {
            model_name: "mistralai/mistral-small-4-119b-2603",
            constructor_name: mistralai_mistral_small_4_119b_2603,
            display_name: "mistral-small-4-119b-2603",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMixtral8x22bInstruct {
            model_name: "mistralai/mixtral-8x22b-instruct",
            constructor_name: mistralai_mixtral_8x22b_instruct,
            display_name: "Mistral: Mixtral 8x22B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMixtral8x7bInstruct {
            model_name: "mistralai/mixtral-8x7b-instruct",
            constructor_name: mistralai_mixtral_8x7b_instruct,
            display_name: "Mistral: Mixtral 8x7B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct {
            model_name: "moonshotai/kimi-k2-instruct",
            constructor_name: moonshotai_kimi_k2_instruct,
            display_name: "Kimi K2 Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct0905 {
            model_name: "moonshotai/kimi-k2-instruct-0905",
            constructor_name: moonshotai_kimi_k2_instruct_0905,
            display_name: "Kimi K2 0905",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/kimi-k2-thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK26 {
            model_name: "moonshotai/kimi-k2.6",
            constructor_name: moonshotai_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NvidiaActiveSpeakerDetection {
            model_name: "nvidia/active-speaker-detection",
            constructor_name: nvidia_active_speaker_detection,
            display_name: "Active Speaker Detection",
            capabilities: [ImageInputSupport, TextOutputSupport, VideoInputSupport]
        },
        NvidiaBevformer {
            model_name: "nvidia/bevformer",
            constructor_name: nvidia_bevformer,
            display_name: "bevformer",
            capabilities: [ImageInputSupport, TextOutputSupport, VideoInputSupport]
        },
        NvidiaCosmosPredict15b {
            model_name: "nvidia/cosmos-predict1-5b",
            constructor_name: nvidia_cosmos_predict1_5b,
            display_name: "cosmos-predict1-5b",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        NvidiaCosmosTransfer17b {
            model_name: "nvidia/cosmos-transfer1-7b",
            constructor_name: nvidia_cosmos_transfer1_7b,
            display_name: "cosmos-transfer1-7b",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        NvidiaCosmosTransfer252b {
            model_name: "nvidia/cosmos-transfer2_5-2b",
            constructor_name: nvidia_cosmos_transfer2_5_2b,
            display_name: "cosmos-transfer2.5-2b",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        NvidiaGlinerPii {
            model_name: "nvidia/gliner-pii",
            constructor_name: nvidia_gliner_pii,
            display_name: "gliner-pii",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlama31NemotronSafetyGuard8bV3 {
            model_name: "nvidia/llama-3_1-nemotron-safety-guard-8b-v3",
            constructor_name: nvidia_llama_3_1_nemotron_safety_guard_8b_v3,
            display_name: "llama-3.1-nemotron-safety-guard-8b-v3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlama32Nemoretriever300mEmbedV1 {
            model_name: "nvidia/llama-3_2-nemoretriever-300m-embed-v1",
            constructor_name: nvidia_llama_3_2_nemoretriever_300m_embed_v1,
            display_name: "llama-3_2-nemoretriever-300m-embed-v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlama33NemotronSuper49bV1 {
            model_name: "nvidia/llama-3_3-nemotron-super-49b-v1",
            constructor_name: nvidia_llama_3_3_nemotron_super_49b_v1,
            display_name: "Llama 3.3 Nemotron Super 49B v1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaLlama33NemotronSuper49bV15 {
            model_name: "nvidia/llama-3_3-nemotron-super-49b-v1_5",
            constructor_name: nvidia_llama_3_3_nemotron_super_49b_v1_5,
            display_name: "Llama 3.3 Nemotron Super 49B v1.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaLlamaNemotronEmbedVl1bV2 {
            model_name: "nvidia/llama-nemotron-embed-vl-1b-v2",
            constructor_name: nvidia_llama_nemotron_embed_vl_1b_v2,
            display_name: "llama-nemotron-embed-vl-1b-v2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaLlamaNemotronRerankVl1bV2 {
            model_name: "nvidia/llama-nemotron-rerank-vl-1b-v2",
            constructor_name: nvidia_llama_nemotron_rerank_vl_1b_v2,
            display_name: "llama-nemotron-rerank-vl-1b-v2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaMagpieTtsZeroshot {
            model_name: "nvidia/magpie-tts-zeroshot",
            constructor_name: nvidia_magpie_tts_zeroshot,
            display_name: "magpie-tts-zeroshot",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport]
        },
        NvidiaNemotron3ContentSafety {
            model_name: "nvidia/nemotron-3-content-safety",
            constructor_name: nvidia_nemotron_3_content_safety,
            display_name: "nemotron-3-content-safety",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3Nano30bA3b {
            model_name: "nvidia/nemotron-3-nano-30b-a3b",
            constructor_name: nvidia_nemotron_3_nano_30b_a3b,
            display_name: "nemotron-3-nano-30b-a3b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3NanoOmni30bA3bReasoning {
            model_name: "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning",
            constructor_name: nvidia_nemotron_3_nano_omni_30b_a3b_reasoning,
            display_name: "Nemotron 3 Nano Omni",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NvidiaNemotron3Super120bA12b {
            model_name: "nvidia/nemotron-3-super-120b-a12b",
            constructor_name: nvidia_nemotron_3_super_120b_a12b,
            display_name: "Nemotron 3 Super",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronContentSafetyReasoning4b {
            model_name: "nvidia/nemotron-content-safety-reasoning-4b",
            constructor_name: nvidia_nemotron_content_safety_reasoning_4b,
            display_name: "nemotron-content-safety-reasoning-4b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotronMini4bInstruct {
            model_name: "nvidia/nemotron-mini-4b-instruct",
            constructor_name: nvidia_nemotron_mini_4b_instruct,
            display_name: "nemotron-mini-4b-instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronVoicechat {
            model_name: "nvidia/nemotron-voicechat",
            constructor_name: nvidia_nemotron_voicechat,
            display_name: "nemotron-voicechat",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNvEmbedV1 {
            model_name: "nvidia/nv-embed-v1",
            constructor_name: nvidia_nv_embed_v1,
            display_name: "nv-embed-v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaNvEmbedcode7bV1 {
            model_name: "nvidia/nv-embedcode-7b-v1",
            constructor_name: nvidia_nv_embedcode_7b_v1,
            display_name: "nv-embedcode-7b-v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaNvidiaNemotronNano9bV2 {
            model_name: "nvidia/nvidia-nemotron-nano-9b-v2",
            constructor_name: nvidia_nvidia_nemotron_nano_9b_v2,
            display_name: "nvidia-nemotron-nano-9b-v2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaRerankQaMistral4b {
            model_name: "nvidia/rerank-qa-mistral-4b",
            constructor_name: nvidia_rerank_qa_mistral_4b,
            display_name: "rerank-qa-mistral-4b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaRivaTranslate4bInstructV11 {
            model_name: "nvidia/riva-translate-4b-instruct-v1_1",
            constructor_name: nvidia_riva_translate_4b_instruct_v1_1,
            display_name: "riva-translate-4b-instruct-v1_1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaSparsedrive {
            model_name: "nvidia/sparsedrive",
            constructor_name: nvidia_sparsedrive,
            display_name: "sparsedrive",
            capabilities: [ImageInputSupport, TextOutputSupport, VideoInputSupport]
        },
        NvidiaStreampetr {
            model_name: "nvidia/streampetr",
            constructor_name: nvidia_streampetr,
            display_name: "streampetr",
            capabilities: [ImageInputSupport, TextOutputSupport, VideoInputSupport]
        },
        NvidiaStudiovoice {
            model_name: "nvidia/studiovoice",
            constructor_name: nvidia_studiovoice,
            display_name: "studiovoice",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaSyntheticVideoDetector {
            model_name: "nvidia/synthetic-video-detector",
            constructor_name: nvidia_synthetic_video_detector,
            display_name: "synthetic-video-detector",
            capabilities: [ImageInputSupport, TextOutputSupport, VideoInputSupport]
        },
        NvidiaUsdcode {
            model_name: "nvidia/usdcode",
            constructor_name: nvidia_usdcode,
            display_name: "usdcode",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaUsdvalidate {
            model_name: "nvidia/usdvalidate",
            constructor_name: nvidia_usdvalidate,
            display_name: "usdvalidate",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiGptOss120b {
            model_name: "openai/gpt-oss-120b",
            constructor_name: openai_gpt_oss_120b,
            display_name: "GPT-OSS-120B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGptOss20b {
            model_name: "openai/gpt-oss-20b",
            constructor_name: openai_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiWhisperLargeV3 {
            model_name: "openai/whisper-large-v3",
            constructor_name: openai_whisper_large_v3,
            display_name: "Whisper Large v3",
            capabilities: [AudioInputSupport, TextOutputSupport]
        },
        QwenQwenImage {
            model_name: "qwen/qwen-image",
            constructor_name: qwen_qwen_image,
            display_name: "Qwen Image",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        QwenQwenImageEdit {
            model_name: "qwen/qwen-image-edit",
            constructor_name: qwen_qwen_image_edit,
            display_name: "Qwen Image Edit",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        QwenQwen25Coder32bInstruct {
            model_name: "qwen/qwen2.5-coder-32b-instruct",
            constructor_name: qwen_qwen2_5_coder_32b_instruct,
            display_name: "Qwen2.5 Coder 32b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder480bA35bInstruct {
            model_name: "qwen/qwen3-coder-480b-a35b-instruct",
            constructor_name: qwen_qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bInstruct {
            model_name: "qwen/qwen3-next-80b-a3b-instruct",
            constructor_name: qwen_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3-Next-80B-A3B-Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bThinking {
            model_name: "qwen/qwen3-next-80b-a3b-thinking",
            constructor_name: qwen_qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3-Next-80B-A3B-Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen35122bA10b {
            model_name: "qwen/qwen3.5-122b-a10b",
            constructor_name: qwen_qwen3_5_122b_a10b,
            display_name: "Qwen3.5 122B-A10B",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen35397bA17b {
            model_name: "qwen/qwen3.5-397b-a17b",
            constructor_name: qwen_qwen3_5_397b_a17b,
            display_name: "Qwen3.5-397B-A17B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SarvamaiSarvamM {
            model_name: "sarvamai/sarvam-m",
            constructor_name: sarvamai_sarvam_m,
            display_name: "sarvam-m",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        StepfunAiStep35Flash {
            model_name: "stepfun-ai/step-3.5-flash",
            constructor_name: stepfun_ai_step_3_5_flash,
            display_name: "Step 3.5 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UpstageSolar107bInstruct {
            model_name: "upstage/solar-10_7b-instruct",
            constructor_name: upstage_solar_10_7b_instruct,
            display_name: "solar-10.7b-instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm51 {
            model_name: "z-ai/glm-5.1",
            constructor_name: z_ai_glm_5_1,
            display_name: "GLM-5.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm47 {
            model_name: "z-ai/glm4.7",
            constructor_name: z_ai_glm4_7,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
