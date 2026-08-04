//! Capabilities for qiniu_ai models.
//!
//! This module defines model types and their capabilities for qiniu_ai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::qiniu_ai::QiniuAi;

model_capabilities! {
    provider: QiniuAi,
    models: {
        MinimaxM1 {
            model_name: "MiniMax-M1",
            constructor_name: minimax_m1,
            display_name: "MiniMax M1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude35Haiku {
            model_name: "claude-3.5-haiku",
            constructor_name: claude_3_5_haiku,
            display_name: "Claude 3.5 Haiku",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude35Sonnet {
            model_name: "claude-3.5-sonnet",
            constructor_name: claude_3_5_sonnet,
            display_name: "Claude 3.5 Sonnet",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude37Sonnet {
            model_name: "claude-3.7-sonnet",
            constructor_name: claude_3_7_sonnet,
            display_name: "Claude 3.7 Sonnet",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude40Opus {
            model_name: "claude-4.0-opus",
            constructor_name: claude_4_0_opus,
            display_name: "Claude 4.0 Opus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude40Sonnet {
            model_name: "claude-4.0-sonnet",
            constructor_name: claude_4_0_sonnet,
            display_name: "Claude 4.0 Sonnet",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude41Opus {
            model_name: "claude-4.1-opus",
            constructor_name: claude_4_1_opus,
            display_name: "Claude 4.1 Opus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude45Haiku {
            model_name: "claude-4.5-haiku",
            constructor_name: claude_4_5_haiku,
            display_name: "Claude 4.5 Haiku",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude45Opus {
            model_name: "claude-4.5-opus",
            constructor_name: claude_4_5_opus,
            display_name: "Claude 4.5 Opus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude45Sonnet {
            model_name: "claude-4.5-sonnet",
            constructor_name: claude_4_5_sonnet,
            display_name: "Claude 4.5 Sonnet",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR1 {
            model_name: "deepseek-r1",
            constructor_name: deepseek_r1,
            display_name: "DeepSeek-R1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR10528 {
            model_name: "deepseek-r1-0528",
            constructor_name: deepseek_r1_0528,
            display_name: "DeepSeek-R1-0528",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV3 {
            model_name: "deepseek-v3",
            constructor_name: deepseek_v3,
            display_name: "DeepSeek-V3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekV30324 {
            model_name: "deepseek-v3-0324",
            constructor_name: deepseek_v3_0324,
            display_name: "DeepSeek-V3-0324",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV31 {
            model_name: "deepseek-v3.1",
            constructor_name: deepseek_v3_1,
            display_name: "DeepSeek-V3.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekMathV2 {
            model_name: "deepseek/deepseek-math-v2",
            constructor_name: deepseek_deepseek_math_v2,
            display_name: "Deepseek/Deepseek-Math-V2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekV31Terminus {
            model_name: "deepseek/deepseek-v3.1-terminus",
            constructor_name: deepseek_deepseek_v3_1_terminus,
            display_name: "DeepSeek/DeepSeek-V3.1-Terminus",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV31TerminusThinking {
            model_name: "deepseek/deepseek-v3.1-terminus-thinking",
            constructor_name: deepseek_deepseek_v3_1_terminus_thinking,
            display_name: "DeepSeek/DeepSeek-V3.1-Terminus-Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekV32251201 {
            model_name: "deepseek/deepseek-v3.2-251201",
            constructor_name: deepseek_deepseek_v3_2_251201,
            display_name: "Deepseek/DeepSeek-V3.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32Exp {
            model_name: "deepseek/deepseek-v3.2-exp",
            constructor_name: deepseek_deepseek_v3_2_exp,
            display_name: "DeepSeek/DeepSeek-V3.2-Exp",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32ExpThinking {
            model_name: "deepseek/deepseek-v3.2-exp-thinking",
            constructor_name: deepseek_deepseek_v3_2_exp_thinking,
            display_name: "DeepSeek/DeepSeek-V3.2-Exp-Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Doubao15Pro32k {
            model_name: "doubao-1.5-pro-32k",
            constructor_name: doubao_1_5_pro_32k,
            display_name: "Doubao 1.5 Pro 32k",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Doubao15ThinkingPro {
            model_name: "doubao-1.5-thinking-pro",
            constructor_name: doubao_1_5_thinking_pro,
            display_name: "Doubao 1.5 Thinking Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Doubao15VisionPro {
            model_name: "doubao-1.5-vision-pro",
            constructor_name: doubao_1_5_vision_pro,
            display_name: "Doubao 1.5 Vision Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        DoubaoSeed16 {
            model_name: "doubao-seed-1.6",
            constructor_name: doubao_seed_1_6,
            display_name: "Doubao-Seed 1.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DoubaoSeed16Flash {
            model_name: "doubao-seed-1.6-flash",
            constructor_name: doubao_seed_1_6_flash,
            display_name: "Doubao-Seed 1.6 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DoubaoSeed16Thinking {
            model_name: "doubao-seed-1.6-thinking",
            constructor_name: doubao_seed_1_6_thinking,
            display_name: "Doubao-Seed 1.6 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DoubaoSeed20Code {
            model_name: "doubao-seed-2.0-code",
            constructor_name: doubao_seed_2_0_code,
            display_name: "Doubao Seed 2.0 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DoubaoSeed20Lite {
            model_name: "doubao-seed-2.0-lite",
            constructor_name: doubao_seed_2_0_lite,
            display_name: "Doubao Seed 2.0 Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DoubaoSeed20Mini {
            model_name: "doubao-seed-2.0-mini",
            constructor_name: doubao_seed_2_0_mini,
            display_name: "Doubao Seed 2.0 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        DoubaoSeed20Pro {
            model_name: "doubao-seed-2.0-pro",
            constructor_name: doubao_seed_2_0_pro,
            display_name: "Doubao Seed 2.0 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini20Flash {
            model_name: "gemini-2.0-flash",
            constructor_name: gemini_2_0_flash,
            display_name: "Gemini 2.0 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini20FlashLite {
            model_name: "gemini-2.0-flash-lite",
            constructor_name: gemini_2_0_flash_lite,
            display_name: "Gemini 2.0 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25Flash {
            model_name: "gemini-2.5-flash",
            constructor_name: gemini_2_5_flash,
            display_name: "Gemini 2.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25FlashImage {
            model_name: "gemini-2.5-flash-image",
            constructor_name: gemini_2_5_flash_image,
            display_name: "Gemini 2.5 Flash Image",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        Gemini25FlashLite {
            model_name: "gemini-2.5-flash-lite",
            constructor_name: gemini_2_5_flash_lite,
            display_name: "Gemini 2.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini25Pro {
            model_name: "gemini-2.5-pro",
            constructor_name: gemini_2_5_pro,
            display_name: "Gemini 2.5 Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini30FlashPreview {
            model_name: "gemini-3.0-flash-preview",
            constructor_name: gemini_3_0_flash_preview,
            display_name: "Gemini 3.0 Flash Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemini30ProImagePreview {
            model_name: "gemini-3.0-pro-image-preview",
            constructor_name: gemini_3_0_pro_image_preview,
            display_name: "Gemini 3.0 Pro Image Preview",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini30ProPreview {
            model_name: "gemini-3.0-pro-preview",
            constructor_name: gemini_3_0_pro_preview,
            display_name: "Gemini 3.0 Pro Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm45 {
            model_name: "glm-4.5",
            constructor_name: glm_4_5,
            display_name: "GLM 4.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm45Air {
            model_name: "glm-4.5-air",
            constructor_name: glm_4_5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "gpt-oss-120b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss20b {
            model_name: "gpt-oss-20b",
            constructor_name: gpt_oss_20b,
            display_name: "gpt-oss-20b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK2 {
            model_name: "kimi-k2",
            constructor_name: kimi_k2,
            display_name: "Kimi K2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KlingV26 {
            model_name: "kling-v2-6",
            constructor_name: kling_v2_6,
            display_name: "Kling-V2 6",
            capabilities: [ImageInputSupport, TextInputSupport, VideoInputSupport, VideoOutputSupport]
        },
        MeituanLongcatFlashChat {
            model_name: "meituan/longcat-flash-chat",
            constructor_name: meituan_longcat_flash_chat,
            display_name: "Meituan/Longcat-Flash-Chat",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MeituanLongcatFlashLite {
            model_name: "meituan/longcat-flash-lite",
            constructor_name: meituan_longcat_flash_lite,
            display_name: "Meituan/Longcat-Flash-Lite",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MimoV2Flash {
            model_name: "mimo-v2-flash",
            constructor_name: mimo_v2_flash,
            display_name: "Mimo-V2-Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM2 {
            model_name: "minimax/minimax-m2",
            constructor_name: minimax_minimax_m2,
            display_name: "Minimax/Minimax-M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM21 {
            model_name: "minimax/minimax-m2.1",
            constructor_name: minimax_minimax_m2_1,
            display_name: "Minimax/Minimax-M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM25 {
            model_name: "minimax/minimax-m2.5",
            constructor_name: minimax_minimax_m2_5,
            display_name: "Minimax/Minimax-M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM25Highspeed {
            model_name: "minimax/minimax-m2.5-highspeed",
            constructor_name: minimax_minimax_m2_5_highspeed,
            display_name: "Minimax/Minimax-M2.5 Highspeed",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK20905 {
            model_name: "moonshotai/kimi-k2-0905",
            constructor_name: moonshotai_kimi_k2_0905,
            display_name: "Kimi K2 0905",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/kimi-k2-thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai/kimi-k2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Moonshotai/Kimi-K2.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        OpenaiGpt5 {
            model_name: "openai/gpt-5",
            constructor_name: openai_gpt_5,
            display_name: "OpenAI/GPT-5",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52 {
            model_name: "openai/gpt-5.2",
            constructor_name: openai_gpt_5_2,
            display_name: "OpenAI/GPT-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenMax20250125 {
            model_name: "qwen-max-2025-01-25",
            constructor_name: qwen_max_2025_01_25,
            display_name: "Qwen2.5-Max-2025-01-25",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenTurbo {
            model_name: "qwen-turbo",
            constructor_name: qwen_turbo,
            display_name: "Qwen-Turbo",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenVlMax20250125 {
            model_name: "qwen-vl-max-2025-01-25",
            constructor_name: qwen_vl_max_2025_01_25,
            display_name: "Qwen VL-MAX-2025-01-25",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen25Vl72bInstruct {
            model_name: "qwen2.5-vl-72b-instruct",
            constructor_name: qwen2_5_vl_72b_instruct,
            display_name: "Qwen 2.5 VL 72B Instruct",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen25Vl7bInstruct {
            model_name: "qwen2.5-vl-7b-instruct",
            constructor_name: qwen2_5_vl_7b_instruct,
            display_name: "Qwen 2.5 VL 7B Instruct",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3235bA22b {
            model_name: "qwen3-235b-a22b",
            constructor_name: qwen3_235b_a22b,
            display_name: "Qwen 3 235B A22B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3235bA22bInstruct2507 {
            model_name: "qwen3-235b-a22b-instruct-2507",
            constructor_name: qwen3_235b_a22b_instruct_2507,
            display_name: "Qwen3 235b A22B Instruct 2507",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3235bA22bThinking2507 {
            model_name: "qwen3-235b-a22b-thinking-2507",
            constructor_name: qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen3 235B A22B Thinking 2507",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen330bA3b {
            model_name: "qwen3-30b-a3b",
            constructor_name: qwen3_30b_a3b,
            display_name: "Qwen3 30B A3B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen330bA3bInstruct2507 {
            model_name: "qwen3-30b-a3b-instruct-2507",
            constructor_name: qwen3_30b_a3b_instruct_2507,
            display_name: "Qwen3 30b A3b Instruct 2507",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen330bA3bThinking2507 {
            model_name: "qwen3-30b-a3b-thinking-2507",
            constructor_name: qwen3_30b_a3b_thinking_2507,
            display_name: "Qwen3 30b A3b Thinking 2507",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen332b {
            model_name: "qwen3-32b",
            constructor_name: qwen3_32b,
            display_name: "Qwen3 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Coder480bA35bInstruct {
            model_name: "qwen3-coder-480b-a35b-instruct",
            constructor_name: qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Max {
            model_name: "qwen3-max",
            constructor_name: qwen3_max,
            display_name: "Qwen3 Max",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3MaxPreview {
            model_name: "qwen3-max-preview",
            constructor_name: qwen3_max_preview,
            display_name: "Qwen3 Max Preview",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Next80bA3bInstruct {
            model_name: "qwen3-next-80b-a3b-instruct",
            constructor_name: qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3 Next 80B A3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Next80bA3bThinking {
            model_name: "qwen3-next-80b-a3b-thinking",
            constructor_name: qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3 Next 80B A3B Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Vl30bA3bThinking {
            model_name: "qwen3-vl-30b-a3b-thinking",
            constructor_name: qwen3_vl_30b_a3b_thinking,
            display_name: "Qwen3-Vl 30b A3b Thinking",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen35397bA17b {
            model_name: "qwen3.5-397b-a17b",
            constructor_name: qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B A17B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        StepfunAiGelabZero4bPreview {
            model_name: "stepfun-ai/gelab-zero-4b-preview",
            constructor_name: stepfun_ai_gelab_zero_4b_preview,
            display_name: "Stepfun-Ai/Gelab Zero 4b Preview",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        StepfunStep35Flash {
            model_name: "stepfun/step-3.5-flash",
            constructor_name: stepfun_step_3_5_flash,
            display_name: "Stepfun/Step-3.5 Flash",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        XAiGrok4Fast {
            model_name: "x-ai/grok-4-fast",
            constructor_name: x_ai_grok_4_fast,
            display_name: "x-AI/Grok-4-Fast",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XAiGrok4FastNonReasoning {
            model_name: "x-ai/grok-4-fast-non-reasoning",
            constructor_name: x_ai_grok_4_fast_non_reasoning,
            display_name: "X-Ai/Grok-4-Fast-Non-Reasoning",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XAiGrok4FastReasoning {
            model_name: "x-ai/grok-4-fast-reasoning",
            constructor_name: x_ai_grok_4_fast_reasoning,
            display_name: "X-Ai/Grok-4-Fast-Reasoning",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XAiGrok41Fast {
            model_name: "x-ai/grok-4.1-fast",
            constructor_name: x_ai_grok_4_1_fast,
            display_name: "x-AI/Grok-4.1-Fast",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok41FastNonReasoning {
            model_name: "x-ai/grok-4.1-fast-non-reasoning",
            constructor_name: x_ai_grok_4_1_fast_non_reasoning,
            display_name: "X-Ai/Grok 4.1 Fast Non Reasoning",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XAiGrok41FastReasoning {
            model_name: "x-ai/grok-4.1-fast-reasoning",
            constructor_name: x_ai_grok_4_1_fast_reasoning,
            display_name: "X-Ai/Grok 4.1 Fast Reasoning",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XAiGrokCodeFast1 {
            model_name: "x-ai/grok-code-fast-1",
            constructor_name: x_ai_grok_code_fast_1,
            display_name: "x-AI/Grok-Code-Fast 1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV2Flash {
            model_name: "xiaomi/mimo-v2-flash",
            constructor_name: xiaomi_mimo_v2_flash,
            display_name: "Xiaomi/Mimo-V2-Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiAutoglmPhone9b {
            model_name: "z-ai/autoglm-phone-9b",
            constructor_name: z_ai_autoglm_phone_9b,
            display_name: "Z-Ai/Autoglm Phone 9b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm46 {
            model_name: "z-ai/glm-4.6",
            constructor_name: z_ai_glm_4_6,
            display_name: "Z-AI/GLM 4.6",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm47 {
            model_name: "z-ai/glm-4.7",
            constructor_name: z_ai_glm_4_7,
            display_name: "Z-Ai/GLM 4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5 {
            model_name: "z-ai/glm-5",
            constructor_name: z_ai_glm_5,
            display_name: "Z-Ai/GLM 5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
