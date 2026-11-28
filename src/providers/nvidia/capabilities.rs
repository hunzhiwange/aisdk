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
        GoogleGooglePaligemma {
            model_name: "google/google-paligemma",
            constructor_name: google_google_paligemma,
            display_name: "paligemma",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MinimaxaiMinimaxM31Flash {
            model_name: "minimaxai/minimax-m3.1-flash",
            constructor_name: minimaxai_minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
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
        NvidiaCosmosTransfer252b {
            model_name: "nvidia/cosmos-transfer2_5-2b",
            constructor_name: nvidia_cosmos_transfer2_5_2b,
            display_name: "cosmos-transfer2.5-2b",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        NvidiaLlama31NemotronSafetyGuard8bV3 {
            model_name: "nvidia/llama-3.1-nemotron-safety-guard-8b-v3",
            constructor_name: nvidia_llama_3_1_nemotron_safety_guard_8b_v3,
            display_name: "llama-3.1-nemotron-safety-guard-8b-v3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaMagpieTtsZeroshot {
            model_name: "nvidia/magpie-tts-zeroshot",
            constructor_name: nvidia_magpie_tts_zeroshot,
            display_name: "magpie-tts-zeroshot",
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport, TextInputSupport]
        },
        NvidiaNemotron3NanoOmni30bA3bReasoning {
            model_name: "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning",
            constructor_name: nvidia_nemotron_3_nano_omni_30b_a3b_reasoning,
            display_name: "Nemotron 3 Nano Omni",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NvidiaNemotron3Ultra550bA55b {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b,
            display_name: "Nemotron 3 Ultra 550B A55B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35ContentSafety {
            model_name: "nvidia/nemotron-3.5-content-safety",
            constructor_name: nvidia_nemotron_3_5_content_safety,
            display_name: "Nemotron 3.5 Content Safety",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron35Lightning30bA3b {
            model_name: "nvidia/nemotron-3.5-lightning-30b-a3b",
            constructor_name: nvidia_nemotron_3_5_lightning_30b_a3b,
            display_name: "Nemotron 3.5 Lightning 30B A3B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron35SuperVl120bA12b {
            model_name: "nvidia/nemotron-3.5-super-vl-120b-a12b",
            constructor_name: nvidia_nemotron_3_5_super_vl_120b_a12b,
            display_name: "Nemotron 3.5 Super VL 120B A12B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
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
            capabilities: [AudioInputSupport, AudioOutputSupport, ImageInputSupport]
        },
        NvidiaSyntheticVideoDetector {
            model_name: "nvidia/synthetic-video-detector",
            constructor_name: nvidia_synthetic_video_detector,
            display_name: "synthetic-video-detector",
            capabilities: [ImageInputSupport, TextOutputSupport, VideoInputSupport]
        },
        OpenaiWhisperLargeV3 {
            model_name: "openai/whisper-large-v3",
            constructor_name: openai_whisper_large_v3,
            display_name: "Whisper Large v3",
            capabilities: [AudioInputSupport, ImageInputSupport, TextOutputSupport]
        },
        PoolsideLagunaXs21 {
            model_name: "poolside/laguna-xs-2.1",
            constructor_name: poolside_laguna_xs_2_1,
            display_name: "Laguna XS 2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwenImage {
            model_name: "qwen/qwen-image",
            constructor_name: qwen_qwen_image,
            display_name: "Qwen Image",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        QwenQwenImageEdit {
            model_name: "qwen/qwen-image-edit",
            constructor_name: qwen_qwen_image_edit,
            display_name: "Qwen Image Edit",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        StepfunAiStep5Preview {
            model_name: "stepfun-ai/step-5-preview",
            constructor_name: stepfun_ai_step_5_preview,
            display_name: "Step 5 Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53Flash {
            model_name: "z-ai/glm-5.3-flash",
            constructor_name: z_ai_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
