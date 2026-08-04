//! Capabilities for nano_gpt models.
//!
//! This module defines model types and their capabilities for nano_gpt providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::nano_gpt::NanoGpt;

model_capabilities! {
    provider: NanoGpt,
    models: {
        BaichuanM2 {
            model_name: "Baichuan-M2",
            constructor_name: baichuan_m2,
            display_name: "Baichuan M2 32B Medical",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Baichuan4Air {
            model_name: "Baichuan4-Air",
            constructor_name: baichuan4_air,
            display_name: "Baichuan 4 Air",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Baichuan4Turbo {
            model_name: "Baichuan4-Turbo",
            constructor_name: baichuan4_turbo,
            display_name: "Baichuan 4 Turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DoctorShotgunMs3224bMagnumDiamond {
            model_name: "Doctor-Shotgun/MS3.2-24B-Magnum-Diamond",
            constructor_name: doctor_shotgun_ms3_2_24b_magnum_diamond,
            display_name: "MS3.2 24B Magnum Diamond",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        EvaUnit01EvaLlama33370bV00 {
            model_name: "EVA-UNIT-01/EVA-LLaMA-3.33-70B-v0.0",
            constructor_name: eva_unit_01_eva_llama_3_33_70b_v0_0,
            display_name: "EVA Llama 3.33 70B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        EvaUnit01EvaLlama33370bV01 {
            model_name: "EVA-UNIT-01/EVA-LLaMA-3.33-70B-v0.1",
            constructor_name: eva_unit_01_eva_llama_3_33_70b_v0_1,
            display_name: "EVA-LLaMA-3.33-70B-v0.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        EvaUnit01EvaQwen2532bV02 {
            model_name: "EVA-UNIT-01/EVA-Qwen2.5-32B-v0.2",
            constructor_name: eva_unit_01_eva_qwen2_5_32b_v0_2,
            display_name: "EVA-Qwen2.5-32B-v0.2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        EvaUnit01EvaQwen2572bV02 {
            model_name: "EVA-UNIT-01/EVA-Qwen2.5-72B-v0.2",
            constructor_name: eva_unit_01_eva_qwen2_5_72b_v0_2,
            display_name: "EVA-Qwen2.5-72B-v0.2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        EnvoidLlama305NtStorybreakerMinistral70b {
            model_name: "Envoid/Llama-3.05-NT-Storybreaker-Ministral-70B",
            constructor_name: envoid_llama_3_05_nt_storybreaker_ministral_70b,
            display_name: "Llama 3.05 Storybreaker Ministral 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        EnvoidLlama305NemotronTenyxchatStorybreaker70b {
            model_name: "Envoid/Llama-3.05-Nemotron-Tenyxchat-Storybreaker-70B",
            constructor_name: envoid_llama_3_05_nemotron_tenyxchat_storybreaker_70b,
            display_name: "Nemotron Tenyxchat Storybreaker 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm46DerestrictedV5 {
            model_name: "GLM-4.6-Derestricted-v5",
            constructor_name: glm_4_6_derestricted_v5,
            display_name: "GLM 4.6 Derestricted v5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GalrionsoftworksMnLoosecannon12bV1 {
            model_name: "GalrionSoftworks/MN-LooseCannon-12B-v1",
            constructor_name: galrionsoftworks_mn_loosecannon_12b_v1,
            display_name: "MN-LooseCannon-12B-v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemma431bClaude46OpusReasoningDistilled {
            model_name: "Gemma-4-31B-Claude-4.6-Opus-Reasoning-Distilled",
            constructor_name: gemma_4_31b_claude_4_6_opus_reasoning_distilled,
            display_name: "Gemma 4 31B Claude 4.6 Opus Reasoning Distilled",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemma431bCognitiveUnshackled {
            model_name: "Gemma-4-31B-Cognitive-Unshackled",
            constructor_name: gemma_4_31b_cognitive_unshackled,
            display_name: "Gemma 4 31B Cognitive Unshackled",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemma431bDarkidol {
            model_name: "Gemma-4-31B-DarkIdol",
            constructor_name: gemma_4_31b_darkidol,
            display_name: "Gemma 4 31B DarkIdol",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemma431bGarnetv2 {
            model_name: "Gemma-4-31B-GarnetV2",
            constructor_name: gemma_4_31b_garnetv2,
            display_name: "Gemma 4 31B Garnet V2",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemma431bGemopus {
            model_name: "Gemma-4-31B-Gemopus",
            constructor_name: gemma_4_31b_gemopus,
            display_name: "Gemma 4 31B Gemopus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemma431bMusicaV1 {
            model_name: "Gemma-4-31B-Musica-v1",
            constructor_name: gemma_4_31b_musica_v1,
            display_name: "Gemma 4 31B Musica v1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemma431bQueen {
            model_name: "Gemma-4-31B-Queen",
            constructor_name: gemma_4_31b_queen,
            display_name: "Gemma 4 31B Queen",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GrypheMythomaxL213b {
            model_name: "Gryphe/MythoMax-L2-13b",
            constructor_name: gryphe_mythomax_l2_13b,
            display_name: "MythoMax 13B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llm360K2Think {
            model_name: "LLM360/K2-Think",
            constructor_name: llm360_k2_think,
            display_name: "K2-Think",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        LatitudegamesWayfarerLarge70bLlama33 {
            model_name: "LatitudeGames/Wayfarer-Large-70B-Llama-3.3",
            constructor_name: latitudegames_wayfarer_large_70b_llama_3_3,
            display_name: "Llama 3.3 70B Wayfarer",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MarinaraspaghettiNemomixUnleashed12b {
            model_name: "MarinaraSpaghetti/NemoMix-Unleashed-12B",
            constructor_name: marinaraspaghetti_nemomix_unleashed_12b,
            display_name: "NemoMix 12B Unleashed",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlama318bInstructFp8 {
            model_name: "Meta-Llama-3-1-8B-Instruct-FP8",
            constructor_name: meta_llama_3_1_8b_instruct_fp8,
            display_name: "Llama 3.1 8B (decentralized)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MinimaxM1 {
            model_name: "MiniMax-M1",
            constructor_name: minimax_m1,
            display_name: "MiniMax M1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MinimaxM2 {
            model_name: "MiniMax-M2",
            constructor_name: minimax_m2,
            display_name: "MiniMax M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        MinimaxaiMinimaxM180k {
            model_name: "MiniMaxAI/MiniMax-M1-80k",
            constructor_name: minimaxai_minimax_m1_80k,
            display_name: "MiniMax M1 80K",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NeversleepLumimaidV0270b {
            model_name: "NeverSleep/Lumimaid-v0.2-70B",
            constructor_name: neversleep_lumimaid_v0_2_70b,
            display_name: "Lumimaid v0.2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NousresearchHermes470bThinking {
            model_name: "NousResearch/Hermes-4-70B:thinking",
            constructor_name: nousresearch_hermes_4_70b_thinking,
            display_name: "Hermes 4 (Thinking)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NousresearchHermes3Llama3170b {
            model_name: "NousResearch/hermes-3-llama-3.1-70b",
            constructor_name: nousresearch_hermes_3_llama_3_1_70b,
            display_name: "Hermes 3 70B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NousresearchHermes4405b {
            model_name: "NousResearch/hermes-4-405b",
            constructor_name: nousresearch_hermes_4_405b,
            display_name: "Hermes 4 Large",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        NousresearchHermes4405bThinking {
            model_name: "NousResearch/hermes-4-405b:thinking",
            constructor_name: nousresearch_hermes_4_405b_thinking,
            display_name: "Hermes 4 Large (Thinking)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        NousresearchHermes470b {
            model_name: "NousResearch/hermes-4-70b",
            constructor_name: nousresearch_hermes_4_70b,
            display_name: "Hermes 4 Medium",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwen3527bAnko {
            model_name: "Qwen3.5-27B-Anko",
            constructor_name: qwen3_5_27b_anko,
            display_name: "Qwen3.5 27B Anko",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bBluestarDerestricted {
            model_name: "Qwen3.5-27B-BlueStar-Derestricted",
            constructor_name: qwen3_5_27b_bluestar_derestricted,
            display_name: "Qwen3.5 27B BlueStar Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bBluestarDerestrictedLite {
            model_name: "Qwen3.5-27B-BlueStar-Derestricted-Lite",
            constructor_name: qwen3_5_27b_bluestar_derestricted_lite,
            display_name: "Qwen3.5 27B BlueStar Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bBluestarV2Derestricted {
            model_name: "Qwen3.5-27B-BlueStar-v2-Derestricted",
            constructor_name: qwen3_5_27b_bluestar_v2_derestricted,
            display_name: "Qwen3.5 27B BlueStar v2 Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bBluestarV2DerestrictedLite {
            model_name: "Qwen3.5-27B-BlueStar-v2-Derestricted-Lite",
            constructor_name: qwen3_5_27b_bluestar_v2_derestricted_lite,
            display_name: "Qwen3.5 27B BlueStar v2 Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bBluestarV3Derestricted {
            model_name: "Qwen3.5-27B-BlueStar-v3-Derestricted",
            constructor_name: qwen3_5_27b_bluestar_v3_derestricted,
            display_name: "Qwen3.5 27B BlueStar v3 Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bBluestarV3DerestrictedLite {
            model_name: "Qwen3.5-27B-BlueStar-v3-Derestricted-Lite",
            constructor_name: qwen3_5_27b_bluestar_v3_derestricted_lite,
            display_name: "Qwen3.5 27B BlueStar v3 Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bDerestricted {
            model_name: "Qwen3.5-27B-Derestricted",
            constructor_name: qwen3_5_27b_derestricted,
            display_name: "Qwen3.5 27B Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bInfracelestial {
            model_name: "Qwen3.5-27B-Infracelestial",
            constructor_name: qwen3_5_27b_infracelestial,
            display_name: "Qwen3.5 27B Infracelestial",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bMarvinDpoV2Derestricted {
            model_name: "Qwen3.5-27B-Marvin-DPO-V2-Derestricted",
            constructor_name: qwen3_5_27b_marvin_dpo_v2_derestricted,
            display_name: "Qwen3.5 27B Marvin DPO V2 Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bMarvinDpoV2DerestrictedLite {
            model_name: "Qwen3.5-27B-Marvin-DPO-V2-Derestricted-Lite",
            constructor_name: qwen3_5_27b_marvin_dpo_v2_derestricted_lite,
            display_name: "Qwen3.5 27B Marvin DPO V2 Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bMarvinV2Derestricted {
            model_name: "Qwen3.5-27B-Marvin-V2-Derestricted",
            constructor_name: qwen3_5_27b_marvin_v2_derestricted,
            display_name: "Qwen3.5 27B Marvin V2 Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bMarvinV2DerestrictedLite {
            model_name: "Qwen3.5-27B-Marvin-V2-Derestricted-Lite",
            constructor_name: qwen3_5_27b_marvin_v2_derestricted_lite,
            display_name: "Qwen3.5 27B Marvin V2 Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bMusicaV1 {
            model_name: "Qwen3.5-27B-Musica-v1",
            constructor_name: qwen3_5_27b_musica_v1,
            display_name: "Qwen3.5 27B Musica v1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bNanovelDerestricted {
            model_name: "Qwen3.5-27B-NaNovel-Derestricted",
            constructor_name: qwen3_5_27b_nanovel_derestricted,
            display_name: "Qwen3.5 27B NaNovel Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bNanovelDerestrictedLite {
            model_name: "Qwen3.5-27B-NaNovel-Derestricted-Lite",
            constructor_name: qwen3_5_27b_nanovel_derestricted_lite,
            display_name: "Qwen3.5 27B NaNovel Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bOmegaEvolutionV20Derestricted {
            model_name: "Qwen3.5-27B-Omega-Evolution-v2.0-Derestricted",
            constructor_name: qwen3_5_27b_omega_evolution_v2_0_derestricted,
            display_name: "Qwen3.5 27B Omega Evolution v2.0 Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bOmegaEvolutionV20DerestrictedLite {
            model_name: "Qwen3.5-27B-Omega-Evolution-v2.0-Derestricted-Lite",
            constructor_name: qwen3_5_27b_omega_evolution_v2_0_derestricted_lite,
            display_name: "Qwen3.5 27B Omega Evolution v2.0 Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bOmegaEvolutionV22Derestricted {
            model_name: "Qwen3.5-27B-Omega-Evolution-v2.2-Derestricted",
            constructor_name: qwen3_5_27b_omega_evolution_v2_2_derestricted,
            display_name: "Qwen3.5 27B Omega Evolution v2.2 Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bOmegaEvolutionV22DerestrictedLite {
            model_name: "Qwen3.5-27B-Omega-Evolution-v2.2-Derestricted-Lite",
            constructor_name: qwen3_5_27b_omega_evolution_v2_2_derestricted_lite,
            display_name: "Qwen3.5 27B Omega Evolution v2.2 Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bQueenDerestricted {
            model_name: "Qwen3.5-27B-Queen-Derestricted",
            constructor_name: qwen3_5_27b_queen_derestricted,
            display_name: "Qwen3.5 27B Queen Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bQueenDerestrictedLite {
            model_name: "Qwen3.5-27B-Queen-Derestricted-Lite",
            constructor_name: qwen3_5_27b_queen_derestricted_lite,
            display_name: "Qwen3.5 27B Queen Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bRprmaxV1 {
            model_name: "Qwen3.5-27B-RpRMax-v1",
            constructor_name: qwen3_5_27b_rprmax_v1,
            display_name: "Qwen3.5 27B RpRMax v1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bVividDurian {
            model_name: "Qwen3.5-27B-Vivid-Durian",
            constructor_name: qwen3_5_27b_vivid_durian,
            display_name: "Qwen3.5 27B Vivid Durian",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bWriterDerestricted {
            model_name: "Qwen3.5-27B-Writer-Derestricted",
            constructor_name: qwen3_5_27b_writer_derestricted,
            display_name: "Qwen3.5 27B Writer Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bWriterDerestrictedLite {
            model_name: "Qwen3.5-27B-Writer-Derestricted-Lite",
            constructor_name: qwen3_5_27b_writer_derestricted_lite,
            display_name: "Qwen3.5 27B Writer Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bWriterV2Derestricted {
            model_name: "Qwen3.5-27B-Writer-V2-Derestricted",
            constructor_name: qwen3_5_27b_writer_v2_derestricted,
            display_name: "Qwen3.5 27B Writer V2 Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bWriterV2DerestrictedLite {
            model_name: "Qwen3.5-27B-Writer-V2-Derestricted-Lite",
            constructor_name: qwen3_5_27b_writer_v2_derestricted_lite,
            display_name: "Qwen3.5 27B Writer V2 Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bEaricaDerestricted {
            model_name: "Qwen3.5-27B-earica-Derestricted",
            constructor_name: qwen3_5_27b_earica_derestricted,
            display_name: "Qwen3.5 27B earica Derestricted",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3527bEaricaDerestrictedLite {
            model_name: "Qwen3.5-27B-earica-Derestricted-Lite",
            constructor_name: qwen3_5_27b_earica_derestricted_lite,
            display_name: "Qwen3.5 27B earica Derestricted Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ReadyartMs32TheOmegaDirective24bUnslopV20 {
            model_name: "ReadyArt/MS3.2-The-Omega-Directive-24B-Unslop-v2.0",
            constructor_name: readyart_ms3_2_the_omega_directive_24b_unslop_v2_0,
            display_name: "Omega Directive 24B Unslop v2.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SalesforceLlamaXlam270bFcR {
            model_name: "Salesforce/Llama-xLAM-2-70b-fc-r",
            constructor_name: salesforce_llama_xlam_2_70b_fc_r,
            display_name: "Llama-xLAM-2 70B fc-r",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Sao10kL38bSthenoV32 {
            model_name: "Sao10K/L3-8B-Stheno-v3.2",
            constructor_name: sao10k_l3_8b_stheno_v3_2,
            display_name: "Sao10K Stheno 8b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Sao10kL3170bEuryaleV22 {
            model_name: "Sao10K/L3.1-70B-Euryale-v2.2",
            constructor_name: sao10k_l3_1_70b_euryale_v2_2,
            display_name: "Llama 3.1 70B Euryale",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Sao10kL3170bHanamiX1 {
            model_name: "Sao10K/L3.1-70B-Hanami-x1",
            constructor_name: sao10k_l3_1_70b_hanami_x1,
            display_name: "Llama 3.1 70B Hanami",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Sao10kL3370bEuryaleV23 {
            model_name: "Sao10K/L3.3-70B-Euryale-v2.3",
            constructor_name: sao10k_l3_3_70b_euryale_v2_3,
            display_name: "Llama 3.3 70B Euryale",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SteelskullL33CuMaiR170b {
            model_name: "Steelskull/L3.3-Cu-Mai-R1-70b",
            constructor_name: steelskull_l3_3_cu_mai_r1_70b,
            display_name: "Llama 3.3 70B Cu Mai",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SteelskullL33ElectraR170b {
            model_name: "Steelskull/L3.3-Electra-R1-70b",
            constructor_name: steelskull_l3_3_electra_r1_70b,
            display_name: "Steelskull Electra R1 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SteelskullL33MsEvayale70b {
            model_name: "Steelskull/L3.3-MS-Evayale-70B",
            constructor_name: steelskull_l3_3_ms_evayale_70b,
            display_name: "Evayale 70b ",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SteelskullL33MsNevoria70b {
            model_name: "Steelskull/L3.3-MS-Nevoria-70b",
            constructor_name: steelskull_l3_3_ms_nevoria_70b,
            display_name: "Steelskull Nevoria 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SteelskullL33NevoriaR170b {
            model_name: "Steelskull/L3.3-Nevoria-R1-70b",
            constructor_name: steelskull_l3_3_nevoria_r1_70b,
            display_name: "Steelskull Nevoria R1 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeDeepseekV31 {
            model_name: "TEE/deepseek-v3.1",
            constructor_name: tee_deepseek_v3_1,
            display_name: "DeepSeek V3.1 TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeDeepseekV32 {
            model_name: "TEE/deepseek-v3.2",
            constructor_name: tee_deepseek_v3_2,
            display_name: "DeepSeek V3.2 TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeDeepseekV4Flash {
            model_name: "TEE/deepseek-v4-flash",
            constructor_name: tee_deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeGemma327bIt {
            model_name: "TEE/gemma-3-27b-it",
            constructor_name: tee_gemma_3_27b_it,
            display_name: "Gemma 3 27B TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeGemma426bA4bUncensored {
            model_name: "TEE/gemma-4-26b-a4b-uncensored",
            constructor_name: tee_gemma_4_26b_a4b_uncensored,
            display_name: "Gemma 4 26B A4B Uncensored TEE",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGemma431bIt {
            model_name: "TEE/gemma-4-31b-it",
            constructor_name: tee_gemma_4_31b_it,
            display_name: "Gemma 4 31B IT TEE",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGemma431b {
            model_name: "TEE/gemma4-31b",
            constructor_name: tee_gemma4_31b,
            display_name: "Gemma 4 31B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        TeeGemma431bThinking {
            model_name: "TEE/gemma4-31b:thinking",
            constructor_name: tee_gemma4_31b_thinking,
            display_name: "Gemma 4 31B Thinking TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        TeeGlm47 {
            model_name: "TEE/glm-4.7",
            constructor_name: tee_glm_4_7,
            display_name: "GLM 4.7 TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeGlm51 {
            model_name: "TEE/glm-5.1",
            constructor_name: tee_glm_5_1,
            display_name: "GLM 5.1 TEE",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        TeeGlm51Thinking {
            model_name: "TEE/glm-5.1-thinking",
            constructor_name: tee_glm_5_1_thinking,
            display_name: "GLM 5.1 Thinking TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGlm52 {
            model_name: "TEE/glm-5.2",
            constructor_name: tee_glm_5_2,
            display_name: "GLM 5.2 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGlm52Thinking {
            model_name: "TEE/glm-5.2:thinking",
            constructor_name: tee_glm_5_2_thinking,
            display_name: "GLM 5.2 Thinking TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeGptOss120b {
            model_name: "TEE/gpt-oss-120b",
            constructor_name: tee_gpt_oss_120b,
            display_name: "GPT-OSS 120B TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeGptOss20b {
            model_name: "TEE/gpt-oss-20b",
            constructor_name: tee_gpt_oss_20b,
            display_name: "GPT-OSS 20B TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeKimiK25 {
            model_name: "TEE/kimi-k2.5",
            constructor_name: tee_kimi_k2_5,
            display_name: "Kimi K2.5 TEE",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        TeeKimiK25Thinking {
            model_name: "TEE/kimi-k2.5-thinking",
            constructor_name: tee_kimi_k2_5_thinking,
            display_name: "Kimi K2.5 Thinking TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        TeeKimiK26 {
            model_name: "TEE/kimi-k2.6",
            constructor_name: tee_kimi_k2_6,
            display_name: "Kimi K2.6 TEE",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeKimiK3 {
            model_name: "TEE/kimi-k3",
            constructor_name: tee_kimi_k3,
            display_name: "Kimi K3 TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        TeeLlama3370b {
            model_name: "TEE/llama3-3-70b",
            constructor_name: tee_llama3_3_70b,
            display_name: "Llama 3.3 70B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeMinimaxM25 {
            model_name: "TEE/minimax-m2.5",
            constructor_name: tee_minimax_m2_5,
            display_name: "MiniMax M2.5 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeQwen25Vl72bInstruct {
            model_name: "TEE/qwen2.5-vl-72b-instruct",
            constructor_name: tee_qwen2_5_vl_72b_instruct,
            display_name: "Qwen2.5 VL 72B TEE",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        TeeQwen35122bA10b {
            model_name: "TEE/qwen3.5-122b-a10b",
            constructor_name: tee_qwen3_5_122b_a10b,
            display_name: "Qwen3.5 122B A10B TEE",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeQwen3527b {
            model_name: "TEE/qwen3.5-27b",
            constructor_name: tee_qwen3_5_27b,
            display_name: "Qwen3.5 27B TEE",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        TeeQwen35397bA17b {
            model_name: "TEE/qwen3.5-397b-a17b",
            constructor_name: tee_qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B A17B TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeQwen3627b {
            model_name: "TEE/qwen3.6-27b",
            constructor_name: tee_qwen3_6_27b,
            display_name: "Qwen3.6 27B TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        TeeQwen3635bA3b {
            model_name: "TEE/qwen3.6-35b-a3b",
            constructor_name: tee_qwen3_6_35b_a3b,
            display_name: "Qwen3.6 35B A3B TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeQwen3635bA3bUncensored {
            model_name: "TEE/qwen3.6-35b-a3b-uncensored",
            constructor_name: tee_qwen3_6_35b_a3b_uncensored,
            display_name: "Qwen3.6 35B A3B Uncensored TEE",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThudmGlm432b0414 {
            model_name: "THUDM/GLM-4-32B-0414",
            constructor_name: thudm_glm_4_32b_0414,
            display_name: "GLM 4 32B 0414",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThudmGlm49b0414 {
            model_name: "THUDM/GLM-4-9B-0414",
            constructor_name: thudm_glm_4_9b_0414,
            display_name: "GLM 4 9B 0414",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThudmGlmZ19b0414 {
            model_name: "THUDM/GLM-Z1-9B-0414",
            constructor_name: thudm_glm_z1_9b_0414,
            display_name: "GLM Z1 9B 0414",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerAnubis70bV1 {
            model_name: "TheDrummer/Anubis-70B-v1",
            constructor_name: thedrummer_anubis_70b_v1,
            display_name: "Anubis 70B v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerAnubis70bV11 {
            model_name: "TheDrummer/Anubis-70B-v1.1",
            constructor_name: thedrummer_anubis_70b_v1_1,
            display_name: "Anubis 70B v1.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerCydonia24bV2 {
            model_name: "TheDrummer/Cydonia-24B-v2",
            constructor_name: thedrummer_cydonia_24b_v2,
            display_name: "The Drummer Cydonia 24B v2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerCydonia24bV4 {
            model_name: "TheDrummer/Cydonia-24B-v4",
            constructor_name: thedrummer_cydonia_24b_v4,
            display_name: "The Drummer Cydonia 24B v4",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerCydonia24bV41 {
            model_name: "TheDrummer/Cydonia-24B-v4.1",
            constructor_name: thedrummer_cydonia_24b_v4_1,
            display_name: "The Drummer Cydonia 24B v4.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerCydonia24bV43 {
            model_name: "TheDrummer/Cydonia-24B-v4.3",
            constructor_name: thedrummer_cydonia_24b_v4_3,
            display_name: "The Drummer Cydonia 24B v4.3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerMagidonia24bV43 {
            model_name: "TheDrummer/Magidonia-24B-v4.3",
            constructor_name: thedrummer_magidonia_24b_v4_3,
            display_name: "The Drummer Magidonia 24B v4.3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerRocinante12bV11 {
            model_name: "TheDrummer/Rocinante-12B-v1.1",
            constructor_name: thedrummer_rocinante_12b_v1_1,
            display_name: "Rocinante 12b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThedrummerUnslopnemo12bV41 {
            model_name: "TheDrummer/UnslopNemo-12B-v4.1",
            constructor_name: thedrummer_unslopnemo_12b_v4_1,
            display_name: "UnslopNemo 12b v4",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ThedrummerSkyfall36bV2 {
            model_name: "TheDrummer/skyfall-36b-v2",
            constructor_name: thedrummer_skyfall_36b_v2,
            display_name: "TheDrummer Skyfall 36B V2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        TongyiZhiwenQwenlongL132b {
            model_name: "Tongyi-Zhiwen/QwenLong-L1-32B",
            constructor_name: tongyi_zhiwen_qwenlong_l1_32b,
            display_name: "QwenLong L1 32B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VongolachoukoStarcannonUnleashed12bV10 {
            model_name: "VongolaChouko/Starcannon-Unleashed-12B-v1.0",
            constructor_name: vongolachouko_starcannon_unleashed_12b_v1_0,
            display_name: "Mistral Nemo Starcannon 12b v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AbacusaiDracarys72bInstruct {
            model_name: "abacusai/Dracarys-72B-Instruct",
            constructor_name: abacusai_dracarys_72b_instruct,
            display_name: "Llama 3.1 70B Dracarys 2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AionLabsAion20 {
            model_name: "aion-labs/aion-2.0",
            constructor_name: aion_labs_aion_2_0,
            display_name: "AionLabs: Aion-2.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AionLabsAion25 {
            model_name: "aion-labs/aion-2.5",
            constructor_name: aion_labs_aion_2_5,
            display_name: "AionLabs: Aion-2.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AionLabsAion30 {
            model_name: "aion-labs/aion-3.0",
            constructor_name: aion_labs_aion_3_0,
            display_name: "AionLabs: Aion 3.0",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        AionLabsAion30Mini {
            model_name: "aion-labs/aion-3.0-mini",
            constructor_name: aion_labs_aion_3_0_mini,
            display_name: "AionLabs: Aion 3.0 Mini",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        AionLabsAionRpLlama318b {
            model_name: "aion-labs/aion-rp-llama-3.1-8b",
            constructor_name: aion_labs_aion_rp_llama_3_1_8b,
            display_name: "Llama 3.1 8b (uncensored)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AlibabaQwen3627b {
            model_name: "alibaba/qwen3.6-27b",
            constructor_name: alibaba_qwen3_6_27b,
            display_name: "Qwen3.6 27B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        AlibabaQwen3627bThinking {
            model_name: "alibaba/qwen3.6-27b:thinking",
            constructor_name: alibaba_qwen3_6_27b_thinking,
            display_name: "Qwen3.6 27B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        AlibabaQwen36Flash {
            model_name: "alibaba/qwen3.6-flash",
            constructor_name: alibaba_qwen3_6_flash,
            display_name: "Qwen3.6 Flash",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        AmazonNova2LiteV1 {
            model_name: "amazon/nova-2-lite-v1",
            constructor_name: amazon_nova_2_lite_v1,
            display_name: "Amazon Nova 2 Lite",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AmazonNovaLiteV1 {
            model_name: "amazon/nova-lite-v1",
            constructor_name: amazon_nova_lite_v1,
            display_name: "Amazon Nova Lite 1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AmazonNovaMicroV1 {
            model_name: "amazon/nova-micro-v1",
            constructor_name: amazon_nova_micro_v1,
            display_name: "Amazon Nova Micro 1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AmazonNovaProV1 {
            model_name: "amazon/nova-pro-v1",
            constructor_name: amazon_nova_pro_v1,
            display_name: "Amazon Nova Pro 1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AnthraciteOrgMagnumV272b {
            model_name: "anthracite-org/magnum-v2-72b",
            constructor_name: anthracite_org_magnum_v2_72b,
            display_name: "Magnum V2 72B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AnthraciteOrgMagnumV472b {
            model_name: "anthracite-org/magnum-v4-72b",
            constructor_name: anthracite_org_magnum_v4_72b,
            display_name: "Magnum v4 72B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        AnthropicClaudeFable5 {
            model_name: "anthropic/claude-fable-5",
            constructor_name: anthropic_claude_fable_5,
            display_name: "Claude Fable 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeFableLatest {
            model_name: "anthropic/claude-fable-latest",
            constructor_name: anthropic_claude_fable_latest,
            display_name: "Claude Fable Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaikuLatest {
            model_name: "anthropic/claude-haiku-latest",
            constructor_name: anthropic_claude_haiku_latest,
            display_name: "Claude Haiku Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46 {
            model_name: "anthropic/claude-opus-4.6",
            constructor_name: anthropic_claude_opus_4_6,
            display_name: "Claude 4.6 Opus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46Thinking {
            model_name: "anthropic/claude-opus-4.6:thinking",
            constructor_name: anthropic_claude_opus_4_6_thinking,
            display_name: "Claude 4.6 Opus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46ThinkingLow {
            model_name: "anthropic/claude-opus-4.6:thinking:low",
            constructor_name: anthropic_claude_opus_4_6_thinking_low,
            display_name: "Claude 4.6 Opus Thinking Low",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46ThinkingMax {
            model_name: "anthropic/claude-opus-4.6:thinking:max",
            constructor_name: anthropic_claude_opus_4_6_thinking_max,
            display_name: "Claude 4.6 Opus Thinking Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46ThinkingMedium {
            model_name: "anthropic/claude-opus-4.6:thinking:medium",
            constructor_name: anthropic_claude_opus_4_6_thinking_medium,
            display_name: "Claude 4.6 Opus Thinking Medium",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus47 {
            model_name: "anthropic/claude-opus-4.7",
            constructor_name: anthropic_claude_opus_4_7,
            display_name: "Claude 4.7 Opus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus47Thinking {
            model_name: "anthropic/claude-opus-4.7:thinking",
            constructor_name: anthropic_claude_opus_4_7_thinking,
            display_name: "Claude 4.7 Opus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus48 {
            model_name: "anthropic/claude-opus-4.8",
            constructor_name: anthropic_claude_opus_4_8,
            display_name: "Claude Opus 4.8",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus48Thinking {
            model_name: "anthropic/claude-opus-4.8:thinking",
            constructor_name: anthropic_claude_opus_4_8_thinking,
            display_name: "Claude Opus 4.8 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus5 {
            model_name: "anthropic/claude-opus-5",
            constructor_name: anthropic_claude_opus_5,
            display_name: "Claude Opus 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpusLatest {
            model_name: "anthropic/claude-opus-latest",
            constructor_name: anthropic_claude_opus_latest,
            display_name: "Claude Opus Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet46 {
            model_name: "anthropic/claude-sonnet-4.6",
            constructor_name: anthropic_claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet46Thinking {
            model_name: "anthropic/claude-sonnet-4.6:thinking",
            constructor_name: anthropic_claude_sonnet_4_6_thinking,
            display_name: "Claude Sonnet 4.6 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet5 {
            model_name: "anthropic/claude-sonnet-5",
            constructor_name: anthropic_claude_sonnet_5,
            display_name: "Claude Sonnet 5",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet5Thinking {
            model_name: "anthropic/claude-sonnet-5:thinking",
            constructor_name: anthropic_claude_sonnet_5_thinking,
            display_name: "Claude Sonnet 5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnetLatest {
            model_name: "anthropic/claude-sonnet-latest",
            constructor_name: anthropic_claude_sonnet_latest,
            display_name: "Claude Sonnet Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ArceeAiTrinityLargeThinking {
            model_name: "arcee-ai/trinity-large-thinking",
            constructor_name: arcee_ai_trinity_large_thinking,
            display_name: "Trinity Large Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Asi1Mini {
            model_name: "asi1-mini",
            constructor_name: asi1_mini,
            display_name: "ASI1 Mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        AutoModel {
            model_name: "auto-model",
            constructor_name: auto_model,
            display_name: "Auto model",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AutoModelBasic {
            model_name: "auto-model-basic",
            constructor_name: auto_model_basic,
            display_name: "Auto model (Basic)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AutoModelPremium {
            model_name: "auto-model-premium",
            constructor_name: auto_model_premium,
            display_name: "Auto model (Premium)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AutoModelStandard {
            model_name: "auto-model-standard",
            constructor_name: auto_model_standard,
            display_name: "Auto model (Standard)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AzureGpt4Turbo {
            model_name: "azure-gpt-4-turbo",
            constructor_name: azure_gpt_4_turbo,
            display_name: "Azure gpt-4-turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AzureGpt4o {
            model_name: "azure-gpt-4o",
            constructor_name: azure_gpt_4o,
            display_name: "Azure gpt-4o",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AzureGpt4oMini {
            model_name: "azure-gpt-4o-mini",
            constructor_name: azure_gpt_4o_mini,
            display_name: "Azure gpt-4o-mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AzureO1 {
            model_name: "azure-o1",
            constructor_name: azure_o1,
            display_name: "Azure o1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AzureO3Mini {
            model_name: "azure-o3-mini",
            constructor_name: azure_o3_mini,
            display_name: "Azure o3-mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BaiduErnie45Vl28bA3b {
            model_name: "baidu/ernie-4.5-vl-28b-a3b",
            constructor_name: baidu_ernie_4_5_vl_28b_a3b,
            display_name: "ERNIE 4.5 VL 28B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        BasetenKimiK2InstructFp4 {
            model_name: "baseten/Kimi-K2-Instruct-FP4",
            constructor_name: baseten_kimi_k2_instruct_fp4,
            display_name: "Kimi K2 0711 Instruct FP4",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Brave {
            model_name: "brave",
            constructor_name: brave,
            display_name: "Brave (Answers)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BravePro {
            model_name: "brave-pro",
            constructor_name: brave_pro,
            display_name: "Brave (Pro)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BraveResearch {
            model_name: "brave-research",
            constructor_name: brave_research,
            display_name: "Brave (Research)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BytedanceSeedSeed20Lite {
            model_name: "bytedance-seed/seed-2.0-lite",
            constructor_name: bytedance_seed_seed_2_0_lite,
            display_name: "ByteDance Seed 2.0 Lite",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        BytedanceDoubaoSeed21Pro {
            model_name: "bytedance/doubao-seed-2.1-pro",
            constructor_name: bytedance_doubao_seed_2_1_pro,
            display_name: "Doubao Seed 2.1 Pro",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BytedanceDoubaoSeed21Turbo {
            model_name: "bytedance/doubao-seed-2.1-turbo",
            constructor_name: bytedance_doubao_seed_2_1_turbo,
            display_name: "Doubao Seed 2.1 Turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BytedanceDoubaoSeedCharacter {
            model_name: "bytedance/doubao-seed-character",
            constructor_name: bytedance_doubao_seed_character,
            display_name: "Doubao Seed Character",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Celeris1 {
            model_name: "celeris-1",
            constructor_name: celeris_1,
            display_name: "Celeris 1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ChutesaiMistralSmall3224bInstruct2506 {
            model_name: "chutesai/Mistral-Small-3.2-24B-Instruct-2506",
            constructor_name: chutesai_mistral_small_3_2_24b_instruct_2506,
            display_name: "Mistral Small 3.2 24b Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ClaudeHaiku4520251001 {
            model_name: "claude-haiku-4-5-20251001",
            constructor_name: claude_haiku_4_5_20251001,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku4520251001Thinking {
            model_name: "claude-haiku-4-5-20251001-thinking",
            constructor_name: claude_haiku_4_5_20251001_thinking,
            display_name: "Claude Haiku 4.5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4120250805 {
            model_name: "claude-opus-4-1-20250805",
            constructor_name: claude_opus_4_1_20250805,
            display_name: "Claude 4.1 Opus",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus41Thinking {
            model_name: "claude-opus-4-1-thinking",
            constructor_name: claude_opus_4_1_thinking,
            display_name: "Claude 4.1 Opus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus41Thinking1024 {
            model_name: "claude-opus-4-1-thinking:1024",
            constructor_name: claude_opus_4_1_thinking_1024,
            display_name: "Claude 4.1 Opus Thinking (1K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus41Thinking32000 {
            model_name: "claude-opus-4-1-thinking:32000",
            constructor_name: claude_opus_4_1_thinking_32000,
            display_name: "Claude 4.1 Opus Thinking (32K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus41Thinking32768 {
            model_name: "claude-opus-4-1-thinking:32768",
            constructor_name: claude_opus_4_1_thinking_32768,
            display_name: "Claude 4.1 Opus Thinking (32K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus41Thinking8192 {
            model_name: "claude-opus-4-1-thinking:8192",
            constructor_name: claude_opus_4_1_thinking_8192,
            display_name: "Claude 4.1 Opus Thinking (8K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus420250514 {
            model_name: "claude-opus-4-20250514",
            constructor_name: claude_opus_4_20250514,
            display_name: "Claude 4 Opus",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4520251101 {
            model_name: "claude-opus-4-5-20251101",
            constructor_name: claude_opus_4_5_20251101,
            display_name: "Claude 4.5 Opus",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4520251101Thinking {
            model_name: "claude-opus-4-5-20251101:thinking",
            constructor_name: claude_opus_4_5_20251101_thinking,
            display_name: "Claude 4.5 Opus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4Thinking {
            model_name: "claude-opus-4-thinking",
            constructor_name: claude_opus_4_thinking,
            display_name: "Claude 4 Opus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4Thinking1024 {
            model_name: "claude-opus-4-thinking:1024",
            constructor_name: claude_opus_4_thinking_1024,
            display_name: "Claude 4 Opus Thinking (1K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4Thinking32000 {
            model_name: "claude-opus-4-thinking:32000",
            constructor_name: claude_opus_4_thinking_32000,
            display_name: "Claude 4 Opus Thinking (32K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4Thinking32768 {
            model_name: "claude-opus-4-thinking:32768",
            constructor_name: claude_opus_4_thinking_32768,
            display_name: "Claude 4 Opus Thinking (32K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus4Thinking8192 {
            model_name: "claude-opus-4-thinking:8192",
            constructor_name: claude_opus_4_thinking_8192,
            display_name: "Claude 4 Opus Thinking (8K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet420250514 {
            model_name: "claude-sonnet-4-20250514",
            constructor_name: claude_sonnet_4_20250514,
            display_name: "Claude 4 Sonnet",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4520250929 {
            model_name: "claude-sonnet-4-5-20250929",
            constructor_name: claude_sonnet_4_5_20250929,
            display_name: "Claude Sonnet 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4520250929Thinking {
            model_name: "claude-sonnet-4-5-20250929-thinking",
            constructor_name: claude_sonnet_4_5_20250929_thinking,
            display_name: "Claude Sonnet 4.5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4Thinking {
            model_name: "claude-sonnet-4-thinking",
            constructor_name: claude_sonnet_4_thinking,
            display_name: "Claude 4 Sonnet Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4Thinking1024 {
            model_name: "claude-sonnet-4-thinking:1024",
            constructor_name: claude_sonnet_4_thinking_1024,
            display_name: "Claude 4 Sonnet Thinking (1K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4Thinking32768 {
            model_name: "claude-sonnet-4-thinking:32768",
            constructor_name: claude_sonnet_4_thinking_32768,
            display_name: "Claude 4 Sonnet Thinking (32K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4Thinking64000 {
            model_name: "claude-sonnet-4-thinking:64000",
            constructor_name: claude_sonnet_4_thinking_64000,
            display_name: "Claude 4 Sonnet Thinking (64K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet4Thinking8192 {
            model_name: "claude-sonnet-4-thinking:8192",
            constructor_name: claude_sonnet_4_thinking_8192,
            display_name: "Claude 4 Sonnet Thinking (8K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClawHigh {
            model_name: "claw-high",
            constructor_name: claw_high,
            display_name: "Claw High",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClawLow {
            model_name: "claw-low",
            constructor_name: claw_low,
            display_name: "Claw Low",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ClawMedium {
            model_name: "claw-medium",
            constructor_name: claw_medium,
            display_name: "Claw Medium",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereCommandRPlus082024 {
            model_name: "cohere/command-r-plus-08-2024",
            constructor_name: cohere_command_r_plus_08_2024,
            display_name: "Cohere: Command R+",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereNorthMiniCode {
            model_name: "cohere/north-mini-code",
            constructor_name: cohere_north_mini_code,
            display_name: "Cohere North Mini Code 1.0",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        CommandAPlus052026 {
            model_name: "command-a-plus-05-2026",
            constructor_name: command_a_plus_05_2026,
            display_name: "Cohere Command A+ (05/2026)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        CommandAReasoning082025 {
            model_name: "command-a-reasoning-08-2025",
            constructor_name: command_a_reasoning_08_2025,
            display_name: "Cohere Command A (08/2025)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CrofaiGreg2Super {
            model_name: "crofai/greg-2-super",
            constructor_name: crofai_greg_2_super,
            display_name: "Greg 2 Super",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CrofaiGreg2Ultra {
            model_name: "crofai/greg-2-ultra",
            constructor_name: crofai_greg_2_ultra,
            display_name: "Greg 2 Ultra",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Deepclaude {
            model_name: "deepclaude",
            constructor_name: deepclaude,
            display_name: "DeepClaude",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        DeepcogitoCogitoV1PreviewQwen32b {
            model_name: "deepcogito/cogito-v1-preview-qwen-32B",
            constructor_name: deepcogito_cogito_v1_preview_qwen_32b,
            display_name: "Cogito v1 Preview Qwen 32B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekR10528 {
            model_name: "deepseek-ai/DeepSeek-R1-0528",
            constructor_name: deepseek_ai_deepseek_r1_0528,
            display_name: "DeepSeek R1 0528",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekV31 {
            model_name: "deepseek-ai/DeepSeek-V3.1",
            constructor_name: deepseek_ai_deepseek_v3_1,
            display_name: "DeepSeek V3.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekV31Terminus {
            model_name: "deepseek-ai/DeepSeek-V3.1-Terminus",
            constructor_name: deepseek_ai_deepseek_v3_1_terminus,
            display_name: "DeepSeek V3.1 Terminus",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31TerminusThinking {
            model_name: "deepseek-ai/DeepSeek-V3.1-Terminus:thinking",
            constructor_name: deepseek_ai_deepseek_v3_1_terminus_thinking,
            display_name: "DeepSeek V3.1 Terminus (Thinking)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekAiDeepseekV31Thinking {
            model_name: "deepseek-ai/DeepSeek-V3.1:thinking",
            constructor_name: deepseek_ai_deepseek_v3_1_thinking,
            display_name: "DeepSeek V3.1 Thinking",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekV32Exp {
            model_name: "deepseek-ai/deepseek-v3.2-exp",
            constructor_name: deepseek_ai_deepseek_v3_2_exp,
            display_name: "DeepSeek V3.2 Exp",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekAiDeepseekV32ExpThinking {
            model_name: "deepseek-ai/deepseek-v3.2-exp-thinking",
            constructor_name: deepseek_ai_deepseek_v3_2_exp_thinking,
            display_name: "DeepSeek V3.2 Exp Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekChat {
            model_name: "deepseek-chat",
            constructor_name: deepseek_chat,
            display_name: "DeepSeek V3/Deepseek Chat",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekChatCheaper {
            model_name: "deepseek-chat-cheaper",
            constructor_name: deepseek_chat_cheaper,
            display_name: "DeepSeek V3/Chat Cheaper",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR1 {
            model_name: "deepseek-r1",
            constructor_name: deepseek_r1,
            display_name: "DeepSeek R1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekR1Sambanova {
            model_name: "deepseek-r1-sambanova",
            constructor_name: deepseek_r1_sambanova,
            display_name: "DeepSeek R1 Fast",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekReasoner {
            model_name: "deepseek-reasoner",
            constructor_name: deepseek_reasoner,
            display_name: "DeepSeek Reasoner",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekReasonerCheaper {
            model_name: "deepseek-reasoner-cheaper",
            constructor_name: deepseek_reasoner_cheaper,
            display_name: "Deepseek R1 Cheaper",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekV30324 {
            model_name: "deepseek-v3-0324",
            constructor_name: deepseek_v3_0324,
            display_name: "DeepSeek Chat 0324",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekLatest {
            model_name: "deepseek/deepseek-latest",
            constructor_name: deepseek_deepseek_latest,
            display_name: "DeepSeek Latest",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekProverV2671b {
            model_name: "deepseek/deepseek-prover-v2-671b",
            constructor_name: deepseek_deepseek_prover_v2_671b,
            display_name: "DeepSeek Prover v2 671B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekV32 {
            model_name: "deepseek/deepseek-v3.2",
            constructor_name: deepseek_deepseek_v3_2,
            display_name: "DeepSeek V3.2",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32Thinking {
            model_name: "deepseek/deepseek-v3.2:thinking",
            constructor_name: deepseek_deepseek_v3_2_thinking,
            display_name: "DeepSeek V3.2 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash {
            model_name: "deepseek/deepseek-v4-flash",
            constructor_name: deepseek_deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash0731 {
            model_name: "deepseek/deepseek-v4-flash-0731",
            constructor_name: deepseek_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash0731Cheaper {
            model_name: "deepseek/deepseek-v4-flash-0731-cheaper",
            constructor_name: deepseek_deepseek_v4_flash_0731_cheaper,
            display_name: "DeepSeek V4 Flash 0731 Cheaper",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash0731CheaperThinking {
            model_name: "deepseek/deepseek-v4-flash-0731-cheaper:thinking",
            constructor_name: deepseek_deepseek_v4_flash_0731_cheaper_thinking,
            display_name: "DeepSeek V4 Flash 0731 Cheaper (Thinking)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Flash0731Thinking {
            model_name: "deepseek/deepseek-v4-flash-0731:thinking",
            constructor_name: deepseek_deepseek_v4_flash_0731_thinking,
            display_name: "DeepSeek V4 Flash 0731 (Thinking)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4FlashLatest {
            model_name: "deepseek/deepseek-v4-flash-latest",
            constructor_name: deepseek_deepseek_v4_flash_latest,
            display_name: "DeepSeek V4 Flash Latest",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4FlashThinking {
            model_name: "deepseek/deepseek-v4-flash:thinking",
            constructor_name: deepseek_deepseek_v4_flash_thinking,
            display_name: "DeepSeek V4 Flash (Thinking)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Pro {
            model_name: "deepseek/deepseek-v4-pro",
            constructor_name: deepseek_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4ProCheaper {
            model_name: "deepseek/deepseek-v4-pro-cheaper",
            constructor_name: deepseek_deepseek_v4_pro_cheaper,
            display_name: "DeepSeek V4 Pro Cheaper",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4ProCheaperThinking {
            model_name: "deepseek/deepseek-v4-pro-cheaper:thinking",
            constructor_name: deepseek_deepseek_v4_pro_cheaper_thinking,
            display_name: "DeepSeek V4 Pro Cheaper (Thinking)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4ProThinking {
            model_name: "deepseek/deepseek-v4-pro:thinking",
            constructor_name: deepseek_deepseek_v4_pro_thinking,
            display_name: "DeepSeek V4 Pro (Thinking)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DmindDmind1Mini {
            model_name: "dmind/dmind-1-mini",
            constructor_name: dmind_dmind_1_mini,
            display_name: "DMind-1-Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Doubao15Pro256k {
            model_name: "doubao-1.5-pro-256k",
            constructor_name: doubao_1_5_pro_256k,
            display_name: "Doubao 1.5 Pro 256k",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Doubao15Pro32k {
            model_name: "doubao-1.5-pro-32k",
            constructor_name: doubao_1_5_pro_32k,
            display_name: "Doubao 1.5 Pro 32k",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Doubao15VisionPro32k {
            model_name: "doubao-1.5-vision-pro-32k",
            constructor_name: doubao_1_5_vision_pro_32k,
            display_name: "Doubao 1.5 Vision Pro 32k",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        DoubaoSeed16250615 {
            model_name: "doubao-seed-1-6-250615",
            constructor_name: doubao_seed_1_6_250615,
            display_name: "Doubao Seed 1.6",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DoubaoSeed16Flash250615 {
            model_name: "doubao-seed-1-6-flash-250615",
            constructor_name: doubao_seed_1_6_flash_250615,
            display_name: "Doubao Seed 1.6 Flash",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DoubaoSeed18251215 {
            model_name: "doubao-seed-1-8-251215",
            constructor_name: doubao_seed_1_8_251215,
            display_name: "Doubao Seed 1.8",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DoubaoSeed20CodePreview260215 {
            model_name: "doubao-seed-2-0-code-preview-260215",
            constructor_name: doubao_seed_2_0_code_preview_260215,
            display_name: "Doubao Seed 2.0 Code Preview",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DoubaoSeed20Lite260215 {
            model_name: "doubao-seed-2-0-lite-260215",
            constructor_name: doubao_seed_2_0_lite_260215,
            display_name: "Doubao Seed 2.0 Lite",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DoubaoSeed20Mini260215 {
            model_name: "doubao-seed-2-0-mini-260215",
            constructor_name: doubao_seed_2_0_mini_260215,
            display_name: "Doubao Seed 2.0 Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DoubaoSeed20Pro260215 {
            model_name: "doubao-seed-2-0-pro-260215",
            constructor_name: doubao_seed_2_0_pro_260215,
            display_name: "Doubao Seed 2.0 Pro",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Ernie50ThinkingPreview {
            model_name: "ernie-5.0-thinking-preview",
            constructor_name: ernie_5_0_thinking_preview,
            display_name: "Ernie 5.0 Thinking Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Ernie51 {
            model_name: "ernie-5.1",
            constructor_name: ernie_5_1,
            display_name: "ERNIE 5.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Ernie51Thinking {
            model_name: "ernie-5.1:thinking",
            constructor_name: ernie_5_1_thinking,
            display_name: "ERNIE 5.1 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ErnieX11Preview {
            model_name: "ernie-x1.1-preview",
            constructor_name: ernie_x1_1_preview,
            display_name: "ERNIE X1.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ExaAnswer {
            model_name: "exa-answer",
            constructor_name: exa_answer,
            display_name: "Exa (Answer)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        FailspyMetaLlama370bInstructAbliteratedV35 {
            model_name: "failspy/Meta-Llama-3-70B-Instruct-abliterated-v3.5",
            constructor_name: failspy_meta_llama_3_70b_instruct_abliterated_v3_5,
            display_name: "Llama 3 70B abliterated",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Fastgpt {
            model_name: "fastgpt",
            constructor_name: fastgpt,
            display_name: "Web Answer",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        FeatherlessAiQwerky72b {
            model_name: "featherless-ai/Qwerky-72B",
            constructor_name: featherless_ai_qwerky_72b,
            display_name: "Qwerky 72B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemini20ProExp0205 {
            model_name: "gemini-2.0-pro-exp-02-05",
            constructor_name: gemini_2_0_pro_exp_02_05,
            display_name: "Gemini 2.0 Pro 0205",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini20ProReasoner {
            model_name: "gemini-2.0-pro-reasoner",
            constructor_name: gemini_2_0_pro_reasoner,
            display_name: "Gemini 2.0 Pro Reasoner",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25Flash {
            model_name: "gemini-2.5-flash",
            constructor_name: gemini_2_5_flash,
            display_name: "Gemini 2.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashLite {
            model_name: "gemini-2.5-flash-lite",
            constructor_name: gemini_2_5_flash_lite,
            display_name: "Gemini 2.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashLitePreview0617 {
            model_name: "gemini-2.5-flash-lite-preview-06-17",
            constructor_name: gemini_2_5_flash_lite_preview_06_17,
            display_name: "Gemini 2.5 Flash Lite Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashLitePreview092025 {
            model_name: "gemini-2.5-flash-lite-preview-09-2025",
            constructor_name: gemini_2_5_flash_lite_preview_09_2025,
            display_name: "Gemini 2.5 Flash Lite Preview (09/2025)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashLitePreview092025Thinking {
            model_name: "gemini-2.5-flash-lite-preview-09-2025-thinking",
            constructor_name: gemini_2_5_flash_lite_preview_09_2025_thinking,
            display_name: "Gemini 2.5 Flash Lite Preview (09/2025) – Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashNothinking {
            model_name: "gemini-2.5-flash-nothinking",
            constructor_name: gemini_2_5_flash_nothinking,
            display_name: "Gemini 2.5 Flash (No Thinking)",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashPreview0417 {
            model_name: "gemini-2.5-flash-preview-04-17",
            constructor_name: gemini_2_5_flash_preview_04_17,
            display_name: "Gemini 2.5 Flash Preview",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashPreview0417Thinking {
            model_name: "gemini-2.5-flash-preview-04-17:thinking",
            constructor_name: gemini_2_5_flash_preview_04_17_thinking,
            display_name: "Gemini 2.5 Flash Preview Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashPreview0520 {
            model_name: "gemini-2.5-flash-preview-05-20",
            constructor_name: gemini_2_5_flash_preview_05_20,
            display_name: "Gemini 2.5 Flash 0520",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashPreview0520Thinking {
            model_name: "gemini-2.5-flash-preview-05-20:thinking",
            constructor_name: gemini_2_5_flash_preview_05_20_thinking,
            display_name: "Gemini 2.5 Flash 0520 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashPreview092025 {
            model_name: "gemini-2.5-flash-preview-09-2025",
            constructor_name: gemini_2_5_flash_preview_09_2025,
            display_name: "Gemini 2.5 Flash Preview (09/2025)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashPreview092025Thinking {
            model_name: "gemini-2.5-flash-preview-09-2025-thinking",
            constructor_name: gemini_2_5_flash_preview_09_2025_thinking,
            display_name: "Gemini 2.5 Flash Preview (09/2025) – Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25Pro {
            model_name: "gemini-2.5-pro",
            constructor_name: gemini_2_5_pro,
            display_name: "Gemini 2.5 Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25ProExp0325 {
            model_name: "gemini-2.5-pro-exp-03-25",
            constructor_name: gemini_2_5_pro_exp_03_25,
            display_name: "Gemini 2.5 Pro Experimental 0325",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25ProPreview0325 {
            model_name: "gemini-2.5-pro-preview-03-25",
            constructor_name: gemini_2_5_pro_preview_03_25,
            display_name: "Gemini 2.5 Pro Preview 0325",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25ProPreview0506 {
            model_name: "gemini-2.5-pro-preview-05-06",
            constructor_name: gemini_2_5_pro_preview_05_06,
            display_name: "Gemini 2.5 Pro Preview 0506",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25ProPreview0605 {
            model_name: "gemini-2.5-pro-preview-06-05",
            constructor_name: gemini_2_5_pro_preview_06_05,
            display_name: "Gemini 2.5 Pro Preview 0605",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini3ProImagePreview {
            model_name: "gemini-3-pro-image-preview",
            constructor_name: gemini_3_pro_image_preview,
            display_name: "Gemini 3 Pro Image",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GeminiExp1206 {
            model_name: "gemini-exp-1206",
            constructor_name: gemini_exp_1206,
            display_name: "Gemini 2.0 Pro 1206",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemma412bIt {
            model_name: "gemma-4-12b-it",
            constructor_name: gemma_4_12b_it,
            display_name: "Gemma 4 12B Instruct",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemma4E2bIt {
            model_name: "gemma-4-e2b-it",
            constructor_name: gemma_4_e2b_it,
            display_name: "Gemma 4 E2B Instruct",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Gemma4E4bIt {
            model_name: "gemma-4-e4b-it",
            constructor_name: gemma_4_e4b_it,
            display_name: "Gemma 4 E4B Instruct",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Glm4 {
            model_name: "glm-4",
            constructor_name: glm_4,
            display_name: "GLM-4",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm4Air {
            model_name: "glm-4-air",
            constructor_name: glm_4_air,
            display_name: "GLM-4 Air",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm4Air0111 {
            model_name: "glm-4-air-0111",
            constructor_name: glm_4_air_0111,
            display_name: "GLM 4 Air 0111",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm4Airx {
            model_name: "glm-4-airx",
            constructor_name: glm_4_airx,
            display_name: "GLM-4 AirX",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm4Flash {
            model_name: "glm-4-flash",
            constructor_name: glm_4_flash,
            display_name: "GLM-4 Flash",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm4Long {
            model_name: "glm-4-long",
            constructor_name: glm_4_long,
            display_name: "GLM-4 Long",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm4Plus {
            model_name: "glm-4-plus",
            constructor_name: glm_4_plus,
            display_name: "GLM-4 Plus",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm4Plus0111 {
            model_name: "glm-4-plus-0111",
            constructor_name: glm_4_plus_0111,
            display_name: "GLM 4 Plus 0111",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm41vThinkingFlash {
            model_name: "glm-4.1v-thinking-flash",
            constructor_name: glm_4_1v_thinking_flash,
            display_name: "GLM 4.1V Thinking Flash",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Glm41vThinkingFlashx {
            model_name: "glm-4.1v-thinking-flashx",
            constructor_name: glm_4_1v_thinking_flashx,
            display_name: "GLM 4.1V Thinking FlashX",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        GlmZ1Air {
            model_name: "glm-z1-air",
            constructor_name: glm_z1_air,
            display_name: "GLM Z1 Air",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GlmZ1Airx {
            model_name: "glm-z1-airx",
            constructor_name: glm_z1_airx,
            display_name: "GLM Z1 AirX",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GlmZeroPreview {
            model_name: "glm-zero-preview",
            constructor_name: glm_zero_preview,
            display_name: "GLM Zero Preview",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleGemini3FlashPreview {
            model_name: "google/gemini-3-flash-preview",
            constructor_name: google_gemini_3_flash_preview,
            display_name: "Gemini 3 Flash (Preview)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini3FlashPreviewThinking {
            model_name: "google/gemini-3-flash-preview-thinking",
            constructor_name: google_gemini_3_flash_preview_thinking,
            display_name: "Gemini 3 Flash Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini31FlashLite {
            model_name: "google/gemini-3.1-flash-lite",
            constructor_name: google_gemini_3_1_flash_lite,
            display_name: "Gemini 3.1 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini31ProPreview {
            model_name: "google/gemini-3.1-pro-preview",
            constructor_name: google_gemini_3_1_pro_preview,
            display_name: "Gemini 3.1 Pro (Preview)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini31ProPreviewCustomtools {
            model_name: "google/gemini-3.1-pro-preview-customtools",
            constructor_name: google_gemini_3_1_pro_preview_customtools,
            display_name: "Gemini 3.1 Pro (Preview Custom Tools)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini31ProPreviewHigh {
            model_name: "google/gemini-3.1-pro-preview-high",
            constructor_name: google_gemini_3_1_pro_preview_high,
            display_name: "Gemini 3.1 Pro (Preview High)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini31ProPreviewLow {
            model_name: "google/gemini-3.1-pro-preview-low",
            constructor_name: google_gemini_3_1_pro_preview_low,
            display_name: "Gemini 3.1 Pro (Preview Low)",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini35Flash {
            model_name: "google/gemini-3.5-flash",
            constructor_name: google_gemini_3_5_flash,
            display_name: "Gemini 3.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini35FlashLite {
            model_name: "google/gemini-3.5-flash-lite",
            constructor_name: google_gemini_3_5_flash_lite,
            display_name: "Gemini 3.5 Flash Lite",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini35FlashThinking {
            model_name: "google/gemini-3.5-flash-thinking",
            constructor_name: google_gemini_3_5_flash_thinking,
            display_name: "Gemini 3.5 Flash Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini36Flash {
            model_name: "google/gemini-3.6-flash",
            constructor_name: google_gemini_3_6_flash,
            display_name: "Gemini 3.6 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGeminiFlashLatest {
            model_name: "google/gemini-flash-latest",
            constructor_name: google_gemini_flash_latest,
            display_name: "Gemini Flash Latest",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGeminiFlashLiteLatest {
            model_name: "google/gemini-flash-lite-latest",
            constructor_name: google_gemini_flash_lite_latest,
            display_name: "Gemini Flash Lite Latest",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGeminiProLatest {
            model_name: "google/gemini-pro-latest",
            constructor_name: google_gemini_pro_latest,
            display_name: "Gemini Pro Latest",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma426bA4bIt {
            model_name: "google/gemma-4-26b-a4b-it",
            constructor_name: google_gemma_4_26b_a4b_it,
            display_name: "Gemma 4 26B A4B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma426bA4bItThinking {
            model_name: "google/gemma-4-26b-a4b-it:thinking",
            constructor_name: google_gemma_4_26b_a4b_it_thinking,
            display_name: "Gemma 4 26B A4B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31b-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma 4 31B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma431bItThinking {
            model_name: "google/gemma-4-31b-it:thinking",
            constructor_name: google_gemma_4_31b_it_thinking,
            display_name: "Gemma 4 31B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        HermesHigh {
            model_name: "hermes-high",
            constructor_name: hermes_high,
            display_name: "Hermes High",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HermesLow {
            model_name: "hermes-low",
            constructor_name: hermes_low,
            display_name: "Hermes Low",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        HermesMedium {
            model_name: "hermes-medium",
            constructor_name: hermes_medium,
            display_name: "Hermes Medium",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Holo335bA3b {
            model_name: "holo3-35b-a3b",
            constructor_name: holo3_35b_a3b,
            display_name: "Holo3-35B-A3B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Holo335bA3bThinking {
            model_name: "holo3-35b-a3b:thinking",
            constructor_name: holo3_35b_a3b_thinking,
            display_name: "Holo3-35B-A3B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HuihuiAiDeepseekR1DistillLlama70bAbliterated {
            model_name: "huihui-ai/DeepSeek-R1-Distill-Llama-70B-abliterated",
            constructor_name: huihui_ai_deepseek_r1_distill_llama_70b_abliterated,
            display_name: "DeepSeek R1 Llama 70B Abliterated",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        HuihuiAiDeepseekR1DistillQwen32bAbliterated {
            model_name: "huihui-ai/DeepSeek-R1-Distill-Qwen-32B-abliterated",
            constructor_name: huihui_ai_deepseek_r1_distill_qwen_32b_abliterated,
            display_name: "DeepSeek R1 Qwen Abliterated",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        HuihuiAiLlama3370bInstructAbliterated {
            model_name: "huihui-ai/Llama-3.3-70B-Instruct-abliterated",
            constructor_name: huihui_ai_llama_3_3_70b_instruct_abliterated,
            display_name: "Llama 3.3 70B Instruct abliterated",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        HuihuiAiQwen2532bInstructAbliterated {
            model_name: "huihui-ai/Qwen2.5-32B-Instruct-abliterated",
            constructor_name: huihui_ai_qwen2_5_32b_instruct_abliterated,
            display_name: "Qwen 2.5 32B Abliterated",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        HunyuanTurbos20250226 {
            model_name: "hunyuan-turbos-20250226",
            constructor_name: hunyuan_turbos_20250226,
            display_name: "Hunyuan Turbo S",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        IbmGraniteGranite418b {
            model_name: "ibm-granite/granite-4.1-8b",
            constructor_name: ibm_granite_granite_4_1_8b,
            display_name: "Granite 4.1 8B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing261t {
            model_name: "inclusionai/ling-2.6-1t",
            constructor_name: inclusionai_ling_2_6_1t,
            display_name: "Ling 2.6 1T",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing26Flash {
            model_name: "inclusionai/ling-2.6-flash",
            constructor_name: inclusionai_ling_2_6_flash,
            display_name: "Ling 2.6 Flash",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing30Flash {
            model_name: "inclusionai/ling-3.0-flash",
            constructor_name: inclusionai_ling_3_0_flash,
            display_name: "Ling 3.0 Flash",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing30FlashThinking {
            model_name: "inclusionai/ling-3.0-flash:thinking",
            constructor_name: inclusionai_ling_3_0_flash_thinking,
            display_name: "Ling 3.0 Flash Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiRing261t {
            model_name: "inclusionai/ring-2.6-1t",
            constructor_name: inclusionai_ring_2_6_1t,
            display_name: "Ring 2.6 1T",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InflatebotMn12bMagMellR1 {
            model_name: "inflatebot/MN-12B-Mag-Mell-R1",
            constructor_name: inflatebot_mn_12b_mag_mell_r1,
            display_name: "Mag Mell R1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        InflectionInflection3Pi {
            model_name: "inflection/inflection-3-pi",
            constructor_name: inflection_inflection_3_pi,
            display_name: "Inflection 3 Pi",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        InflectionInflection3Productivity {
            model_name: "inflection/inflection-3-productivity",
            constructor_name: inflection_inflection_3_productivity,
            display_name: "Inflection 3 Productivity",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        JambaLarge {
            model_name: "jamba-large",
            constructor_name: jamba_large,
            display_name: "Jamba Large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        JambaLarge16 {
            model_name: "jamba-large-1.6",
            constructor_name: jamba_large_1_6,
            display_name: "Jamba Large 1.6",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        JambaLarge17 {
            model_name: "jamba-large-1.7",
            constructor_name: jamba_large_1_7,
            display_name: "Jamba Large 1.7",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        JambaMini {
            model_name: "jamba-mini",
            constructor_name: jamba_mini,
            display_name: "Jamba Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        JambaMini16 {
            model_name: "jamba-mini-1.6",
            constructor_name: jamba_mini_1_6,
            display_name: "Jamba Mini 1.6",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        JambaMini17 {
            model_name: "jamba-mini-1.7",
            constructor_name: jamba_mini_1_7,
            display_name: "Jamba Mini 1.7",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        KimiK2InstructFast {
            model_name: "kimi-k2-instruct-fast",
            constructor_name: kimi_k2_instruct_fast,
            display_name: "Kimi K2 0711 Fast",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        KwaipilotKatCoderAirV25 {
            model_name: "kwaipilot/kat-coder-air-v2.5",
            constructor_name: kwaipilot_kat_coder_air_v2_5,
            display_name: "KAT Coder Air V2.5",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KwaipilotKatCoderProV2 {
            model_name: "kwaipilot/kat-coder-pro-v2",
            constructor_name: kwaipilot_kat_coder_pro_v2,
            display_name: "KAT Coder Pro V2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        KwaipilotKatCoderProV25 {
            model_name: "kwaipilot/kat-coder-pro-v2.5",
            constructor_name: kwaipilot_kat_coder_pro_v2_5,
            display_name: "KAT Coder Pro V2.5",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Learnlm15ProExperimental {
            model_name: "learnlm-1.5-pro-experimental",
            constructor_name: learnlm_1_5_pro_experimental,
            display_name: "Gemini LearnLM Experimental",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Longcat20 {
            model_name: "longcat-2.0",
            constructor_name: longcat_2_0,
            display_name: "LongCat 2.0",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Longcat20Thinking {
            model_name: "longcat-2.0:thinking",
            constructor_name: longcat_2_0_thinking,
            display_name: "LongCat 2.0 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MeganovaAiMantaFlash10 {
            model_name: "meganova-ai/manta-flash-1.0",
            constructor_name: meganova_ai_manta_flash_1_0,
            display_name: "Manta Flash 1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MeganovaAiMantaMini10 {
            model_name: "meganova-ai/manta-mini-1.0",
            constructor_name: meganova_ai_manta_mini_1_0,
            display_name: "Manta Mini 1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MeganovaAiMantaPro10 {
            model_name: "meganova-ai/manta-pro-1.0",
            constructor_name: meganova_ai_manta_pro_1_0,
            display_name: "Manta Pro 1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Mercury2 {
            model_name: "mercury-2",
            constructor_name: mercury_2,
            display_name: "Mercury 2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MercuryCoderSmall {
            model_name: "mercury-coder-small",
            constructor_name: mercury_coder_small,
            display_name: "Mercury Coder Small",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama318bInstruct {
            model_name: "meta-llama/llama-3.1-8b-instruct",
            constructor_name: meta_llama_llama_3_1_8b_instruct,
            display_name: "Llama 3.1 8b Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama323bInstruct {
            model_name: "meta-llama/llama-3.2-3b-instruct",
            constructor_name: meta_llama_llama_3_2_3b_instruct,
            display_name: "Llama 3.2 3b Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama3370bInstruct {
            model_name: "meta-llama/llama-3.3-70b-instruct",
            constructor_name: meta_llama_llama_3_3_70b_instruct,
            display_name: "Llama 3.3 70b Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama4Maverick {
            model_name: "meta-llama/llama-4-maverick",
            constructor_name: meta_llama_llama_4_maverick,
            display_name: "Llama 4 Maverick",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        MetaLlamaLlama4Scout {
            model_name: "meta-llama/llama-4-scout",
            constructor_name: meta_llama_llama_4_scout,
            display_name: "Llama 4 Scout",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseSpark11 {
            model_name: "meta/muse-spark-1.1",
            constructor_name: meta_muse_spark_1_1,
            display_name: "Muse Spark 1.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        MicrosoftWizardlm28x22b {
            model_name: "microsoft/wizardlm-2-8x22b",
            constructor_name: microsoft_wizardlm_2_8x22b,
            display_name: "WizardLM-2 8x22B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MinimaxMinimax01 {
            model_name: "minimax/minimax-01",
            constructor_name: minimax_minimax_01,
            display_name: "MiniMax 01",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MinimaxMinimaxLatest {
            model_name: "minimax/minimax-latest",
            constructor_name: minimax_minimax_latest,
            display_name: "MiniMax Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM2Her {
            model_name: "minimax/minimax-m2-her",
            constructor_name: minimax_minimax_m2_her,
            display_name: "MiniMax M2-her",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MinimaxMinimaxM21 {
            model_name: "minimax/minimax-m2.1",
            constructor_name: minimax_minimax_m2_1,
            display_name: "MiniMax M2.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM25 {
            model_name: "minimax/minimax-m2.5",
            constructor_name: minimax_minimax_m2_5,
            display_name: "MiniMax M2.5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM27 {
            model_name: "minimax/minimax-m2.7",
            constructor_name: minimax_minimax_m2_7,
            display_name: "MiniMax M2.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM27Turbo {
            model_name: "minimax/minimax-m2.7-turbo",
            constructor_name: minimax_minimax_m2_7_turbo,
            display_name: "MiniMax M2.7 Turbo",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM3 {
            model_name: "minimax/minimax-m3",
            constructor_name: minimax_minimax_m3,
            display_name: "MiniMax M3",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM3Thinking {
            model_name: "minimax/minimax-m3:thinking",
            constructor_name: minimax_minimax_m3_thinking,
            display_name: "MiniMax M3 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Mirothinker17Deepresearch {
            model_name: "mirothinker-1-7-deepresearch",
            constructor_name: mirothinker_1_7_deepresearch,
            display_name: "MiroThinker 1.7 Deep Research",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Mirothinker17DeepresearchMini {
            model_name: "mirothinker-1-7-deepresearch-mini",
            constructor_name: mirothinker_1_7_deepresearch_mini,
            display_name: "MiroThinker 1.7 Deep Research Mini",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        MistralCodeAgentLatest {
            model_name: "mistral-code-agent-latest",
            constructor_name: mistral_code_agent_latest,
            display_name: "Mistral Code Agent Latest",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralCodeLatest {
            model_name: "mistral-code-latest",
            constructor_name: mistral_code_latest,
            display_name: "Mistral Code Latest",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralSmall3124bInstruct {
            model_name: "mistral-small-31-24b-instruct",
            constructor_name: mistral_small_31_24b_instruct,
            display_name: "Mistral Small 31 24b Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralMistralMedium35 {
            model_name: "mistral/mistral-medium-3.5",
            constructor_name: mistral_mistral_medium_3_5,
            display_name: "Mistral Medium 3.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistralMedium35Thinking {
            model_name: "mistral/mistral-medium-3.5:thinking",
            constructor_name: mistral_mistral_medium_3_5_thinking,
            display_name: "Mistral Medium 3.5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiDevstralSmall2505 {
            model_name: "mistralai/Devstral-Small-2505",
            constructor_name: mistralai_devstral_small_2505,
            display_name: "Mistral Devstral Small 2505",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralNemoInstruct2407 {
            model_name: "mistralai/Mistral-Nemo-Instruct-2407",
            constructor_name: mistralai_mistral_nemo_instruct_2407,
            display_name: "Mistral Nemo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiCodestral2508 {
            model_name: "mistralai/codestral-2508",
            constructor_name: mistralai_codestral_2508,
            display_name: "Codestral 2508",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiDevstral2123bInstruct2512 {
            model_name: "mistralai/devstral-2-123b-instruct-2512",
            constructor_name: mistralai_devstral_2_123b_instruct_2512,
            display_name: "Devstral 2 123B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMinistral14b2512 {
            model_name: "mistralai/ministral-14b-2512",
            constructor_name: mistralai_ministral_14b_2512,
            display_name: "Ministral 14B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMinistral14bInstruct2512 {
            model_name: "mistralai/ministral-14b-instruct-2512",
            constructor_name: mistralai_ministral_14b_instruct_2512,
            display_name: "Ministral 3 14B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralaiMinistral3b2512 {
            model_name: "mistralai/ministral-3b-2512",
            constructor_name: mistralai_ministral_3b_2512,
            display_name: "Ministral 3B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMinistral8b2512 {
            model_name: "mistralai/ministral-8b-2512",
            constructor_name: mistralai_ministral_8b_2512,
            display_name: "Ministral 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralLarge {
            model_name: "mistralai/mistral-large",
            constructor_name: mistralai_mistral_large,
            display_name: "Mistral Large 2411",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralLarge3675bInstruct2512 {
            model_name: "mistralai/mistral-large-3-675b-instruct-2512",
            constructor_name: mistralai_mistral_large_3_675b_instruct_2512,
            display_name: "Mistral Large 3 675B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralMedium3 {
            model_name: "mistralai/mistral-medium-3",
            constructor_name: mistralai_mistral_medium_3,
            display_name: "Mistral Medium 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralMedium31 {
            model_name: "mistralai/mistral-medium-3.1",
            constructor_name: mistralai_mistral_medium_3_1,
            display_name: "Mistral Medium 3.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralSaba {
            model_name: "mistralai/mistral-saba",
            constructor_name: mistralai_mistral_saba,
            display_name: "Mistral Saba",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMistralSmall4119b2603 {
            model_name: "mistralai/mistral-small-4-119b-2603",
            constructor_name: mistralai_mistral_small_4_119b_2603,
            display_name: "Mistral Small 4 119B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralSmall4119b2603Thinking {
            model_name: "mistralai/mistral-small-4-119b-2603:thinking",
            constructor_name: mistralai_mistral_small_4_119b_2603_thinking,
            display_name: "Mistral Small 4 119B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMixtral8x22bInstructV01 {
            model_name: "mistralai/mixtral-8x22b-instruct-v0.1",
            constructor_name: mistralai_mixtral_8x22b_instruct_v0_1,
            display_name: "Mixtral 8x22B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MlabonneNeuraldaredevil8bAbliterated {
            model_name: "mlabonne/NeuralDaredevil-8B-abliterated",
            constructor_name: mlabonne_neuraldaredevil_8b_abliterated,
            display_name: "Neural Daredevil 8B abliterated",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MoonshotaiKimiK2Instruct0905 {
            model_name: "moonshotai/Kimi-K2-Instruct-0905",
            constructor_name: moonshotai_kimi_k2_instruct_0905,
            display_name: "Kimi K2 0905",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct {
            model_name: "moonshotai/kimi-k2-instruct",
            constructor_name: moonshotai_kimi_k2_instruct,
            display_name: "Kimi K2 Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Instruct0711 {
            model_name: "moonshotai/kimi-k2-instruct-0711",
            constructor_name: moonshotai_kimi_k2_instruct_0711,
            display_name: "Kimi K2 0711",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/kimi-k2-thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai/kimi-k2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25Thinking {
            model_name: "moonshotai/kimi-k2.5:thinking",
            constructor_name: moonshotai_kimi_k2_5_thinking,
            display_name: "Kimi K2.5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK26 {
            model_name: "moonshotai/kimi-k2.6",
            constructor_name: moonshotai_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK26Thinking {
            model_name: "moonshotai/kimi-k2.6:thinking",
            constructor_name: moonshotai_kimi_k2_6_thinking,
            display_name: "Kimi K2.6 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK27Code {
            model_name: "moonshotai/kimi-k2.7-code",
            constructor_name: moonshotai_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK27CodeHighspeed {
            model_name: "moonshotai/kimi-k2.7-code-highspeed",
            constructor_name: moonshotai_kimi_k2_7_code_highspeed,
            display_name: "Kimi K2.7 Code High-Speed",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK3 {
            model_name: "moonshotai/kimi-k3",
            constructor_name: moonshotai_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MoonshotaiKimiLatest {
            model_name: "moonshotai/kimi-latest",
            constructor_name: moonshotai_kimi_latest,
            display_name: "Kimi Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NanoGptHelp {
            model_name: "nano-gpt-help",
            constructor_name: nano_gpt_help,
            display_name: "NanoGPT Help",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NanogptCodingRouter {
            model_name: "nanogpt/coding-router",
            constructor_name: nanogpt_coding_router,
            display_name: "Coding Router",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NanogptCodingRouterHigh {
            model_name: "nanogpt/coding-router:high",
            constructor_name: nanogpt_coding_router_high,
            display_name: "Coding Router High",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NanogptCodingRouterLow {
            model_name: "nanogpt/coding-router:low",
            constructor_name: nanogpt_coding_router_low,
            display_name: "Coding Router Low",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NanogptCodingRouterMax {
            model_name: "nanogpt/coding-router:max",
            constructor_name: nanogpt_coding_router_max,
            display_name: "Coding Router Max",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NanogptCodingRouterMedium {
            model_name: "nanogpt/coding-router:medium",
            constructor_name: nanogpt_coding_router_medium,
            display_name: "Coding Router Medium",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NexAgiNexN2Mini {
            model_name: "nex-agi/nex-n2-mini",
            constructor_name: nex_agi_nex_n2_mini,
            display_name: "Nex N2 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NexAgiNexN2Pro {
            model_name: "nex-agi/nex-n2-pro",
            constructor_name: nex_agi_nex_n2_pro,
            display_name: "Nex N2 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NothingiisrealL3170bCelesteV01Bf16 {
            model_name: "nothingiisreal/L3.1-70B-Celeste-V0.1-BF16",
            constructor_name: nothingiisreal_l3_1_70b_celeste_v0_1_bf16,
            display_name: "Llama 3.1 70B Celeste v0.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlama31Nemotron70bInstructHf {
            model_name: "nvidia/Llama-3.1-Nemotron-70B-Instruct-HF",
            constructor_name: nvidia_llama_3_1_nemotron_70b_instruct_hf,
            display_name: "Nvidia Nemotron 70b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlama33NemotronSuper49bV1 {
            model_name: "nvidia/Llama-3.3-Nemotron-Super-49B-v1",
            constructor_name: nvidia_llama_3_3_nemotron_super_49b_v1,
            display_name: "Nvidia Nemotron Super 49B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3Nano30bA3b {
            model_name: "nvidia/nemotron-3-nano-30b-a3b",
            constructor_name: nvidia_nemotron_3_nano_30b_a3b,
            display_name: "Nvidia Nemotron 3 Nano 30B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3NanoOmni30bA3bReasoning {
            model_name: "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning",
            constructor_name: nvidia_nemotron_3_nano_omni_30b_a3b_reasoning,
            display_name: "Nvidia Nemotron 3 Nano Omni",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3Super120bA12b {
            model_name: "nvidia/nemotron-3-super-120b-a12b",
            constructor_name: nvidia_nemotron_3_super_120b_a12b,
            display_name: "Nvidia Nemotron 3 Super 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3Super120bA12bThinking {
            model_name: "nvidia/nemotron-3-super-120b-a12b:thinking",
            constructor_name: nvidia_nemotron_3_super_120b_a12b_thinking,
            display_name: "Nvidia Nemotron 3 Super 120B Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3Ultra550bA55b {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b,
            display_name: "Nvidia Nemotron 3 Ultra 550B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotron3Ultra550bA55bThinking {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b:thinking",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b_thinking,
            display_name: "Nvidia Nemotron 3 Ultra 550B Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt35Turbo {
            model_name: "openai/gpt-3.5-turbo",
            constructor_name: openai_gpt_3_5_turbo,
            display_name: "GPT-3.5 Turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt4Turbo {
            model_name: "openai/gpt-4-turbo",
            constructor_name: openai_gpt_4_turbo,
            display_name: "GPT-4 Turbo",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt4TurboPreview {
            model_name: "openai/gpt-4-turbo-preview",
            constructor_name: openai_gpt_4_turbo_preview,
            display_name: "GPT-4 Turbo Preview",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt41 {
            model_name: "openai/gpt-4.1",
            constructor_name: openai_gpt_4_1,
            display_name: "GPT 4.1",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Mini {
            model_name: "openai/gpt-4.1-mini",
            constructor_name: openai_gpt_4_1_mini,
            display_name: "GPT 4.1 Mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt41Nano {
            model_name: "openai/gpt-4.1-nano",
            constructor_name: openai_gpt_4_1_nano,
            display_name: "GPT 4.1 Nano",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt4o {
            model_name: "openai/gpt-4o",
            constructor_name: openai_gpt_4o,
            display_name: "GPT-4o",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt4o20240806 {
            model_name: "openai/gpt-4o-2024-08-06",
            constructor_name: openai_gpt_4o_2024_08_06,
            display_name: "GPT-4o (2024-08-06)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt4o20241120 {
            model_name: "openai/gpt-4o-2024-11-20",
            constructor_name: openai_gpt_4o_2024_11_20,
            display_name: "GPT-4o (2024-11-20)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt4oMini {
            model_name: "openai/gpt-4o-mini",
            constructor_name: openai_gpt_4o_mini,
            display_name: "GPT-4o mini",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt4oMiniSearchPreview {
            model_name: "openai/gpt-4o-mini-search-preview",
            constructor_name: openai_gpt_4o_mini_search_preview,
            display_name: "GPT-4o mini Search Preview",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt4oSearchPreview {
            model_name: "openai/gpt-4o-search-preview",
            constructor_name: openai_gpt_4o_search_preview,
            display_name: "GPT-4o Search Preview",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt5 {
            model_name: "openai/gpt-5",
            constructor_name: openai_gpt_5,
            display_name: "GPT 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Codex {
            model_name: "openai/gpt-5-codex",
            constructor_name: openai_gpt_5_codex,
            display_name: "GPT-5 Codex",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Mini {
            model_name: "openai/gpt-5-mini",
            constructor_name: openai_gpt_5_mini,
            display_name: "GPT 5 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt5Nano {
            model_name: "openai/gpt-5-nano",
            constructor_name: openai_gpt_5_nano,
            display_name: "GPT 5 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt5Pro {
            model_name: "openai/gpt-5-pro",
            constructor_name: openai_gpt_5_pro,
            display_name: "GPT 5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt51 {
            model_name: "openai/gpt-5.1",
            constructor_name: openai_gpt_5_1,
            display_name: "GPT 5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5120251113 {
            model_name: "openai/gpt-5.1-2025-11-13",
            constructor_name: openai_gpt_5_1_2025_11_13,
            display_name: "GPT-5.1 (2025-11-13)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Codex {
            model_name: "openai/gpt-5.1-codex",
            constructor_name: openai_gpt_5_1_codex,
            display_name: "GPT 5.1 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt51CodexMax {
            model_name: "openai/gpt-5.1-codex-max",
            constructor_name: openai_gpt_5_1_codex_max,
            display_name: "GPT 5.1 Codex Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51CodexMini {
            model_name: "openai/gpt-5.1-codex-mini",
            constructor_name: openai_gpt_5_1_codex_mini,
            display_name: "GPT 5.1 Codex Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt52 {
            model_name: "openai/gpt-5.2",
            constructor_name: openai_gpt_5_2,
            display_name: "GPT 5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Codex {
            model_name: "openai/gpt-5.2-codex",
            constructor_name: openai_gpt_5_2_codex,
            display_name: "GPT 5.2 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt53Codex {
            model_name: "openai/gpt-5.3-codex",
            constructor_name: openai_gpt_5_3_codex,
            display_name: "GPT 5.3 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54 {
            model_name: "openai/gpt-5.4",
            constructor_name: openai_gpt_5_4,
            display_name: "GPT 5.4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54Mini {
            model_name: "openai/gpt-5.4-mini",
            constructor_name: openai_gpt_5_4_mini,
            display_name: "GPT 5.4 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt54Nano {
            model_name: "openai/gpt-5.4-nano",
            constructor_name: openai_gpt_5_4_nano,
            display_name: "GPT 5.4 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt55 {
            model_name: "openai/gpt-5.5",
            constructor_name: openai_gpt_5_5,
            display_name: "GPT 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Luna {
            model_name: "openai/gpt-5.6-luna",
            constructor_name: openai_gpt_5_6_luna,
            display_name: "GPT 5.6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56LunaPro {
            model_name: "openai/gpt-5.6-luna-pro",
            constructor_name: openai_gpt_5_6_luna_pro,
            display_name: "GPT 5.6 Luna Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Sol {
            model_name: "openai/gpt-5.6-sol",
            constructor_name: openai_gpt_5_6_sol,
            display_name: "GPT 5.6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56SolPro {
            model_name: "openai/gpt-5.6-sol-pro",
            constructor_name: openai_gpt_5_6_sol_pro,
            display_name: "GPT 5.6 Sol Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Terra {
            model_name: "openai/gpt-5.6-terra",
            constructor_name: openai_gpt_5_6_terra,
            display_name: "GPT 5.6 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56TerraPro {
            model_name: "openai/gpt-5.6-terra-pro",
            constructor_name: openai_gpt_5_6_terra_pro,
            display_name: "GPT 5.6 Terra Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptChatLatest {
            model_name: "openai/gpt-chat-latest",
            constructor_name: openai_gpt_chat_latest,
            display_name: "GPT Chat Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptLatest {
            model_name: "openai/gpt-latest",
            constructor_name: openai_gpt_latest,
            display_name: "GPT Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGptOssSafeguard20b {
            model_name: "openai/gpt-oss-safeguard-20b",
            constructor_name: openai_gpt_oss_safeguard_20b,
            display_name: "GPT OSS Safeguard 20B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiO1 {
            model_name: "openai/o1",
            constructor_name: openai_o1,
            display_name: "OpenAI o1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiO1Preview {
            model_name: "openai/o1-preview",
            constructor_name: openai_o1_preview,
            display_name: "OpenAI o1-preview",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiO1Pro {
            model_name: "openai/o1-pro",
            constructor_name: openai_o1_pro,
            display_name: "OpenAI o1 Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiO3 {
            model_name: "openai/o3",
            constructor_name: openai_o3,
            display_name: "OpenAI o3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiO3DeepResearch {
            model_name: "openai/o3-deep-research",
            constructor_name: openai_o3_deep_research,
            display_name: "OpenAI o3 Deep Research",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiO3Mini {
            model_name: "openai/o3-mini",
            constructor_name: openai_o3_mini,
            display_name: "OpenAI o3-mini",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3MiniHigh {
            model_name: "openai/o3-mini-high",
            constructor_name: openai_o3_mini_high,
            display_name: "OpenAI o3-mini (High)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3MiniLow {
            model_name: "openai/o3-mini-low",
            constructor_name: openai_o3_mini_low,
            display_name: "OpenAI o3-mini (Low)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3Pro20250610 {
            model_name: "openai/o3-pro-2025-06-10",
            constructor_name: openai_o3_pro_2025_06_10,
            display_name: "OpenAI o3-pro (2025-06-10)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO4Mini {
            model_name: "openai/o4-mini",
            constructor_name: openai_o4_mini,
            display_name: "OpenAI o4-mini",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO4MiniDeepResearch {
            model_name: "openai/o4-mini-deep-research",
            constructor_name: openai_o4_mini_deep_research,
            display_name: "OpenAI o4-mini Deep Research",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiO4MiniHigh {
            model_name: "openai/o4-mini-high",
            constructor_name: openai_o4_mini_high,
            display_name: "OpenAI o4-mini high",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PamanseauOpenreasoningNemotron32b {
            model_name: "pamanseau/OpenReasoning-Nemotron-32B",
            constructor_name: pamanseau_openreasoning_nemotron_32b,
            display_name: "OpenReasoning Nemotron 32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        PerceptronPerceptronMk1 {
            model_name: "perceptron/perceptron-mk1",
            constructor_name: perceptron_perceptron_mk1,
            display_name: "Perceptron Mk1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        PerplexityAcademicResearcher {
            model_name: "perplexity-academic-researcher",
            constructor_name: perplexity_academic_researcher,
            display_name: "Perplexity Academic Researcher",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Phi4MiniInstruct {
            model_name: "phi-4-mini-instruct",
            constructor_name: phi_4_mini_instruct,
            display_name: "Phi 4 Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Phi4MultimodalInstruct {
            model_name: "phi-4-multimodal-instruct",
            constructor_name: phi_4_multimodal_instruct,
            display_name: "Phi 4 Multimodal",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        PoolsideLagunaM1 {
            model_name: "poolside/laguna-m.1",
            constructor_name: poolside_laguna_m_1,
            display_name: "Laguna M.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        PoolsideLagunaS21 {
            model_name: "poolside/laguna-s-2.1",
            constructor_name: poolside_laguna_s_2_1,
            display_name: "Laguna S 2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PoolsideLagunaS21Thinking {
            model_name: "poolside/laguna-s-2.1:thinking",
            constructor_name: poolside_laguna_s_2_1_thinking,
            display_name: "Laguna S 2.1 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QvqMax {
            model_name: "qvq-max",
            constructor_name: qvq_max,
            display_name: "Qwen: QvQ Max",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen36Plus {
            model_name: "qwen-3.6-plus",
            constructor_name: qwen_3_6_plus,
            display_name: "Qwen 3.6 Plus",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenLong {
            model_name: "qwen-long",
            constructor_name: qwen_long,
            display_name: "Qwen Long 10M",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenMax {
            model_name: "qwen-max",
            constructor_name: qwen_max,
            display_name: "Qwen 2.5 Max",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenPlus {
            model_name: "qwen-plus",
            constructor_name: qwen_plus,
            display_name: "Qwen Plus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        QwenTurbo {
            model_name: "qwen-turbo",
            constructor_name: qwen_turbo,
            display_name: "Qwen Turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen25Coder32bInstruct {
            model_name: "qwen/Qwen2.5-Coder-32B-Instruct",
            constructor_name: qwen_qwen2_5_coder_32b_instruct,
            display_name: "Qwen 2.5 Coder 32b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3235bA22bInstruct2507 {
            model_name: "qwen/Qwen3-235B-A22B-Instruct-2507",
            constructor_name: qwen_qwen3_235b_a22b_instruct_2507,
            display_name: "Qwen 3 235b A22B 2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bInstruct2507Tee {
            model_name: "qwen/Qwen3-235B-A22B-Instruct-2507-TEE",
            constructor_name: qwen_qwen3_235b_a22b_instruct_2507_tee,
            display_name: "Qwen 3 235b A22B 2507 (TEE)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22bThinking2507 {
            model_name: "qwen/Qwen3-235B-A22B-Thinking-2507",
            constructor_name: qwen_qwen3_235b_a22b_thinking_2507,
            display_name: "Qwen 3 235b A22B 2507 Thinking",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen38b {
            model_name: "qwen/Qwen3-8B",
            constructor_name: qwen_qwen3_8b,
            display_name: "Qwen 3 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3Next80bA3bInstruct {
            model_name: "qwen/Qwen3-Next-80B-A3B-Instruct",
            constructor_name: qwen_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3 Next 80B A3B (Instruct)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl235bA22bInstruct {
            model_name: "qwen/Qwen3-VL-235B-A22B-Instruct",
            constructor_name: qwen_qwen3_vl_235b_a22b_instruct,
            display_name: "Qwen3 VL 235B A22B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen3635bA3b {
            model_name: "qwen/Qwen3.6-35B-A3B",
            constructor_name: qwen_qwen3_6_35b_a3b,
            display_name: "Qwen3.6 35B A3B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen3635bA3bThinking {
            model_name: "qwen/Qwen3.6-35B-A3B:thinking",
            constructor_name: qwen_qwen3_6_35b_a3b_thinking,
            display_name: "Qwen3.6 35B A3B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen2572bInstruct {
            model_name: "qwen/qwen-2.5-72b-instruct",
            constructor_name: qwen_qwen_2_5_72b_instruct,
            display_name: "Qwen2.5 72B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen314b {
            model_name: "qwen/qwen3-14b",
            constructor_name: qwen_qwen3_14b,
            display_name: "Qwen 3 14b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3235bA22b {
            model_name: "qwen/qwen3-235b-a22b",
            constructor_name: qwen_qwen3_235b_a22b,
            display_name: "Qwen 3 235b A22B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen330bA3b {
            model_name: "qwen/qwen3-30b-a3b",
            constructor_name: qwen_qwen3_30b_a3b,
            display_name: "Qwen3 30B A3B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen332b {
            model_name: "qwen/qwen3-32b",
            constructor_name: qwen_qwen3_32b,
            display_name: "Qwen 3 32b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen3Coder {
            model_name: "qwen/qwen3-coder",
            constructor_name: qwen_qwen3_coder,
            display_name: "Qwen 3 Coder 480B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderFlash {
            model_name: "qwen/qwen3-coder-flash",
            constructor_name: qwen_qwen3_coder_flash,
            display_name: "Qwen3 Coder Flash",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3CoderNext {
            model_name: "qwen/qwen3-coder-next",
            constructor_name: qwen_qwen3_coder_next,
            display_name: "Qwen3 Coder Next",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderPlus {
            model_name: "qwen/qwen3-coder-plus",
            constructor_name: qwen_qwen3_coder_plus,
            display_name: "Qwen3 Coder Plus",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3Max {
            model_name: "qwen/qwen3-max",
            constructor_name: qwen_qwen3_max,
            display_name: "Qwen3 Max",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen3Next80bA3bThinking {
            model_name: "qwen/qwen3-next-80b-a3b-thinking",
            constructor_name: qwen_qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3 Next 80B A3B (Thinking)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        QwenQwen35397bA17b {
            model_name: "qwen/qwen3.5-397b-a17b",
            constructor_name: qwen_qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B A17B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen35397bA17bThinking {
            model_name: "qwen/qwen3.5-397b-a17b-thinking",
            constructor_name: qwen_qwen3_5_397b_a17b_thinking,
            display_name: "Qwen3.5 397B A17B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen359b {
            model_name: "qwen/qwen3.5-9b",
            constructor_name: qwen_qwen3_5_9b,
            display_name: "Qwen3.5 9B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        QwenQwen35Plus {
            model_name: "qwen/qwen3.5-plus",
            constructor_name: qwen_qwen3_5_plus,
            display_name: "Qwen3.5 Plus",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        QwenQwen35PlusThinking {
            model_name: "qwen/qwen3.5-plus-thinking",
            constructor_name: qwen_qwen3_5_plus_thinking,
            display_name: "Qwen3.5 Plus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen25Vl72bInstruct {
            model_name: "qwen25-vl-72b-instruct",
            constructor_name: qwen25_vl_72b_instruct,
            display_name: "Qwen25 VL 72b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen330bA3bInstruct2507 {
            model_name: "qwen3-30b-a3b-instruct-2507",
            constructor_name: qwen3_30b_a3b_instruct_2507,
            display_name: "Qwen3 30B A3B Instruct 2507",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwen3Coder30bA3bInstruct {
            model_name: "qwen3-coder-30b-a3b-instruct",
            constructor_name: qwen3_coder_30b_a3b_instruct,
            display_name: "Qwen3 Coder 30B A3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Max20260123 {
            model_name: "qwen3-max-2026-01-23",
            constructor_name: qwen3_max_2026_01_23,
            display_name: "Qwen3 Max 2026-01-23",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwen3Vl235bA22bInstructOriginal {
            model_name: "qwen3-vl-235b-a22b-instruct-original",
            constructor_name: qwen3_vl_235b_a22b_instruct_original,
            display_name: "Qwen3 VL 235B A22B Instruct Original",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen3Vl235bA22bThinking {
            model_name: "qwen3-vl-235b-a22b-thinking",
            constructor_name: qwen3_vl_235b_a22b_thinking,
            display_name: "Qwen3 VL 235B A22B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen35122bA10b {
            model_name: "qwen3.5-122b-a10b",
            constructor_name: qwen3_5_122b_a10b,
            display_name: "Qwen3.5 122B A10B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen35122bA10bThinking {
            model_name: "qwen3.5-122b-a10b:thinking",
            constructor_name: qwen3_5_122b_a10b_thinking,
            display_name: "Qwen3.5 122B A10B Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3527b {
            model_name: "qwen3.5-27b",
            constructor_name: qwen3_5_27b,
            display_name: "Qwen3.5 27B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen3527bThinking {
            model_name: "qwen3.5-27b:thinking",
            constructor_name: qwen3_5_27b_thinking,
            display_name: "Qwen3.5 27B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen3535bA3b {
            model_name: "qwen3.5-35b-a3b",
            constructor_name: qwen3_5_35b_a3b,
            display_name: "Qwen3.5 35B A3B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen3535bA3bThinking {
            model_name: "qwen3.5-35b-a3b:thinking",
            constructor_name: qwen3_5_35b_a3b_thinking,
            display_name: "Qwen3.5 35B A3B Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen35Flash {
            model_name: "qwen3.5-flash",
            constructor_name: qwen3_5_flash,
            display_name: "Qwen3.5 Flash",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen35FlashThinking {
            model_name: "qwen3.5-flash:thinking",
            constructor_name: qwen3_5_flash_thinking,
            display_name: "Qwen3.5 Flash Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen35OmniFlash {
            model_name: "qwen3.5-omni-flash",
            constructor_name: qwen3_5_omni_flash,
            display_name: "Qwen3.5 Omni Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen35OmniPlus {
            model_name: "qwen3.5-omni-plus",
            constructor_name: qwen3_5_omni_plus,
            display_name: "Qwen3.5 Omni Plus",
            capabilities: [AudioInputSupport, ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen36MaxPreview {
            model_name: "qwen3.6-max-preview",
            constructor_name: qwen3_6_max_preview,
            display_name: "Qwen3.6 Max Preview",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwen37Flash {
            model_name: "qwen3.7-flash",
            constructor_name: qwen3_7_flash,
            display_name: "Qwen3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen37FlashThinking {
            model_name: "qwen3.7-flash:thinking",
            constructor_name: qwen3_7_flash_thinking,
            display_name: "Qwen3.7 Flash Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen37Max {
            model_name: "qwen3.7-max",
            constructor_name: qwen3_7_max,
            display_name: "Qwen3.7 Max",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen37MaxThinking {
            model_name: "qwen3.7-max:thinking",
            constructor_name: qwen3_7_max_thinking,
            display_name: "Qwen3.7 Max Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Qwen37Plus {
            model_name: "qwen3.7-plus",
            constructor_name: qwen3_7_plus,
            display_name: "Qwen3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen37PlusThinking {
            model_name: "qwen3.7-plus:thinking",
            constructor_name: qwen3_7_plus_thinking,
            display_name: "Qwen3.7 Plus Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        Qwen38Max {
            model_name: "qwen3.8-max",
            constructor_name: qwen3_8_max,
            display_name: "Qwen3.8 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen38MaxPreview {
            model_name: "qwen3.8-max-preview",
            constructor_name: qwen3_8_max_preview,
            display_name: "Qwen3.8 Max Preview",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SakanaFuguUltra {
            model_name: "sakana/fugu-ultra",
            constructor_name: sakana_fugu_ultra,
            display_name: "Fugu Ultra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        SakanaFuguUltraV11 {
            model_name: "sakana/fugu-ultra-v1.1",
            constructor_name: sakana_fugu_ultra_v1_1,
            display_name: "Fugu Ultra v1.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Sarvam105b {
            model_name: "sarvam-105b",
            constructor_name: sarvam_105b,
            display_name: "Sarvam 105B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Sarvam30b {
            model_name: "sarvam-30b",
            constructor_name: sarvam_30b,
            display_name: "Sarvam 30B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ShisaAiShisaV2Llama3370b {
            model_name: "shisa-ai/shisa-v2-llama3.3-70b",
            constructor_name: shisa_ai_shisa_v2_llama3_3_70b,
            display_name: "Shisa V2 Llama 3.3 70B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ShisaAiShisaV21Llama3370b {
            model_name: "shisa-ai/shisa-v2.1-llama3.3-70b",
            constructor_name: shisa_ai_shisa_v2_1_llama3_3_70b,
            display_name: "Shisa V2.1 Llama 3.3 70B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Sonar {
            model_name: "sonar",
            constructor_name: sonar,
            display_name: "Perplexity Simple",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SonarDeepResearch {
            model_name: "sonar-deep-research",
            constructor_name: sonar_deep_research,
            display_name: "Perplexity Deep Research",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SonarPro {
            model_name: "sonar-pro",
            constructor_name: sonar_pro,
            display_name: "Perplexity Pro",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        SonarReasoningPro {
            model_name: "sonar-reasoning-pro",
            constructor_name: sonar_reasoning_pro,
            display_name: "Perplexity Reasoning Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Soob3123GraylineQwen38b {
            model_name: "soob3123/GrayLine-Qwen3-8B",
            constructor_name: soob3123_grayline_qwen3_8b,
            display_name: "Grayline Qwen3 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Soob3123VeiledCalla12b {
            model_name: "soob3123/Veiled-Calla-12B",
            constructor_name: soob3123_veiled_calla_12b,
            display_name: "Veiled Calla 12B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Soob3123AmoralGemma327bV2 {
            model_name: "soob3123/amoral-gemma3-27B-v2",
            constructor_name: soob3123_amoral_gemma3_27b_v2,
            display_name: "Amoral Gemma3 27B v2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Step216kExp {
            model_name: "step-2-16k-exp",
            constructor_name: step_2_16k_exp,
            display_name: "Step-2 16k Exp",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Step2Mini {
            model_name: "step-2-mini",
            constructor_name: step_2_mini,
            display_name: "Step-2 Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Step3 {
            model_name: "step-3",
            constructor_name: step_3,
            display_name: "Step-3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        StepR1VMini {
            model_name: "step-r1-v-mini",
            constructor_name: step_r1_v_mini,
            display_name: "Step R1 V Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        StepfunAiStep35Flash {
            model_name: "stepfun-ai/step-3.5-flash",
            constructor_name: stepfun_ai_step_3_5_flash,
            display_name: "Step 3.5 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        StepfunAiStep35Flash2603 {
            model_name: "stepfun-ai/step-3.5-flash-2603",
            constructor_name: stepfun_ai_step_3_5_flash_2603,
            display_name: "Step 3.5 Flash 2603",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        StepfunStep37FlashThinking {
            model_name: "stepfun/step-3.7-flash:thinking",
            constructor_name: stepfun_step_3_7_flash_thinking,
            display_name: "Step 3.7 Flash Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        TencentHunyuanMt7b {
            model_name: "tencent/Hunyuan-MT-7B",
            constructor_name: tencent_hunyuan_mt_7b,
            display_name: "Hunyuan MT 7B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TencentHy3 {
            model_name: "tencent/hy3",
            constructor_name: tencent_hy3,
            display_name: "Tencent Hy3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInklingSmall {
            model_name: "thinkingmachines/Inkling-Small",
            constructor_name: thinkingmachines_inkling_small,
            display_name: "Inkling Small",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInklingSmallThinking {
            model_name: "thinkingmachines/Inkling-Small:thinking",
            constructor_name: thinkingmachines_inkling_small_thinking,
            display_name: "Inkling Small Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInkling {
            model_name: "thinkingmachines/inkling",
            constructor_name: thinkingmachines_inkling,
            display_name: "Inkling",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInklingThinking {
            model_name: "thinkingmachines/inkling:thinking",
            constructor_name: thinkingmachines_inkling_thinking,
            display_name: "Inkling Thinking",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Undi95RemmSlerpL213b {
            model_name: "undi95/remm-slerp-l2-13b",
            constructor_name: undi95_remm_slerp_l2_13b,
            display_name: "ReMM SLERP 13B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        UniversalSummarizer {
            model_name: "universal-summarizer",
            constructor_name: universal_summarizer,
            display_name: "Universal Summarizer",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        UnslothGemma312bIt {
            model_name: "unsloth/gemma-3-12b-it",
            constructor_name: unsloth_gemma_3_12b_it,
            display_name: "Gemma 3 12B IT",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        UnslothGemma327bIt {
            model_name: "unsloth/gemma-3-27b-it",
            constructor_name: unsloth_gemma_3_27b_it,
            display_name: "Gemma 3 27B IT",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        UnslothGemma34bIt {
            model_name: "unsloth/gemma-3-4b-it",
            constructor_name: unsloth_gemma_3_4b_it,
            display_name: "Gemma 3 4B IT",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        UpstageSolarPro3 {
            model_name: "upstage/solar-pro-3",
            constructor_name: upstage_solar_pro_3,
            display_name: "Solar Pro 3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VeniceUncensored {
            model_name: "venice-uncensored",
            constructor_name: venice_uncensored,
            display_name: "Venice Uncensored",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        XAiGrok420 {
            model_name: "x-ai/grok-4.20",
            constructor_name: x_ai_grok_4_20,
            display_name: "Grok 4.20",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok420MultiAgent {
            model_name: "x-ai/grok-4.20-multi-agent",
            constructor_name: x_ai_grok_4_20_multi_agent,
            display_name: "Grok 4.20 Multi-Agent",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        XAiGrok43 {
            model_name: "x-ai/grok-4.3",
            constructor_name: x_ai_grok_4_3,
            display_name: "Grok 4.3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok45 {
            model_name: "x-ai/grok-4.5",
            constructor_name: x_ai_grok_4_5,
            display_name: "Grok 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrokBuild01 {
            model_name: "x-ai/grok-build-0.1",
            constructor_name: x_ai_grok_build_0_1,
            display_name: "Grok Build 0.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrokLatest {
            model_name: "x-ai/grok-latest",
            constructor_name: x_ai_grok_latest,
            display_name: "Grok Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV25 {
            model_name: "xiaomi/mimo-v2.5",
            constructor_name: xiaomi_mimo_v2_5,
            display_name: "MiMo V2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XiaomiMimoV25Pro {
            model_name: "xiaomi/mimo-v2.5-pro",
            constructor_name: xiaomi_mimo_v2_5_pro,
            display_name: "MiMo V2.5 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV25ProCrof {
            model_name: "xiaomi/mimo-v2.5-pro-crof",
            constructor_name: xiaomi_mimo_v2_5_pro_crof,
            display_name: "MiMo V2.5 Pro (Crof)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV25ProCrofThinking {
            model_name: "xiaomi/mimo-v2.5-pro-crof:thinking",
            constructor_name: xiaomi_mimo_v2_5_pro_crof_thinking,
            display_name: "MiMo V2.5 Pro Thinking (Crof)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV25ProThinking {
            model_name: "xiaomi/mimo-v2.5-pro:thinking",
            constructor_name: xiaomi_mimo_v2_5_pro_thinking,
            display_name: "MiMo V2.5 Pro Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV25Thinking {
            model_name: "xiaomi/mimo-v2.5:thinking",
            constructor_name: xiaomi_mimo_v2_5_thinking,
            display_name: "MiMo V2.5 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        YiLarge {
            model_name: "yi-large",
            constructor_name: yi_large,
            display_name: "Yi Large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        YiLightning {
            model_name: "yi-lightning",
            constructor_name: yi_lightning,
            display_name: "Yi Lightning",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        YiMedium200k {
            model_name: "yi-medium-200k",
            constructor_name: yi_medium_200k,
            display_name: "Yi Medium 200k",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ZAiGlm45v {
            model_name: "z-ai/glm-4.5v",
            constructor_name: z_ai_glm_4_5v,
            display_name: "GLM 4.5V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ZAiGlm45vThinking {
            model_name: "z-ai/glm-4.5v:thinking",
            constructor_name: z_ai_glm_4_5v_thinking,
            display_name: "GLM 4.5V Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ZAiGlm46 {
            model_name: "z-ai/glm-4.6",
            constructor_name: z_ai_glm_4_6,
            display_name: "GLM 4.6",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm46Thinking {
            model_name: "z-ai/glm-4.6:thinking",
            constructor_name: z_ai_glm_4_6_thinking,
            display_name: "GLM 4.6 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5Turbo {
            model_name: "z-ai/glm-5-turbo",
            constructor_name: z_ai_glm_5_turbo,
            display_name: "GLM 5 Turbo",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5vTurbo {
            model_name: "z-ai/glm-5v-turbo",
            constructor_name: z_ai_glm_5v_turbo,
            display_name: "GLM 5V Turbo",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm5vTurboThinking {
            model_name: "z-ai/glm-5v-turbo:thinking",
            constructor_name: z_ai_glm_5v_turbo_thinking,
            display_name: "GLM 5V Turbo Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45Air {
            model_name: "zai-org/GLM-4.5-Air",
            constructor_name: zai_org_glm_4_5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45AirThinking {
            model_name: "zai-org/GLM-4.5-Air:thinking",
            constructor_name: zai_org_glm_4_5_air_thinking,
            display_name: "GLM 4.5 Air (Thinking)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm45Thinking {
            model_name: "zai-org/GLM-4.5:thinking",
            constructor_name: zai_org_glm_4_5_thinking,
            display_name: "GLM 4.5 (Thinking)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm46Turbo {
            model_name: "zai-org/GLM-4.6-turbo",
            constructor_name: zai_org_glm_4_6_turbo,
            display_name: "GLM 4.6 Turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm46TurboThinking {
            model_name: "zai-org/GLM-4.6-turbo:thinking",
            constructor_name: zai_org_glm_4_6_turbo_thinking,
            display_name: "GLM 4.6 Turbo (Thinking)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm45 {
            model_name: "zai-org/glm-4.5",
            constructor_name: zai_org_glm_4_5,
            display_name: "GLM 4.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm46Original {
            model_name: "zai-org/glm-4.6-original",
            constructor_name: zai_org_glm_4_6_original,
            display_name: "GLM 4.6 Original",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm46v {
            model_name: "zai-org/glm-4.6v",
            constructor_name: zai_org_glm_4_6v,
            display_name: "GLM 4.6V",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm46vFlashOriginal {
            model_name: "zai-org/glm-4.6v-flash-original",
            constructor_name: zai_org_glm_4_6v_flash_original,
            display_name: "GLM 4.6V Flash",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        ZaiOrgGlm46vOriginal {
            model_name: "zai-org/glm-4.6v-original",
            constructor_name: zai_org_glm_4_6v_original,
            display_name: "GLM 4.6V Original",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm47 {
            model_name: "zai-org/glm-4.7",
            constructor_name: zai_org_glm_4_7,
            display_name: "GLM 4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Flash {
            model_name: "zai-org/glm-4.7-flash",
            constructor_name: zai_org_glm_4_7_flash,
            display_name: "GLM 4.7 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47FlashOriginal {
            model_name: "zai-org/glm-4.7-flash-original",
            constructor_name: zai_org_glm_4_7_flash_original,
            display_name: "GLM 4.7 Flash Original",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47FlashOriginalThinking {
            model_name: "zai-org/glm-4.7-flash-original:thinking",
            constructor_name: zai_org_glm_4_7_flash_original_thinking,
            display_name: "GLM 4.7 Flash Original Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm47FlashThinking {
            model_name: "zai-org/glm-4.7-flash:thinking",
            constructor_name: zai_org_glm_4_7_flash_thinking,
            display_name: "GLM 4.7 Flash Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ZaiOrgGlm47Original {
            model_name: "zai-org/glm-4.7-original",
            constructor_name: zai_org_glm_4_7_original,
            display_name: "GLM 4.7 Original",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47OriginalThinking {
            model_name: "zai-org/glm-4.7-original:thinking",
            constructor_name: zai_org_glm_4_7_original_thinking,
            display_name: "GLM 4.7 Original Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm47Thinking {
            model_name: "zai-org/glm-4.7:thinking",
            constructor_name: zai_org_glm_4_7_thinking,
            display_name: "GLM 4.7 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5 {
            model_name: "zai-org/glm-5",
            constructor_name: zai_org_glm_5,
            display_name: "GLM 5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5Original {
            model_name: "zai-org/glm-5-original",
            constructor_name: zai_org_glm_5_original,
            display_name: "GLM 5 Original",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5OriginalThinking {
            model_name: "zai-org/glm-5-original:thinking",
            constructor_name: zai_org_glm_5_original_thinking,
            display_name: "GLM 5 Original Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm51 {
            model_name: "zai-org/glm-5.1",
            constructor_name: zai_org_glm_5_1,
            display_name: "GLM 5.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm51Thinking {
            model_name: "zai-org/glm-5.1:thinking",
            constructor_name: zai_org_glm_5_1_thinking,
            display_name: "GLM 5.1 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm52 {
            model_name: "zai-org/glm-5.2",
            constructor_name: zai_org_glm_5_2,
            display_name: "GLM 5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm52Thinking {
            model_name: "zai-org/glm-5.2:thinking",
            constructor_name: zai_org_glm_5_2_thinking,
            display_name: "GLM 5.2 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm5Thinking {
            model_name: "zai-org/glm-5:thinking",
            constructor_name: zai_org_glm_5_thinking,
            display_name: "GLM 5 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlmLatest {
            model_name: "zai-org/glm-latest",
            constructor_name: zai_org_glm_latest,
            display_name: "GLM Latest",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
