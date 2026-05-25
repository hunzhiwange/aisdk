//! Capabilities for chutes models.
//!
//! This module defines model types and their capabilities for chutes providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::chutes::Chutes;

model_capabilities! {
    provider: Chutes,
    models: {
        MinimaxaiMinimaxM25Tee {
            model_name: "MiniMaxAI/MiniMax-M2.5-TEE",
            constructor_name: minimaxai_minimax_m2_5_tee,
            display_name: "MiniMax M2.5 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchDeephermes3Mistral24bPreview {
            model_name: "NousResearch/DeepHermes-3-Mistral-24B-Preview",
            constructor_name: nousresearch_deephermes_3_mistral_24b_preview,
            display_name: "DeepHermes 3 Mistral 24B Preview",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NousresearchHermes414b {
            model_name: "NousResearch/Hermes-4-14B",
            constructor_name: nousresearch_hermes_4_14b,
            display_name: "Hermes 4 14B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen2572bInstruct {
            model_name: "Qwen/Qwen2.5-72B-Instruct",
            constructor_name: qwen_qwen2_5_72b_instruct,
            display_name: "Qwen2.5 72B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen25Coder32bInstruct {
            model_name: "Qwen/Qwen2.5-Coder-32B-Instruct",
            constructor_name: qwen_qwen2_5_coder_32b_instruct,
            display_name: "Qwen2.5 Coder 32B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen25Vl32bInstruct {
            model_name: "Qwen/Qwen2.5-VL-32B-Instruct",
            constructor_name: qwen_qwen2_5_vl_32b_instruct,
            display_name: "Qwen2.5 VL 32B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen3235bA22bInstruct2507Tee {
            model_name: "Qwen/Qwen3-235B-A22B-Instruct-2507-TEE",
            constructor_name: qwen_qwen3_235b_a22b_instruct_2507_tee,
            display_name: "Qwen3 235B A22B Instruct 2507 TEE",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bThinking2507 {
            model_name: "Qwen/Qwen3-235B-A22B-Thinking-2507",
            constructor_name: qwen_qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen3 235B A22B Thinking 2507",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3b {
            model_name: "Qwen/Qwen3-30B-A3B",
            constructor_name: qwen_qwen3_30b_a3b,
            display_name: "Qwen3 30B A3B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen332bTee {
            model_name: "Qwen/Qwen3-32B-TEE",
            constructor_name: qwen_qwen3_32b_tee,
            display_name: "Qwen3 32B TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderNextTee {
            model_name: "Qwen/Qwen3-Coder-Next-TEE",
            constructor_name: qwen_qwen3_coder_next_tee,
            display_name: "Qwen3 Coder Next TEE",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3bInstruct {
            model_name: "Qwen/Qwen3-Next-80B-A3B-Instruct",
            constructor_name: qwen_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3 Next 80B A3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen35397bA17bTee {
            model_name: "Qwen/Qwen3.5-397B-A17B-TEE",
            constructor_name: qwen_qwen3_5_397b_a17b_tee,
            display_name: "Qwen3.5 397B A17B TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3627bTee {
            model_name: "Qwen/Qwen3.6-27B-TEE",
            constructor_name: qwen_qwen3_6_27b_tee,
            display_name: "Qwen3.6 27B TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3guardGen06b {
            model_name: "Qwen/Qwen3Guard-Gen-0.6B",
            constructor_name: qwen_qwen3guard_gen_0_6b,
            display_name: "Qwen3Guard Gen 0.6B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        XiaomimimoMimoV2FlashTee {
            model_name: "XiaomiMiMo/MiMo-V2-Flash-TEE",
            constructor_name: xiaomimimo_mimo_v2_flash_tee,
            display_name: "MiMo V2 Flash TEE",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekR10528Tee {
            model_name: "deepseek-ai/DeepSeek-R1-0528-TEE",
            constructor_name: deepseek_ai_deepseek_r1_0528_tee,
            display_name: "DeepSeek R1 0528 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekR1DistillLlama70b {
            model_name: "deepseek-ai/DeepSeek-R1-Distill-Llama-70B",
            constructor_name: deepseek_ai_deepseek_r1_distill_llama_70b,
            display_name: "DeepSeek R1 Distill Llama 70B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV30324Tee {
            model_name: "deepseek-ai/DeepSeek-V3-0324-TEE",
            constructor_name: deepseek_ai_deepseek_v3_0324_tee,
            display_name: "DeepSeek V3 0324 TEE",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31Tee {
            model_name: "deepseek-ai/DeepSeek-V3.1-TEE",
            constructor_name: deepseek_ai_deepseek_v3_1_tee,
            display_name: "DeepSeek V3.1 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV32Tee {
            model_name: "deepseek-ai/DeepSeek-V3.2-TEE",
            constructor_name: deepseek_ai_deepseek_v3_2_tee,
            display_name: "DeepSeek V3.2 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma431bTurboTee {
            model_name: "google/gemma-4-31B-turbo-TEE",
            constructor_name: google_gemma_4_31b_turbo_tee,
            display_name: "gemma 4 31B turbo TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25Tee {
            model_name: "moonshotai/Kimi-K2.5-TEE",
            constructor_name: moonshotai_kimi_k2_5_tee,
            display_name: "Kimi K2.5 TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiK26Tee {
            model_name: "moonshotai/Kimi-K2.6-TEE",
            constructor_name: moonshotai_kimi_k2_6_tee,
            display_name: "Kimi K2.6 TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        OpenaiGptOss120bTee {
            model_name: "openai/gpt-oss-120b-TEE",
            constructor_name: openai_gpt_oss_120b_tee,
            display_name: "gpt oss 120b TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        RednoteHilabDotsOcr {
            model_name: "rednote-hilab/dots.ocr",
            constructor_name: rednote_hilab_dots_ocr,
            display_name: "dots.ocr",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        TngtechDeepseekTngR1t2ChimeraTee {
            model_name: "tngtech/DeepSeek-TNG-R1T2-Chimera-TEE",
            constructor_name: tngtech_deepseek_tng_r1t2_chimera_tee,
            display_name: "DeepSeek TNG R1T2 Chimera TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UnslothLlama321bInstruct {
            model_name: "unsloth/Llama-3.2-1B-Instruct",
            constructor_name: unsloth_llama_3_2_1b_instruct,
            display_name: "Llama 3.2 1B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        UnslothLlama323bInstruct {
            model_name: "unsloth/Llama-3.2-3B-Instruct",
            constructor_name: unsloth_llama_3_2_3b_instruct,
            display_name: "Llama 3.2 3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        UnslothMistralNemoInstruct2407 {
            model_name: "unsloth/Mistral-Nemo-Instruct-2407",
            constructor_name: unsloth_mistral_nemo_instruct_2407,
            display_name: "Mistral Nemo Instruct 2407",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        UnslothGemma312bIt {
            model_name: "unsloth/gemma-3-12b-it",
            constructor_name: unsloth_gemma_3_12b_it,
            display_name: "gemma 3 12b it",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        UnslothGemma327bIt {
            model_name: "unsloth/gemma-3-27b-it",
            constructor_name: unsloth_gemma_3_27b_it,
            display_name: "gemma 3 27b it",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UnslothGemma34bIt {
            model_name: "unsloth/gemma-3-4b-it",
            constructor_name: unsloth_gemma_3_4b_it,
            display_name: "gemma 3 4b it",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm46v {
            model_name: "zai-org/GLM-4.6V",
            constructor_name: zai_org_glm_4_6v,
            display_name: "GLM 4.6V",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Fp8 {
            model_name: "zai-org/GLM-4.7-FP8",
            constructor_name: zai_org_glm_4_7_fp8,
            display_name: "GLM 4.7 FP8",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Tee {
            model_name: "zai-org/GLM-4.7-TEE",
            constructor_name: zai_org_glm_4_7_tee,
            display_name: "GLM 4.7 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5Tee {
            model_name: "zai-org/GLM-5-TEE",
            constructor_name: zai_org_glm_5_tee,
            display_name: "GLM 5 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5Turbo {
            model_name: "zai-org/GLM-5-Turbo",
            constructor_name: zai_org_glm_5_turbo,
            display_name: "GLM 5 Turbo",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm51Tee {
            model_name: "zai-org/GLM-5.1-TEE",
            constructor_name: zai_org_glm_5_1_tee,
            display_name: "GLM 5.1 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
