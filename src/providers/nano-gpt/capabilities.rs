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
        AlibabaNlpTongyiDeepresearch30bA3b {
            model_name: "Alibaba-NLP/Tongyi-DeepResearch-30B-A3B",
            constructor_name: alibaba_nlp_tongyi_deepresearch_30b_a3b,
            display_name: "Tongyi DeepResearch 30B A3B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
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
        CruciblelabL3370bLokiV20 {
            model_name: "CrucibleLab/L3.3-70B-Loki-V2.0",
            constructor_name: cruciblelab_l3_3_70b_loki_v2_0,
            display_name: "L3.3 70B Loki v2.0",
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
        Glm45AirDerestricted {
            model_name: "GLM-4.5-Air-Derestricted",
            constructor_name: glm_4_5_air_derestricted,
            display_name: "GLM 4.5 Air Derestricted",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm45AirDerestrictedIceblink {
            model_name: "GLM-4.5-Air-Derestricted-Iceblink",
            constructor_name: glm_4_5_air_derestricted_iceblink,
            display_name: "GLM 4.5 Air Derestricted Iceblink",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm45AirDerestrictedIceblinkReextract {
            model_name: "GLM-4.5-Air-Derestricted-Iceblink-ReExtract",
            constructor_name: glm_4_5_air_derestricted_iceblink_reextract,
            display_name: "GLM 4.5 Air Derestricted Iceblink ReExtract",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm45AirDerestrictedIceblinkV2 {
            model_name: "GLM-4.5-Air-Derestricted-Iceblink-v2",
            constructor_name: glm_4_5_air_derestricted_iceblink_v2,
            display_name: "GLM 4.5 Air Derestricted Iceblink v2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm45AirDerestrictedIceblinkV2Reextract {
            model_name: "GLM-4.5-Air-Derestricted-Iceblink-v2-ReExtract",
            constructor_name: glm_4_5_air_derestricted_iceblink_v2_reextract,
            display_name: "GLM 4.5 Air Derestricted Iceblink v2 ReExtract",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm45AirDerestrictedSteam {
            model_name: "GLM-4.5-Air-Derestricted-Steam",
            constructor_name: glm_4_5_air_derestricted_steam,
            display_name: "GLM 4.5 Air Derestricted Steam",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm45AirDerestrictedSteamReextract {
            model_name: "GLM-4.5-Air-Derestricted-Steam-ReExtract",
            constructor_name: glm_4_5_air_derestricted_steam_reextract,
            display_name: "GLM 4.5 Air Derestricted Steam ReExtract",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Glm46DerestrictedV5 {
            model_name: "GLM-4.6-Derestricted-v5",
            constructor_name: glm_4_6_derestricted_v5,
            display_name: "GLM 4.6 Derestricted v5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GalrionsoftworksMnLoosecannon12bV1 {
            model_name: "GalrionSoftworks/MN-LooseCannon-12B-v1",
            constructor_name: galrionsoftworks_mn_loosecannon_12b_v1,
            display_name: "MN-LooseCannon-12B-v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemma327bArliaiRpmaxV3 {
            model_name: "Gemma-3-27B-ArliAI-RPMax-v3",
            constructor_name: gemma_3_27b_arliai_rpmax_v3,
            display_name: "Gemma 3 27B RPMax v3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemma327bBigTigerV3 {
            model_name: "Gemma-3-27B-Big-Tiger-v3",
            constructor_name: gemma_3_27b_big_tiger_v3,
            display_name: "Gemma 3 27B Big Tiger v3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemma327bCardprojectorV4 {
            model_name: "Gemma-3-27B-CardProjector-v4",
            constructor_name: gemma_3_27b_cardprojector_v4,
            display_name: "Gemma 3 27B CardProjector v4",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemma327bGlitter {
            model_name: "Gemma-3-27B-Glitter",
            constructor_name: gemma_3_27b_glitter,
            display_name: "Gemma 3 27B Glitter",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemma327bNidumUncensored {
            model_name: "Gemma-3-27B-Nidum-Uncensored",
            constructor_name: gemma_3_27b_nidum_uncensored,
            display_name: "Gemma 3 27B Nidum Uncensored",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemma327bIt {
            model_name: "Gemma-3-27B-it",
            constructor_name: gemma_3_27b_it,
            display_name: "Gemma 3 27B IT",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemma327bItAbliterated {
            model_name: "Gemma-3-27B-it-Abliterated",
            constructor_name: gemma_3_27b_it_abliterated,
            display_name: "Gemma 3 27B IT Abliterated",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GrypheMythomaxL213b {
            model_name: "Gryphe/MythoMax-L2-13b",
            constructor_name: gryphe_mythomax_l2_13b,
            display_name: "MythoMax 13B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        InfermaticMn12bInferorV00 {
            model_name: "Infermatic/MN-12B-Inferor-v0.0",
            constructor_name: infermatic_mn_12b_inferor_v0_0,
            display_name: "Mistral Nemo Inferor 12B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        KatCoderAirV1 {
            model_name: "KAT-Coder-Air-V1",
            constructor_name: kat_coder_air_v1,
            display_name: "KAT Coder Air V1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        KatCoderExp72b1010 {
            model_name: "KAT-Coder-Exp-72B-1010",
            constructor_name: kat_coder_exp_72b_1010,
            display_name: "KAT Coder Exp 72B 1010",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        KatCoderProV1 {
            model_name: "KAT-Coder-Pro-V1",
            constructor_name: kat_coder_pro_v1,
            display_name: "KAT Coder Pro V1",
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
        Llama3331v3370bHanamiX1 {
            model_name: "Llama-3.3+(3.1v3.3)-70B-Hanami-x1",
            constructor_name: llama_3_3_3_1v3_3_70b_hanami_x1,
            display_name: "Llama 3.3+ 70B Hanami x1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3331v3370bNewDawnV11 {
            model_name: "Llama-3.3+(3.1v3.3)-70B-New-Dawn-v1.1",
            constructor_name: llama_3_3_3_1v3_3_70b_new_dawn_v1_1,
            display_name: "Llama 3.3+ 70B New Dawn v1.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama333v3370bTenyxchatDaybreakstorywriter {
            model_name: "Llama-3.3+(3v3.3)-70B-TenyxChat-DaybreakStorywriter",
            constructor_name: llama_3_3_3v3_3_70b_tenyxchat_daybreakstorywriter,
            display_name: "Llama 3.3+ 70B TenyxChat DaybreakStorywriter",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bAnthrobomination {
            model_name: "Llama-3.3-70B-Anthrobomination",
            constructor_name: llama_3_3_70b_anthrobomination,
            display_name: "Llama 3.3 70B Anthrobomination",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bArgunaut1Sft {
            model_name: "Llama-3.3-70B-Argunaut-1-SFT",
            constructor_name: llama_3_3_70b_argunaut_1_sft,
            display_name: "Llama 3.3 70B Argunaut 1 SFT",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bArliaiRpmaxV14 {
            model_name: "Llama-3.3-70B-ArliAI-RPMax-v1.4",
            constructor_name: llama_3_3_70b_arliai_rpmax_v1_4,
            display_name: "Llama 3.3 70B RPMax v1.4",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bArliaiRpmaxV2 {
            model_name: "Llama-3.3-70B-ArliAI-RPMax-v2",
            constructor_name: llama_3_3_70b_arliai_rpmax_v2,
            display_name: "Llama 3.3 70B ArliAI RPMax v2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bArliaiRpmaxV3 {
            model_name: "Llama-3.3-70B-ArliAI-RPMax-v3",
            constructor_name: llama_3_3_70b_arliai_rpmax_v3,
            display_name: "Llama 3.3 70B ArliAI RPMax v3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bAuroraBorealis {
            model_name: "Llama-3.3-70B-Aurora-Borealis",
            constructor_name: llama_3_3_70b_aurora_borealis,
            display_name: "Llama 3.3 70B Aurora Borealis",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bBiggerBody {
            model_name: "Llama-3.3-70B-Bigger-Body",
            constructor_name: llama_3_3_70b_bigger_body,
            display_name: "Llama 3.3 70B Bigger Body",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bCirrusX1 {
            model_name: "Llama-3.3-70B-Cirrus-x1",
            constructor_name: llama_3_3_70b_cirrus_x1,
            display_name: "Llama 3.3 70B Cirrus x1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bCuMaiR1 {
            model_name: "Llama-3.3-70B-Cu-Mai-R1",
            constructor_name: llama_3_3_70b_cu_mai_r1,
            display_name: "Llama 3.3 70B Cu Mai R1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bDamascusR1 {
            model_name: "Llama-3.3-70B-Damascus-R1",
            constructor_name: llama_3_3_70b_damascus_r1,
            display_name: "Damascus R1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bDarkAgesV01 {
            model_name: "Llama-3.3-70B-Dark-Ages-v0.1",
            constructor_name: llama_3_3_70b_dark_ages_v0_1,
            display_name: "Llama 3.3 70B Dark Ages v0.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bElectraR1 {
            model_name: "Llama-3.3-70B-Electra-R1",
            constructor_name: llama_3_3_70b_electra_r1,
            display_name: "Llama 3.3 70B Electra R1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bElectranovaV10 {
            model_name: "Llama-3.3-70B-Electranova-v1.0",
            constructor_name: llama_3_3_70b_electranova_v1_0,
            display_name: "Llama 3.3 70B Electranova v1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bFallenR1V1 {
            model_name: "Llama-3.3-70B-Fallen-R1-v1",
            constructor_name: llama_3_3_70b_fallen_r1_v1,
            display_name: "Llama 3.3 70B Fallen R1 v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bFallenV1 {
            model_name: "Llama-3.3-70B-Fallen-v1",
            constructor_name: llama_3_3_70b_fallen_v1,
            display_name: "Llama 3.3 70B Fallen v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bForgottenAbominationV50 {
            model_name: "Llama-3.3-70B-Forgotten-Abomination-v5.0",
            constructor_name: llama_3_3_70b_forgotten_abomination_v5_0,
            display_name: "Llama 3.3 70B Forgotten Abomination v5.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bForgottenSafeword36 {
            model_name: "Llama-3.3-70B-Forgotten-Safeword-3.6",
            constructor_name: llama_3_3_70b_forgotten_safeword_3_6,
            display_name: "Llama 3.3 70B Forgotten Safeword 3.6",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bGeneticlemonadeOpus {
            model_name: "Llama-3.3-70B-GeneticLemonade-Opus",
            constructor_name: llama_3_3_70b_geneticlemonade_opus,
            display_name: "Llama 3.3 70B GeneticLemonade Opus",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bGeneticlemonadeUnleashedV3 {
            model_name: "Llama-3.3-70B-GeneticLemonade-Unleashed-v3",
            constructor_name: llama_3_3_70b_geneticlemonade_unleashed_v3,
            display_name: "Llama 3.3 70B GeneticLemonade Unleashed v3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bIgnitionV01 {
            model_name: "Llama-3.3-70B-Ignition-v0.1",
            constructor_name: llama_3_3_70b_ignition_v0_1,
            display_name: "Llama 3.3 70B Ignition v0.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bIncandescentMalevolence {
            model_name: "Llama-3.3-70B-Incandescent-Malevolence",
            constructor_name: llama_3_3_70b_incandescent_malevolence,
            display_name: "Llama 3.3 70B Incandescent Malevolence",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bLegionV21 {
            model_name: "Llama-3.3-70B-Legion-V2.1",
            constructor_name: llama_3_3_70b_legion_v2_1,
            display_name: "Llama 3.3 70B Legion V2.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bMsNevoria {
            model_name: "Llama-3.3-70B-MS-Nevoria",
            constructor_name: llama_3_3_70b_ms_nevoria,
            display_name: "Llama 3.3 70B MS Nevoria",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bMagnumV4Se {
            model_name: "Llama-3.3-70B-Magnum-v4-SE",
            constructor_name: llama_3_3_70b_magnum_v4_se,
            display_name: "Llama 3.3 70B Magnum v4 SE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bMagnumV4SeCirrusX1Slerp {
            model_name: "Llama-3.3-70B-Magnum-v4-SE-Cirrus-x1-SLERP",
            constructor_name: llama_3_3_70b_magnum_v4_se_cirrus_x1_slerp,
            display_name: "Llama 3.3 70B Magnum v4 SE Cirrus x1 SLERP",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bMhnnnX1 {
            model_name: "Llama-3.3-70B-Mhnnn-x1",
            constructor_name: llama_3_3_70b_mhnnn_x1,
            display_name: "Llama 3.3 70B Mhnnn x1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bMiraifanfare {
            model_name: "Llama-3.3-70B-MiraiFanfare",
            constructor_name: llama_3_3_70b_miraifanfare,
            display_name: "Llama 3.3 70b Mirai Fanfare",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bMokumeGaneR1 {
            model_name: "Llama-3.3-70B-Mokume-Gane-R1",
            constructor_name: llama_3_3_70b_mokume_gane_r1,
            display_name: "Llama 3.3 70B Mokume Gane R1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bNova {
            model_name: "Llama-3.3-70B-Nova",
            constructor_name: llama_3_3_70b_nova,
            display_name: "Llama 3.3 70B Nova",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bPredatorialExtasy {
            model_name: "Llama-3.3-70B-Predatorial-Extasy",
            constructor_name: llama_3_3_70b_predatorial_extasy,
            display_name: "Llama 3.3 70B Predatorial Extasy",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bProgenitorV33 {
            model_name: "Llama-3.3-70B-Progenitor-V3.3",
            constructor_name: llama_3_3_70b_progenitor_v3_3,
            display_name: "Llama 3.3 70B Progenitor V3.3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bRawmaw {
            model_name: "Llama-3.3-70B-RAWMAW",
            constructor_name: llama_3_3_70b_rawmaw,
            display_name: "Llama 3.3 70B RAWMAW",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bSapphira01 {
            model_name: "Llama-3.3-70B-Sapphira-0.1",
            constructor_name: llama_3_3_70b_sapphira_0_1,
            display_name: "Llama 3.3 70B Sapphira 0.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bSapphira02 {
            model_name: "Llama-3.3-70B-Sapphira-0.2",
            constructor_name: llama_3_3_70b_sapphira_0_2,
            display_name: "Llama 3.3 70B Sapphira 0.2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bShakudo {
            model_name: "Llama-3.3-70B-Shakudo",
            constructor_name: llama_3_3_70b_shakudo,
            display_name: "Llama 3.3 70B Shakudo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bStrawberrylemonadeV10 {
            model_name: "Llama-3.3-70B-StrawberryLemonade-v1.0",
            constructor_name: llama_3_3_70b_strawberrylemonade_v1_0,
            display_name: "Llama 3.3 70B StrawberryLemonade v1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bStrawberrylemonadeV12 {
            model_name: "Llama-3.3-70B-Strawberrylemonade-v1.2",
            constructor_name: llama_3_3_70b_strawberrylemonade_v1_2,
            display_name: "Llama 3.3 70B StrawberryLemonade v1.2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bTheOmegaDirectiveUnslopV20 {
            model_name: "Llama-3.3-70B-The-Omega-Directive-Unslop-v2.0",
            constructor_name: llama_3_3_70b_the_omega_directive_unslop_v2_0,
            display_name: "Llama 3.3 70B Omega Directive Unslop v2.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bTheOmegaDirectiveUnslopV21 {
            model_name: "Llama-3.3-70B-The-Omega-Directive-Unslop-v2.1",
            constructor_name: llama_3_3_70b_the_omega_directive_unslop_v2_1,
            display_name: "Llama 3.3 70B Omega Directive Unslop v2.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Llama3370bVulpeculaR1 {
            model_name: "Llama-3.3-70B-Vulpecula-R1",
            constructor_name: llama_3_3_70b_vulpecula_r1,
            display_name: "Llama 3.3 70B Vulpecula R1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MagistralSmall2506 {
            model_name: "Magistral-Small-2506",
            constructor_name: magistral_small_2506,
            display_name: "Magistral Small 2506",
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
        MistralNemo12bInstruct2407 {
            model_name: "Mistral-Nemo-12B-Instruct-2407",
            constructor_name: mistral_nemo_12b_instruct_2407,
            display_name: "Mistral Nemo 12B Instruct 2407",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        NeversleepLlama3Lumimaid70bV01 {
            model_name: "NeverSleep/Llama-3-Lumimaid-70B-v0.1",
            constructor_name: neversleep_llama_3_lumimaid_70b_v0_1,
            display_name: "Lumimaid 70b",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        NeversleepLumimaidV0270b {
            model_name: "NeverSleep/Lumimaid-v0.2-70B",
            constructor_name: neversleep_lumimaid_v0_2_70b,
            display_name: "Lumimaid v0.2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Nousresearch2Deephermes3Mistral24bPreview {
            model_name: "NousResearch 2/DeepHermes-3-Mistral-24B-Preview",
            constructor_name: nousresearch_2_deephermes_3_mistral_24b_preview,
            display_name: "DeepHermes-3 Mistral 24B (Preview)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Nousresearch2Hermes470bThinking {
            model_name: "NousResearch 2/Hermes-4-70B:thinking",
            constructor_name: nousresearch_2_hermes_4_70b_thinking,
            display_name: "Hermes 4 (Thinking)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Nousresearch2Hermes3Llama3170b {
            model_name: "NousResearch 2/hermes-3-llama-3.1-70b",
            constructor_name: nousresearch_2_hermes_3_llama_3_1_70b,
            display_name: "Hermes 3 70B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Nousresearch2Hermes4405b {
            model_name: "NousResearch 2/hermes-4-405b",
            constructor_name: nousresearch_2_hermes_4_405b,
            display_name: "Hermes 4 Large",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Nousresearch2Hermes4405bThinking {
            model_name: "NousResearch 2/hermes-4-405b:thinking",
            constructor_name: nousresearch_2_hermes_4_405b_thinking,
            display_name: "Hermes 4 Large (Thinking)",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Nousresearch2Hermes470b {
            model_name: "NousResearch 2/hermes-4-70b",
            constructor_name: nousresearch_2_hermes_4_70b,
            display_name: "Hermes 4 Medium",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwq32bArliaiRprV1 {
            model_name: "QwQ-32B-ArliAI-RpR-v1",
            constructor_name: qwq_32b_arliai_rpr_v1,
            display_name: "QwQ 32b Arli V1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwen2532bEvaV02 {
            model_name: "Qwen2.5-32B-EVA-v0.2",
            constructor_name: qwen2_5_32b_eva_v0_2,
            display_name: "Qwen 2.5 32b EVA",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ReadyartMs32TheOmegaDirective24bUnslopV20 {
            model_name: "ReadyArt/MS3.2-The-Omega-Directive-24B-Unslop-v2.0",
            constructor_name: readyart_ms3_2_the_omega_directive_24b_unslop_v2_0,
            display_name: "Omega Directive 24B Unslop v2.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ReadyartTheOmegaAbominationL70bV10 {
            model_name: "ReadyArt/The-Omega-Abomination-L-70B-v1.0",
            constructor_name: readyart_the_omega_abomination_l_70b_v1_0,
            display_name: "The Omega Abomination V1",
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
        SteelskullL33MsEvalebis70b {
            model_name: "Steelskull/L3.3-MS-Evalebis-70b",
            constructor_name: steelskull_l3_3_ms_evalebis_70b,
            display_name: "MS Evalebis 70b",
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
        TeeDeepseekR10528 {
            model_name: "TEE/deepseek-r1-0528",
            constructor_name: tee_deepseek_r1_0528,
            display_name: "DeepSeek R1 0528 TEE",
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
        TeeGemma327bIt {
            model_name: "TEE/gemma-3-27b-it",
            constructor_name: tee_gemma_3_27b_it,
            display_name: "Gemma 3 27B TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeGlm46 {
            model_name: "TEE/glm-4.6",
            constructor_name: tee_glm_4_6,
            display_name: "GLM 4.6 TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeGlm47 {
            model_name: "TEE/glm-4.7",
            constructor_name: tee_glm_4_7,
            display_name: "GLM 4.7 TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeGlm47Flash {
            model_name: "TEE/glm-4.7-flash",
            constructor_name: tee_glm_4_7_flash,
            display_name: "GLM 4.7 Flash TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeGlm5 {
            model_name: "TEE/glm-5",
            constructor_name: tee_glm_5,
            display_name: "GLM 5 TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        TeeKimiK2Thinking {
            model_name: "TEE/kimi-k2-thinking",
            constructor_name: tee_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeKimiK25 {
            model_name: "TEE/kimi-k2.5",
            constructor_name: tee_kimi_k2_5,
            display_name: "Kimi K2.5 TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeKimiK25Thinking {
            model_name: "TEE/kimi-k2.5-thinking",
            constructor_name: tee_kimi_k2_5_thinking,
            display_name: "Kimi K2.5 Thinking TEE",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        TeeLlama3370b {
            model_name: "TEE/llama3-3-70b",
            constructor_name: tee_llama3_3_70b,
            display_name: "Llama 3.3 70B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeMinimaxM21 {
            model_name: "TEE/minimax-m2.1",
            constructor_name: tee_minimax_m2_1,
            display_name: "MiniMax M2.1 TEE",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TeeQwen25Vl72bInstruct {
            model_name: "TEE/qwen2.5-vl-72b-instruct",
            constructor_name: tee_qwen2_5_vl_72b_instruct,
            display_name: "Qwen2.5 VL 72B TEE",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        TeeQwen330bA3bInstruct2507 {
            model_name: "TEE/qwen3-30b-a3b-instruct-2507",
            constructor_name: tee_qwen3_30b_a3b_instruct_2507,
            display_name: "Qwen3 30B A3B Instruct 2507 TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeQwen3Coder {
            model_name: "TEE/qwen3-coder",
            constructor_name: tee_qwen3_coder,
            display_name: "Qwen3 Coder 480B TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TeeQwen35397bA17b {
            model_name: "TEE/qwen3.5-397b-a17b",
            constructor_name: tee_qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B A17B TEE",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        ThudmGlmZ132b0414 {
            model_name: "THUDM/GLM-Z1-32B-0414",
            constructor_name: thudm_glm_z1_32b_0414,
            display_name: "GLM Z1 32B 0414",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThudmGlmZ19b0414 {
            model_name: "THUDM/GLM-Z1-9B-0414",
            constructor_name: thudm_glm_z1_9b_0414,
            display_name: "GLM Z1 9B 0414",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ThudmGlmZ1Rumination32b0414 {
            model_name: "THUDM/GLM-Z1-Rumination-32B-0414",
            constructor_name: thudm_glm_z1_rumination_32b_0414,
            display_name: "GLM Z1 Rumination 32B 0414",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Thedrummer2Anubis70bV1 {
            model_name: "TheDrummer 2/Anubis-70B-v1",
            constructor_name: thedrummer_2_anubis_70b_v1,
            display_name: "Anubis 70B v1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Thedrummer2Anubis70bV11 {
            model_name: "TheDrummer 2/Anubis-70B-v1.1",
            constructor_name: thedrummer_2_anubis_70b_v1_1,
            display_name: "Anubis 70B v1.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Thedrummer2Cydonia24bV2 {
            model_name: "TheDrummer 2/Cydonia-24B-v2",
            constructor_name: thedrummer_2_cydonia_24b_v2,
            display_name: "The Drummer Cydonia 24B v2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Thedrummer2Cydonia24bV4 {
            model_name: "TheDrummer 2/Cydonia-24B-v4",
            constructor_name: thedrummer_2_cydonia_24b_v4,
            display_name: "The Drummer Cydonia 24B v4",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Thedrummer2Cydonia24bV41 {
            model_name: "TheDrummer 2/Cydonia-24B-v4.1",
            constructor_name: thedrummer_2_cydonia_24b_v4_1,
            display_name: "The Drummer Cydonia 24B v4.1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Thedrummer2Cydonia24bV43 {
            model_name: "TheDrummer 2/Cydonia-24B-v4.3",
            constructor_name: thedrummer_2_cydonia_24b_v4_3,
            display_name: "The Drummer Cydonia 24B v4.3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Thedrummer2Magidonia24bV43 {
            model_name: "TheDrummer 2/Magidonia-24B-v4.3",
            constructor_name: thedrummer_2_magidonia_24b_v4_3,
            display_name: "The Drummer Magidonia 24B v4.3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Thedrummer2Rocinante12bV11 {
            model_name: "TheDrummer 2/Rocinante-12B-v1.1",
            constructor_name: thedrummer_2_rocinante_12b_v1_1,
            display_name: "Rocinante 12b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Thedrummer2Unslopnemo12bV41 {
            model_name: "TheDrummer 2/UnslopNemo-12B-v4.1",
            constructor_name: thedrummer_2_unslopnemo_12b_v4_1,
            display_name: "UnslopNemo 12b v4",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Thedrummer2Skyfall36bV2 {
            model_name: "TheDrummer 2/skyfall-36b-v2",
            constructor_name: thedrummer_2_skyfall_36b_v2,
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
        AionLabsAion10 {
            model_name: "aion-labs/aion-1.0",
            constructor_name: aion_labs_aion_1_0,
            display_name: "Aion 1.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AionLabsAion10Mini {
            model_name: "aion-labs/aion-1.0-mini",
            constructor_name: aion_labs_aion_1_0_mini,
            display_name: "Aion 1.0 mini (DeepSeek)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AionLabsAionRpLlama318b {
            model_name: "aion-labs/aion-rp-llama-3.1-8b",
            constructor_name: aion_labs_aion_rp_llama_3_1_8b,
            display_name: "Llama 3.1 8b (uncensored)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AlibabaQwen36Flash {
            model_name: "alibaba/qwen3.6-flash",
            constructor_name: alibaba_qwen3_6_flash,
            display_name: "Qwen3.6 Flash",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
        },
        AllenaiMolmo28b {
            model_name: "allenai/molmo-2-8b",
            constructor_name: allenai_molmo_2_8b,
            display_name: "Molmo 2 8B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        AllenaiOlmo332bThink {
            model_name: "allenai/olmo-3-32b-think",
            constructor_name: allenai_olmo_3_32b_think,
            display_name: "Olmo 3 32B Think",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        AllenaiOlmo3132bInstruct {
            model_name: "allenai/olmo-3.1-32b-instruct",
            constructor_name: allenai_olmo_3_1_32b_instruct,
            display_name: "Olmo 3.1 32B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AllenaiOlmo3132bThink {
            model_name: "allenai/olmo-3.1-32b-think",
            constructor_name: allenai_olmo_3_1_32b_think,
            display_name: "Olmo 3.1 32B Think",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
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
        AnthropicClaudeSonnet46 {
            model_name: "anthropic/claude-sonnet-4.6",
            constructor_name: anthropic_claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet46Thinking {
            model_name: "anthropic/claude-sonnet-4.6:thinking",
            constructor_name: anthropic_claude_sonnet_4_6_thinking,
            display_name: "Claude Sonnet 4.6 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ArceeAiTrinityLarge {
            model_name: "arcee-ai/trinity-large",
            constructor_name: arcee_ai_trinity_large,
            display_name: "Trinity Large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ArceeAiTrinityMini {
            model_name: "arcee-ai/trinity-mini",
            constructor_name: arcee_ai_trinity_mini,
            display_name: "Trinity Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        BaiduErnie45300bA47b {
            model_name: "baidu/ernie-4.5-300b-a47b",
            constructor_name: baidu_ernie_4_5_300b_a47b,
            display_name: "ERNIE 4.5 300B",
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
        Chroma {
            model_name: "chroma",
            constructor_name: chroma,
            display_name: "Chroma",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        ChutesaiMistralSmall3224bInstruct2506 {
            model_name: "chutesai/Mistral-Small-3.2-24B-Instruct-2506",
            constructor_name: chutesai_mistral_small_3_2_24b_instruct_2506,
            display_name: "Mistral Small 3.2 24b Instruct",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Claude35Haiku20241022 {
            model_name: "claude-3-5-haiku-20241022",
            constructor_name: claude_3_5_haiku_20241022,
            display_name: "Claude 3.5 Haiku",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude35Sonnet20240620 {
            model_name: "claude-3-5-sonnet-20240620",
            constructor_name: claude_3_5_sonnet_20240620,
            display_name: "Claude 3.5 Sonnet Old",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude35Sonnet20241022 {
            model_name: "claude-3-5-sonnet-20241022",
            constructor_name: claude_3_5_sonnet_20241022,
            display_name: "Claude 3.5 Sonnet",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude37Sonnet20250219 {
            model_name: "claude-3-7-sonnet-20250219",
            constructor_name: claude_3_7_sonnet_20250219,
            display_name: "Claude 3.7 Sonnet",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude37SonnetReasoner {
            model_name: "claude-3-7-sonnet-reasoner",
            constructor_name: claude_3_7_sonnet_reasoner,
            display_name: "Claude 3.7 Sonnet Reasoner",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Claude37SonnetThinking {
            model_name: "claude-3-7-sonnet-thinking",
            constructor_name: claude_3_7_sonnet_thinking,
            display_name: "Claude 3.7 Sonnet Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude37SonnetThinking1024 {
            model_name: "claude-3-7-sonnet-thinking:1024",
            constructor_name: claude_3_7_sonnet_thinking_1024,
            display_name: "Claude 3.7 Sonnet Thinking (1K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude37SonnetThinking128000 {
            model_name: "claude-3-7-sonnet-thinking:128000",
            constructor_name: claude_3_7_sonnet_thinking_128000,
            display_name: "Claude 3.7 Sonnet Thinking (128K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude37SonnetThinking32768 {
            model_name: "claude-3-7-sonnet-thinking:32768",
            constructor_name: claude_3_7_sonnet_thinking_32768,
            display_name: "Claude 3.7 Sonnet Thinking (32K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Claude37SonnetThinking8192 {
            model_name: "claude-3-7-sonnet-thinking:8192",
            constructor_name: claude_3_7_sonnet_thinking_8192,
            display_name: "Claude 3.7 Sonnet Thinking (8K)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku4520251001 {
            model_name: "claude-haiku-4-5-20251001",
            constructor_name: claude_haiku_4_5_20251001,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        CognitivecomputationsDolphin292Qwen272b {
            model_name: "cognitivecomputations/dolphin-2.9.2-qwen2-72b",
            constructor_name: cognitivecomputations_dolphin_2_9_2_qwen2_72b,
            display_name: "Dolphin 72b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereCommandR {
            model_name: "cohere/command-r",
            constructor_name: cohere_command_r,
            display_name: "Cohere: Command R",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereCommandRPlus082024 {
            model_name: "cohere/command-r-plus-08-2024",
            constructor_name: cohere_command_r_plus_08_2024,
            display_name: "Cohere: Command R+",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CommandAReasoning082025 {
            model_name: "command-a-reasoning-08-2025",
            constructor_name: command_a_reasoning_08_2025,
            display_name: "Cohere Command A (08/2025)",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        DeepcogitoCogitoV21671b {
            model_name: "deepcogito/cogito-v2.1-671b",
            constructor_name: deepcogito_cogito_v2_1_671b,
            display_name: "Cogito v2.1 671B MoE",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
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
        DeepseekMathV2 {
            model_name: "deepseek-math-v2",
            constructor_name: deepseek_math_v2,
            display_name: "DeepSeek Math V2",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        DeepseekDeepseekV32Speciale {
            model_name: "deepseek/deepseek-v3.2-speciale",
            constructor_name: deepseek_deepseek_v3_2_speciale,
            display_name: "DeepSeek V3.2 Speciale",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekV32Thinking {
            model_name: "deepseek/deepseek-v3.2:thinking",
            constructor_name: deepseek_deepseek_v3_2_thinking,
            display_name: "DeepSeek V3.2 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DmindDmind1 {
            model_name: "dmind/dmind-1",
            constructor_name: dmind_dmind_1,
            display_name: "DMind-1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DmindDmind1Mini {
            model_name: "dmind/dmind-1-mini",
            constructor_name: dmind_dmind_1_mini,
            display_name: "DMind-1-Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Doubao15ThinkingPro250415 {
            model_name: "doubao-1-5-thinking-pro-250415",
            constructor_name: doubao_1_5_thinking_pro_250415,
            display_name: "Doubao 1.5 Thinking Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Doubao15ThinkingProVision250415 {
            model_name: "doubao-1-5-thinking-pro-vision-250415",
            constructor_name: doubao_1_5_thinking_pro_vision_250415,
            display_name: "Doubao 1.5 Thinking Pro Vision",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Doubao15ThinkingVisionPro250428 {
            model_name: "doubao-1-5-thinking-vision-pro-250428",
            constructor_name: doubao_1_5_thinking_vision_pro_250428,
            display_name: "Doubao 1.5 Thinking Vision Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
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
        DoubaoSeed16Thinking250615 {
            model_name: "doubao-seed-1-6-thinking-250615",
            constructor_name: doubao_seed_1_6_thinking_250615,
            display_name: "Doubao Seed 1.6 Thinking",
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
        DoubaoSeedCodePreviewLatest {
            model_name: "doubao-seed-code-preview-latest",
            constructor_name: doubao_seed_code_preview_latest,
            display_name: "Doubao Seed Code Preview",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Ernie458kPreview {
            model_name: "ernie-4.5-8k-preview",
            constructor_name: ernie_4_5_8k_preview,
            display_name: "Ernie 4.5 8k Preview",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Ernie45Turbo128k {
            model_name: "ernie-4.5-turbo-128k",
            constructor_name: ernie_4_5_turbo_128k,
            display_name: "Ernie 4.5 Turbo 128k",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Ernie45TurboVl32k {
            model_name: "ernie-4.5-turbo-vl-32k",
            constructor_name: ernie_4_5_turbo_vl_32k,
            display_name: "Ernie 4.5 Turbo VL 32k",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Ernie50ThinkingLatest {
            model_name: "ernie-5.0-thinking-latest",
            constructor_name: ernie_5_0_thinking_latest,
            display_name: "Ernie 5.0 Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Ernie50ThinkingPreview {
            model_name: "ernie-5.0-thinking-preview",
            constructor_name: ernie_5_0_thinking_preview,
            display_name: "Ernie 5.0 Thinking Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        ErnieX132k {
            model_name: "ernie-x1-32k",
            constructor_name: ernie_x1_32k,
            display_name: "Ernie X1 32k",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ErnieX132kPreview {
            model_name: "ernie-x1-32k-preview",
            constructor_name: ernie_x1_32k_preview,
            display_name: "Ernie X1 32k",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ErnieX1Turbo32k {
            model_name: "ernie-x1-turbo-32k",
            constructor_name: ernie_x1_turbo_32k,
            display_name: "Ernie X1 Turbo 32k",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        ErnieX11Preview {
            model_name: "ernie-x1.1-preview",
            constructor_name: ernie_x1_1_preview,
            display_name: "ERNIE X1.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        EssentialaiRnj1Instruct {
            model_name: "essentialai/rnj-1-instruct",
            constructor_name: essentialai_rnj_1_instruct,
            display_name: "RNJ-1 Instruct 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ExaAnswer {
            model_name: "exa-answer",
            constructor_name: exa_answer,
            display_name: "Exa (Answer)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ExaResearch {
            model_name: "exa-research",
            constructor_name: exa_research,
            display_name: "Exa (Research)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        ExaResearchPro {
            model_name: "exa-research-pro",
            constructor_name: exa_research_pro,
            display_name: "Exa (Research Pro)",
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
        Gemini20Flash001 {
            model_name: "gemini-2.0-flash-001",
            constructor_name: gemini_2_0_flash_001,
            display_name: "Gemini 2.0 Flash",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini20FlashExpImageGeneration {
            model_name: "gemini-2.0-flash-exp-image-generation",
            constructor_name: gemini_2_0_flash_exp_image_generation,
            display_name: "Gemini Text + Image",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemini20FlashLite {
            model_name: "gemini-2.0-flash-lite",
            constructor_name: gemini_2_0_flash_lite,
            display_name: "Gemini 2.0 Flash Lite",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini20FlashThinkingExp0121 {
            model_name: "gemini-2.0-flash-thinking-exp-01-21",
            constructor_name: gemini_2_0_flash_thinking_exp_01_21,
            display_name: "Gemini 2.0 Flash Thinking 0121",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini20FlashThinkingExp1219 {
            model_name: "gemini-2.0-flash-thinking-exp-1219",
            constructor_name: gemini_2_0_flash_thinking_exp_1219,
            display_name: "Gemini 2.0 Flash Thinking 1219",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemini20ProExp0205 {
            model_name: "gemini-2.0-pro-exp-02-05",
            constructor_name: gemini_2_0_pro_exp_02_05,
            display_name: "Gemini 2.0 Pro 0205",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini20ProReasoner {
            model_name: "gemini-2.0-pro-reasoner",
            constructor_name: gemini_2_0_pro_reasoner,
            display_name: "Gemini 2.0 Pro Reasoner",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Gemini25Flash {
            model_name: "gemini-2.5-flash",
            constructor_name: gemini_2_5_flash,
            display_name: "Gemini 2.5 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashLite {
            model_name: "gemini-2.5-flash-lite",
            constructor_name: gemini_2_5_flash_lite,
            display_name: "Gemini 2.5 Flash Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashLitePreview0617 {
            model_name: "gemini-2.5-flash-lite-preview-06-17",
            constructor_name: gemini_2_5_flash_lite_preview_06_17,
            display_name: "Gemini 2.5 Flash Lite Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashLitePreview092025 {
            model_name: "gemini-2.5-flash-lite-preview-09-2025",
            constructor_name: gemini_2_5_flash_lite_preview_09_2025,
            display_name: "Gemini 2.5 Flash Lite Preview (09/2025)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashLitePreview092025Thinking {
            model_name: "gemini-2.5-flash-lite-preview-09-2025-thinking",
            constructor_name: gemini_2_5_flash_lite_preview_09_2025_thinking,
            display_name: "Gemini 2.5 Flash Lite Preview (09/2025) – Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashNothinking {
            model_name: "gemini-2.5-flash-nothinking",
            constructor_name: gemini_2_5_flash_nothinking,
            display_name: "Gemini 2.5 Flash (No Thinking)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashPreview0417 {
            model_name: "gemini-2.5-flash-preview-04-17",
            constructor_name: gemini_2_5_flash_preview_04_17,
            display_name: "Gemini 2.5 Flash Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25FlashPreview0417Thinking {
            model_name: "gemini-2.5-flash-preview-04-17:thinking",
            constructor_name: gemini_2_5_flash_preview_04_17_thinking,
            display_name: "Gemini 2.5 Flash Preview Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25FlashPreview092025Thinking {
            model_name: "gemini-2.5-flash-preview-09-2025-thinking",
            constructor_name: gemini_2_5_flash_preview_09_2025_thinking,
            display_name: "Gemini 2.5 Flash Preview (09/2025) – Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini25Pro {
            model_name: "gemini-2.5-pro",
            constructor_name: gemini_2_5_pro,
            display_name: "Gemini 2.5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25ProExp0325 {
            model_name: "gemini-2.5-pro-exp-03-25",
            constructor_name: gemini_2_5_pro_exp_03_25,
            display_name: "Gemini 2.5 Pro Experimental 0325",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25ProPreview0325 {
            model_name: "gemini-2.5-pro-preview-03-25",
            constructor_name: gemini_2_5_pro_preview_03_25,
            display_name: "Gemini 2.5 Pro Preview 0325",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25ProPreview0506 {
            model_name: "gemini-2.5-pro-preview-05-06",
            constructor_name: gemini_2_5_pro_preview_05_06,
            display_name: "Gemini 2.5 Pro Preview 0506",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini25ProPreview0605 {
            model_name: "gemini-2.5-pro-preview-06-05",
            constructor_name: gemini_2_5_pro_preview_06_05,
            display_name: "Gemini 2.5 Pro Preview 0605",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini3ProImagePreview {
            model_name: "gemini-3-pro-image-preview",
            constructor_name: gemini_3_pro_image_preview,
            display_name: "Gemini 3 Pro Image",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Gemini3ProPreview {
            model_name: "gemini-3-pro-preview",
            constructor_name: gemini_3_pro_preview,
            display_name: "Gemini 3 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemini3ProPreviewThinking {
            model_name: "gemini-3-pro-preview-thinking",
            constructor_name: gemini_3_pro_preview_thinking,
            display_name: "Gemini 3 Pro Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GeminiExp1206 {
            model_name: "gemini-exp-1206",
            constructor_name: gemini_exp_1206,
            display_name: "Gemini 2.0 Pro 1206",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
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
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini3FlashPreviewThinking {
            model_name: "google/gemini-3-flash-preview-thinking",
            constructor_name: google_gemini_3_flash_preview_thinking,
            display_name: "Gemini 3 Flash Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGeminiFlash15 {
            model_name: "google/gemini-flash-1.5",
            constructor_name: google_gemini_flash_1_5,
            display_name: "Gemini 1.5 Flash",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Grok3Beta {
            model_name: "grok-3-beta",
            constructor_name: grok_3_beta,
            display_name: "Grok 3 Beta",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Grok3FastBeta {
            model_name: "grok-3-fast-beta",
            constructor_name: grok_3_fast_beta,
            display_name: "Grok 3 Fast Beta",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Grok3MiniBeta {
            model_name: "grok-3-mini-beta",
            constructor_name: grok_3_mini_beta,
            display_name: "Grok 3 Mini Beta",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Grok3MiniFastBeta {
            model_name: "grok-3-mini-fast-beta",
            constructor_name: grok_3_mini_fast_beta,
            display_name: "Grok 3 Mini Fast Beta",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Hidream {
            model_name: "hidream",
            constructor_name: hidream,
            display_name: "Hidream",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
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
        HuihuiAiLlama31Nemotron70bInstructHfAbliterated {
            model_name: "huihui-ai/Llama-3.1-Nemotron-70B-Instruct-HF-abliterated",
            constructor_name: huihui_ai_llama_3_1_nemotron_70b_instruct_hf_abliterated,
            display_name: "Nemotron 3.1 70B abliterated",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        HunyuanT1Latest {
            model_name: "hunyuan-t1-latest",
            constructor_name: hunyuan_t1_latest,
            display_name: "Hunyuan T1",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        HunyuanTurbos20250226 {
            model_name: "hunyuan-turbos-20250226",
            constructor_name: hunyuan_turbos_20250226,
            display_name: "Hunyuan Turbo S",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        KimiThinkingPreview {
            model_name: "kimi-thinking-preview",
            constructor_name: kimi_thinking_preview,
            display_name: "Kimi Thinking Preview",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        Learnlm15ProExperimental {
            model_name: "learnlm-1.5-pro-experimental",
            constructor_name: learnlm_1_5_pro_experimental,
            display_name: "Gemini LearnLM Experimental",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        MeituanLongcatLongcatFlashChatFp8 {
            model_name: "meituan-longcat/LongCat-Flash-Chat-FP8",
            constructor_name: meituan_longcat_longcat_flash_chat_fp8,
            display_name: "LongCat Flash",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        MetaLlamaLlama3290bVisionInstruct {
            model_name: "meta-llama/llama-3.2-90b-vision-instruct",
            constructor_name: meta_llama_llama_3_2_90b_vision_instruct,
            display_name: "Llama 3.2 Medium",
            capabilities: [TextInputSupport, TextOutputSupport]
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
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama4Scout {
            model_name: "meta-llama/llama-4-scout",
            constructor_name: meta_llama_llama_4_scout,
            display_name: "Llama 4 Scout",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MicrosoftMaiDsR1Fp8 {
            model_name: "microsoft/MAI-DS-R1-FP8",
            constructor_name: microsoft_mai_ds_r1_fp8,
            display_name: "Microsoft DeepSeek R1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
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
        MiromindAiMirothinkerV15235b {
            model_name: "miromind-ai/mirothinker-v1.5-235b",
            constructor_name: miromind_ai_mirothinker_v1_5_235b,
            display_name: "MiroThinker v1.5 235B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralSmall3124bInstruct {
            model_name: "mistral-small-31-24b-instruct",
            constructor_name: mistral_small_31_24b_instruct,
            display_name: "Mistral Small 31 24b Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
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
        MistralaiMistral7bInstruct {
            model_name: "mistralai/mistral-7b-instruct",
            constructor_name: mistralai_mistral_7b_instruct,
            display_name: "Mistral 7B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
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
        MistralaiMistralSmallCreative {
            model_name: "mistralai/mistral-small-creative",
            constructor_name: mistralai_mistral_small_creative,
            display_name: "Mistral Small Creative",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralTiny {
            model_name: "mistralai/mistral-tiny",
            constructor_name: mistralai_mistral_tiny,
            display_name: "Mistral Tiny",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMixtral8x22bInstructV01 {
            model_name: "mistralai/mixtral-8x22b-instruct-v0.1",
            constructor_name: mistralai_mixtral_8x22b_instruct_v0_1,
            display_name: "Mixtral 8x22B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralaiMixtral8x7bInstructV01 {
            model_name: "mistralai/mixtral-8x7b-instruct-v0.1",
            constructor_name: mistralai_mixtral_8x7b_instruct_v0_1,
            display_name: "Mixtral 8x7B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MlabonneNeuraldaredevil8bAbliterated {
            model_name: "mlabonne/NeuralDaredevil-8B-abliterated",
            constructor_name: mlabonne_neuraldaredevil_8b_abliterated,
            display_name: "Neural Daredevil 8B abliterated",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MoonshotaiKimiDev72b {
            model_name: "moonshotai/Kimi-Dev-72B",
            constructor_name: moonshotai_kimi_dev_72b,
            display_name: "Kimi Dev 72B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
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
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2ThinkingOriginal {
            model_name: "moonshotai/kimi-k2-thinking-original",
            constructor_name: moonshotai_kimi_k2_thinking_original,
            display_name: "Kimi K2 Thinking Original",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        MoonshotaiKimiK2ThinkingTurboOriginal {
            model_name: "moonshotai/kimi-k2-thinking-turbo-original",
            constructor_name: moonshotai_kimi_k2_thinking_turbo_original,
            display_name: "Kimi K2 Thinking Turbo Original",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
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
        NexAgiDeepseekV31NexN1 {
            model_name: "nex-agi/deepseek-v3.1-nex-n1",
            constructor_name: nex_agi_deepseek_v3_1_nex_n1,
            display_name: "DeepSeek V3.1 Nex N1",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        NvidiaLlama31NemotronUltra253bV1 {
            model_name: "nvidia/Llama-3.1-Nemotron-Ultra-253B-v1",
            constructor_name: nvidia_llama_3_1_nemotron_ultra_253b_v1,
            display_name: "Nvidia Nemotron Ultra 253B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlama33NemotronSuper49bV1 {
            model_name: "nvidia/Llama-3.3-Nemotron-Super-49B-v1",
            constructor_name: nvidia_llama_3_3_nemotron_super_49b_v1,
            display_name: "Nvidia Nemotron Super 49B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaLlama33NemotronSuper49bV15 {
            model_name: "nvidia/Llama-3_3-Nemotron-Super-49B-v1_5",
            constructor_name: nvidia_llama_3_3_nemotron_super_49b_v1_5,
            display_name: "Nvidia Nemotron Super 49B v1.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3Nano30bA3b {
            model_name: "nvidia/nemotron-3-nano-30b-a3b",
            constructor_name: nvidia_nemotron_3_nano_30b_a3b,
            display_name: "Nvidia Nemotron 3 Nano 30B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaNvidiaNemotronNano9bV2 {
            model_name: "nvidia/nvidia-nemotron-nano-9b-v2",
            constructor_name: nvidia_nvidia_nemotron_nano_9b_v2,
            display_name: "Nvidia Nemotron Nano 9B v2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiChatgpt4oLatest {
            model_name: "openai/chatgpt-4o-latest",
            constructor_name: openai_chatgpt_4o_latest,
            display_name: "ChatGPT 4o",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        OpenaiGpt5ChatLatest {
            model_name: "openai/gpt-5-chat-latest",
            constructor_name: openai_gpt_5_chat_latest,
            display_name: "GPT 5 Chat",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt5Codex {
            model_name: "openai/gpt-5-codex",
            constructor_name: openai_gpt_5_codex,
            display_name: "GPT-5 Codex",
            capabilities: [TextInputSupport, TextOutputSupport]
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
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt51Chat {
            model_name: "openai/gpt-5.1-chat",
            constructor_name: openai_gpt_5_1_chat,
            display_name: "GPT 5.1 Chat",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt51ChatLatest {
            model_name: "openai/gpt-5.1-chat-latest",
            constructor_name: openai_gpt_5_1_chat_latest,
            display_name: "GPT 5.1 Chat (Latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
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
        OpenaiGpt52Chat {
            model_name: "openai/gpt-5.2-chat",
            constructor_name: openai_gpt_5_2_chat,
            display_name: "GPT 5.2 Chat",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt52Codex {
            model_name: "openai/gpt-5.2-codex",
            constructor_name: openai_gpt_5_2_codex,
            display_name: "GPT 5.2 Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Pro {
            model_name: "openai/gpt-5.2-pro",
            constructor_name: openai_gpt_5_2_pro,
            display_name: "GPT 5.2 Pro",
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
            capabilities: [TextInputSupport, TextOutputSupport]
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
        QwenImage {
            model_name: "qwen-image",
            constructor_name: qwen_image,
            display_name: "Qwen Image",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
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
        QwenQwen35397bA17b {
            model_name: "qwen/qwen3.5-397b-a17b",
            constructor_name: qwen_qwen3_5_397b_a17b,
            display_name: "Qwen3.5 397B A17B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, VideoInputSupport]
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
        Qwen36MaxPreview {
            model_name: "qwen3.6-max-preview",
            constructor_name: qwen3_6_max_preview,
            display_name: "Qwen3.6 Max Preview",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwq32b {
            model_name: "qwq-32b",
            constructor_name: qwq_32b,
            display_name: "Qwen: QwQ 32B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        RaifleSorcererlm8x22b {
            model_name: "raifle/sorcererlm-8x22b",
            constructor_name: raifle_sorcererlm_8x22b,
            display_name: "SorcererLM 8x22B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        SarvanMedium {
            model_name: "sarvan-medium",
            constructor_name: sarvan_medium,
            display_name: "Sarvam Medium",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        StepfunAiStep35FlashThinking {
            model_name: "stepfun-ai/step-3.5-flash:thinking",
            constructor_name: stepfun_ai_step_3_5_flash_thinking,
            display_name: "Step 3.5 Flash Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        StudyGptChatgpt4oLatest {
            model_name: "study_gpt-chatgpt-4o-latest",
            constructor_name: study_gpt_chatgpt_4o_latest,
            display_name: "Study Mode",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        TencentHunyuanMt7b {
            model_name: "tencent/Hunyuan-MT-7B",
            constructor_name: tencent_hunyuan_mt_7b,
            display_name: "Hunyuan MT 7B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TngtechDeepseekTngR1t2Chimera {
            model_name: "tngtech/DeepSeek-TNG-R1T2-Chimera",
            constructor_name: tngtech_deepseek_tng_r1t2_chimera,
            display_name: "DeepSeek TNG R1T2 Chimera",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TngtechTngR1tChimera {
            model_name: "tngtech/tng-r1t-chimera",
            constructor_name: tngtech_tng_r1t_chimera,
            display_name: "TNG R1T Chimera",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        UnslothGemma31bIt {
            model_name: "unsloth/gemma-3-1b-it",
            constructor_name: unsloth_gemma_3_1b_it,
            display_name: "Gemma 3 1B IT",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        V010Md {
            model_name: "v0-1.0-md",
            constructor_name: v0_1_0_md,
            display_name: "v0 1.0 MD",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        V015Lg {
            model_name: "v0-1.5-lg",
            constructor_name: v0_1_5_lg,
            display_name: "v0 1.5 LG",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        V015Md {
            model_name: "v0-1.5-md",
            constructor_name: v0_1_5_md,
            display_name: "v0 1.5 MD",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VeniceUncensored {
            model_name: "venice-uncensored",
            constructor_name: venice_uncensored,
            display_name: "Venice Uncensored",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VeniceUncensoredWeb {
            model_name: "venice-uncensored:web",
            constructor_name: venice_uncensored_web,
            display_name: "Venice Uncensored Web",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        XAiGrok40709 {
            model_name: "x-ai/grok-4-07-09",
            constructor_name: x_ai_grok_4_07_09,
            display_name: "Grok 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        XAiGrok4Fast {
            model_name: "x-ai/grok-4-fast",
            constructor_name: x_ai_grok_4_fast,
            display_name: "Grok 4 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok4FastThinking {
            model_name: "x-ai/grok-4-fast:thinking",
            constructor_name: x_ai_grok_4_fast_thinking,
            display_name: "Grok 4 Fast Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok41Fast {
            model_name: "x-ai/grok-4.1-fast",
            constructor_name: x_ai_grok_4_1_fast,
            display_name: "Grok 4.1 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XAiGrok41FastReasoning {
            model_name: "x-ai/grok-4.1-fast-reasoning",
            constructor_name: x_ai_grok_4_1_fast_reasoning,
            display_name: "Grok 4.1 Fast Reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        XAiGrokCodeFast1 {
            model_name: "x-ai/grok-code-fast-1",
            constructor_name: x_ai_grok_code_fast_1,
            display_name: "Grok Code Fast 1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        XiaomiMimoV2Flash {
            model_name: "xiaomi/mimo-v2-flash",
            constructor_name: xiaomi_mimo_v2_flash,
            display_name: "MiMo V2 Flash",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        XiaomiMimoV2FlashOriginal {
            model_name: "xiaomi/mimo-v2-flash-original",
            constructor_name: xiaomi_mimo_v2_flash_original,
            display_name: "MiMo V2 Flash Original",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        XiaomiMimoV2FlashThinking {
            model_name: "xiaomi/mimo-v2-flash-thinking",
            constructor_name: xiaomi_mimo_v2_flash_thinking,
            display_name: "MiMo V2 Flash (Thinking)",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        XiaomiMimoV2FlashThinkingOriginal {
            model_name: "xiaomi/mimo-v2-flash-thinking-original",
            constructor_name: xiaomi_mimo_v2_flash_thinking_original,
            display_name: "MiMo V2 Flash (Thinking) Original",
            capabilities: [TextInputSupport, TextOutputSupport]
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
        ZImageTurbo {
            model_name: "z-image-turbo",
            constructor_name: z_image_turbo,
            display_name: "Z Image Turbo",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
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
        ZaiOrgGlm5 {
            model_name: "zai-org/glm-5",
            constructor_name: zai_org_glm_5,
            display_name: "GLM 5",
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
        ZaiOrgGlm5Thinking {
            model_name: "zai-org/glm-5:thinking",
            constructor_name: zai_org_glm_5_thinking,
            display_name: "GLM 5 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
