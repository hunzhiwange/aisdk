//! Capabilities for amazon_bedrock models.
//!
//! This module defines model types and their capabilities for amazon_bedrock providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::amazon_bedrock::AmazonBedrock;

model_capabilities! {
    provider: AmazonBedrock,
    models: {
        AmazonNova2LiteV10 {
            model_name: "amazon.nova-2-lite-v1:0",
            constructor_name: amazon_nova_2_lite_v1_0,
            display_name: "Nova 2 Lite",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AmazonNovaLiteV10 {
            model_name: "amazon.nova-lite-v1:0",
            constructor_name: amazon_nova_lite_v1_0,
            display_name: "Nova Lite",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AmazonNovaMicroV10 {
            model_name: "amazon.nova-micro-v1:0",
            constructor_name: amazon_nova_micro_v1_0,
            display_name: "Nova Micro",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AmazonNovaProV10 {
            model_name: "amazon.nova-pro-v1:0",
            constructor_name: amazon_nova_pro_v1_0,
            display_name: "Nova Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AnthropicClaudeHaiku4520251001V10 {
            model_name: "anthropic.claude-haiku-4-5-20251001-v1:0",
            constructor_name: anthropic_claude_haiku_4_5_20251001_v1_0,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus4120250805V10 {
            model_name: "anthropic.claude-opus-4-1-20250805-v1:0",
            constructor_name: anthropic_claude_opus_4_1_20250805_v1_0,
            display_name: "Claude Opus 4.1",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus4520251101V10 {
            model_name: "anthropic.claude-opus-4-5-20251101-v1:0",
            constructor_name: anthropic_claude_opus_4_5_20251101_v1_0,
            display_name: "Claude Opus 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46V1 {
            model_name: "anthropic.claude-opus-4-6-v1",
            constructor_name: anthropic_claude_opus_4_6_v1,
            display_name: "Claude Opus 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus47 {
            model_name: "anthropic.claude-opus-4-7",
            constructor_name: anthropic_claude_opus_4_7,
            display_name: "Claude Opus 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet4520250929V10 {
            model_name: "anthropic.claude-sonnet-4-5-20250929-v1:0",
            constructor_name: anthropic_claude_sonnet_4_5_20250929_v1_0,
            display_name: "Claude Sonnet 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet46 {
            model_name: "anthropic.claude-sonnet-4-6",
            constructor_name: anthropic_claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AuAnthropicClaudeHaiku4520251001V10 {
            model_name: "au.anthropic.claude-haiku-4-5-20251001-v1:0",
            constructor_name: au_anthropic_claude_haiku_4_5_20251001_v1_0,
            display_name: "Claude Haiku 4.5 (AU)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AuAnthropicClaudeOpus46V1 {
            model_name: "au.anthropic.claude-opus-4-6-v1",
            constructor_name: au_anthropic_claude_opus_4_6_v1,
            display_name: "AU Anthropic Claude Opus 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AuAnthropicClaudeSonnet4520250929V10 {
            model_name: "au.anthropic.claude-sonnet-4-5-20250929-v1:0",
            constructor_name: au_anthropic_claude_sonnet_4_5_20250929_v1_0,
            display_name: "Claude Sonnet 4.5 (AU)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AuAnthropicClaudeSonnet46 {
            model_name: "au.anthropic.claude-sonnet-4-6",
            constructor_name: au_anthropic_claude_sonnet_4_6,
            display_name: "AU Anthropic Claude Sonnet 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekR1V10 {
            model_name: "deepseek.r1-v1:0",
            constructor_name: deepseek_r1_v1_0,
            display_name: "DeepSeek-R1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV3V10 {
            model_name: "deepseek.v3-v1:0",
            constructor_name: deepseek_v3_v1_0,
            display_name: "DeepSeek-V3.1",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV32 {
            model_name: "deepseek.v3.2",
            constructor_name: deepseek_v3_2,
            display_name: "DeepSeek-V3.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        EuAnthropicClaudeHaiku4520251001V10 {
            model_name: "eu.anthropic.claude-haiku-4-5-20251001-v1:0",
            constructor_name: eu_anthropic_claude_haiku_4_5_20251001_v1_0,
            display_name: "Claude Haiku 4.5 (EU)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        EuAnthropicClaudeOpus4520251101V10 {
            model_name: "eu.anthropic.claude-opus-4-5-20251101-v1:0",
            constructor_name: eu_anthropic_claude_opus_4_5_20251101_v1_0,
            display_name: "Claude Opus 4.5 (EU)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        EuAnthropicClaudeOpus46V1 {
            model_name: "eu.anthropic.claude-opus-4-6-v1",
            constructor_name: eu_anthropic_claude_opus_4_6_v1,
            display_name: "Claude Opus 4.6 (EU)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        EuAnthropicClaudeOpus47 {
            model_name: "eu.anthropic.claude-opus-4-7",
            constructor_name: eu_anthropic_claude_opus_4_7,
            display_name: "Claude Opus 4.7 (EU)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        EuAnthropicClaudeSonnet4520250929V10 {
            model_name: "eu.anthropic.claude-sonnet-4-5-20250929-v1:0",
            constructor_name: eu_anthropic_claude_sonnet_4_5_20250929_v1_0,
            display_name: "Claude Sonnet 4.5 (EU)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        EuAnthropicClaudeSonnet46 {
            model_name: "eu.anthropic.claude-sonnet-4-6",
            constructor_name: eu_anthropic_claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6 (EU)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GlobalAnthropicClaudeHaiku4520251001V10 {
            model_name: "global.anthropic.claude-haiku-4-5-20251001-v1:0",
            constructor_name: global_anthropic_claude_haiku_4_5_20251001_v1_0,
            display_name: "Claude Haiku 4.5 (Global)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GlobalAnthropicClaudeOpus4520251101V10 {
            model_name: "global.anthropic.claude-opus-4-5-20251101-v1:0",
            constructor_name: global_anthropic_claude_opus_4_5_20251101_v1_0,
            display_name: "Claude Opus 4.5 (Global)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GlobalAnthropicClaudeOpus46V1 {
            model_name: "global.anthropic.claude-opus-4-6-v1",
            constructor_name: global_anthropic_claude_opus_4_6_v1,
            display_name: "Claude Opus 4.6 (Global)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GlobalAnthropicClaudeOpus47 {
            model_name: "global.anthropic.claude-opus-4-7",
            constructor_name: global_anthropic_claude_opus_4_7,
            display_name: "Claude Opus 4.7 (Global)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GlobalAnthropicClaudeSonnet4520250929V10 {
            model_name: "global.anthropic.claude-sonnet-4-5-20250929-v1:0",
            constructor_name: global_anthropic_claude_sonnet_4_5_20250929_v1_0,
            display_name: "Claude Sonnet 4.5 (Global)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GlobalAnthropicClaudeSonnet46 {
            model_name: "global.anthropic.claude-sonnet-4-6",
            constructor_name: global_anthropic_claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6 (Global)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma312bIt {
            model_name: "google.gemma-3-12b-it",
            constructor_name: google_gemma_3_12b_it,
            display_name: "Google Gemma 3 12B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma327bIt {
            model_name: "google.gemma-3-27b-it",
            constructor_name: google_gemma_3_27b_it,
            display_name: "Google Gemma 3 27B Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma34bIt {
            model_name: "google.gemma-3-4b-it",
            constructor_name: google_gemma_3_4b_it,
            display_name: "Gemma 3 4B IT",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        JpAnthropicClaudeOpus47 {
            model_name: "jp.anthropic.claude-opus-4-7",
            constructor_name: jp_anthropic_claude_opus_4_7,
            display_name: "Claude Opus 4.7 (JP)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        JpAnthropicClaudeSonnet4520250929V10 {
            model_name: "jp.anthropic.claude-sonnet-4-5-20250929-v1:0",
            constructor_name: jp_anthropic_claude_sonnet_4_5_20250929_v1_0,
            display_name: "Claude Sonnet 4.5 (JP)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        JpAnthropicClaudeSonnet46 {
            model_name: "jp.anthropic.claude-sonnet-4-6",
            constructor_name: jp_anthropic_claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6 (JP)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3170bInstructV10 {
            model_name: "meta.llama3-1-70b-instruct-v1:0",
            constructor_name: meta_llama3_1_70b_instruct_v1_0,
            display_name: "Llama 3.1 70B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama318bInstructV10 {
            model_name: "meta.llama3-1-8b-instruct-v1:0",
            constructor_name: meta_llama3_1_8b_instruct_v1_0,
            display_name: "Llama 3.1 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3370bInstructV10 {
            model_name: "meta.llama3-3-70b-instruct-v1:0",
            constructor_name: meta_llama3_3_70b_instruct_v1_0,
            display_name: "Llama 3.3 70B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama4Maverick17bInstructV10 {
            model_name: "meta.llama4-maverick-17b-instruct-v1:0",
            constructor_name: meta_llama4_maverick_17b_instruct_v1_0,
            display_name: "Llama 4 Maverick 17B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama4Scout17bInstructV10 {
            model_name: "meta.llama4-scout-17b-instruct-v1:0",
            constructor_name: meta_llama4_scout_17b_instruct_v1_0,
            display_name: "Llama 4 Scout 17B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM2 {
            model_name: "minimax.minimax-m2",
            constructor_name: minimax_minimax_m2,
            display_name: "MiniMax M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM21 {
            model_name: "minimax.minimax-m2.1",
            constructor_name: minimax_minimax_m2_1,
            display_name: "MiniMax M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM25 {
            model_name: "minimax.minimax-m2.5",
            constructor_name: minimax_minimax_m2_5,
            display_name: "MiniMax M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralDevstral2123b {
            model_name: "mistral.devstral-2-123b",
            constructor_name: mistral_devstral_2_123b,
            display_name: "Devstral 2 123B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMagistralSmall2509 {
            model_name: "mistral.magistral-small-2509",
            constructor_name: mistral_magistral_small_2509,
            display_name: "Magistral Small 1.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMinistral314bInstruct {
            model_name: "mistral.ministral-3-14b-instruct",
            constructor_name: mistral_ministral_3_14b_instruct,
            display_name: "Ministral 14B 3.0",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMinistral33bInstruct {
            model_name: "mistral.ministral-3-3b-instruct",
            constructor_name: mistral_ministral_3_3b_instruct,
            display_name: "Ministral 3 3B",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMinistral38bInstruct {
            model_name: "mistral.ministral-3-8b-instruct",
            constructor_name: mistral_ministral_3_8b_instruct,
            display_name: "Ministral 3 8B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistralLarge3675bInstruct {
            model_name: "mistral.mistral-large-3-675b-instruct",
            constructor_name: mistral_mistral_large_3_675b_instruct,
            display_name: "Mistral Large 3",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralPixtralLarge2502V10 {
            model_name: "mistral.pixtral-large-2502-v1:0",
            constructor_name: mistral_pixtral_large_2502_v1_0,
            display_name: "Pixtral Large (25.02)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralVoxtralMini3b2507 {
            model_name: "mistral.voxtral-mini-3b-2507",
            constructor_name: mistral_voxtral_mini_3b_2507,
            display_name: "Voxtral Mini 3B 2507",
            capabilities: [AudioInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralVoxtralSmall24b2507 {
            model_name: "mistral.voxtral-small-24b-2507",
            constructor_name: mistral_voxtral_small_24b_2507,
            display_name: "Voxtral Small 24B 2507",
            capabilities: [AudioInputSupport, ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotKimiK2Thinking {
            model_name: "moonshot.kimi-k2-thinking",
            constructor_name: moonshot_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai.kimi-k2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronNano12bV2 {
            model_name: "nvidia.nemotron-nano-12b-v2",
            constructor_name: nvidia_nemotron_nano_12b_v2,
            display_name: "NVIDIA Nemotron Nano 12B v2 VL BF16",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronNano330b {
            model_name: "nvidia.nemotron-nano-3-30b",
            constructor_name: nvidia_nemotron_nano_3_30b,
            display_name: "NVIDIA Nemotron Nano 3 30B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronNano9bV2 {
            model_name: "nvidia.nemotron-nano-9b-v2",
            constructor_name: nvidia_nemotron_nano_9b_v2,
            display_name: "NVIDIA Nemotron Nano 9B v2",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronSuper3120b {
            model_name: "nvidia.nemotron-super-3-120b",
            constructor_name: nvidia_nemotron_super_3_120b,
            display_name: "NVIDIA Nemotron 3 Super 120B A12B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss120b10 {
            model_name: "openai.gpt-oss-120b-1:0",
            constructor_name: openai_gpt_oss_120b_1_0,
            display_name: "gpt-oss-120b",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOss20b10 {
            model_name: "openai.gpt-oss-20b-1:0",
            constructor_name: openai_gpt_oss_20b_1_0,
            display_name: "gpt-oss-20b",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOssSafeguard120b {
            model_name: "openai.gpt-oss-safeguard-120b",
            constructor_name: openai_gpt_oss_safeguard_120b,
            display_name: "GPT OSS Safeguard 120B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptOssSafeguard20b {
            model_name: "openai.gpt-oss-safeguard-20b",
            constructor_name: openai_gpt_oss_safeguard_20b,
            display_name: "GPT OSS Safeguard 20B",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3235bA22b2507V10 {
            model_name: "qwen.qwen3-235b-a22b-2507-v1:0",
            constructor_name: qwen_qwen3_235b_a22b_2507_v1_0,
            display_name: "Qwen3 235B A22B 2507",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen332bV10 {
            model_name: "qwen.qwen3-32b-v1:0",
            constructor_name: qwen_qwen3_32b_v1_0,
            display_name: "Qwen3 32B (dense)",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder30bA3bV10 {
            model_name: "qwen.qwen3-coder-30b-a3b-v1:0",
            constructor_name: qwen_qwen3_coder_30b_a3b_v1_0,
            display_name: "Qwen3 Coder 30B A3B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Coder480bA35bV10 {
            model_name: "qwen.qwen3-coder-480b-a35b-v1:0",
            constructor_name: qwen_qwen3_coder_480b_a35b_v1_0,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3CoderNext {
            model_name: "qwen.qwen3-coder-next",
            constructor_name: qwen_qwen3_coder_next,
            display_name: "Qwen3 Coder Next",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Next80bA3b {
            model_name: "qwen.qwen3-next-80b-a3b",
            constructor_name: qwen_qwen3_next_80b_a3b,
            display_name: "Qwen/Qwen3-Next-80B-A3B-Instruct",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3Vl235bA22b {
            model_name: "qwen.qwen3-vl-235b-a22b",
            constructor_name: qwen_qwen3_vl_235b_a22b,
            display_name: "Qwen/Qwen3-VL-235B-A22B-Instruct",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UsAnthropicClaudeHaiku4520251001V10 {
            model_name: "us.anthropic.claude-haiku-4-5-20251001-v1:0",
            constructor_name: us_anthropic_claude_haiku_4_5_20251001_v1_0,
            display_name: "Claude Haiku 4.5 (US)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UsAnthropicClaudeOpus4120250805V10 {
            model_name: "us.anthropic.claude-opus-4-1-20250805-v1:0",
            constructor_name: us_anthropic_claude_opus_4_1_20250805_v1_0,
            display_name: "Claude Opus 4.1 (US)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UsAnthropicClaudeOpus4520251101V10 {
            model_name: "us.anthropic.claude-opus-4-5-20251101-v1:0",
            constructor_name: us_anthropic_claude_opus_4_5_20251101_v1_0,
            display_name: "Claude Opus 4.5 (US)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UsAnthropicClaudeOpus46V1 {
            model_name: "us.anthropic.claude-opus-4-6-v1",
            constructor_name: us_anthropic_claude_opus_4_6_v1,
            display_name: "Claude Opus 4.6 (US)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UsAnthropicClaudeOpus47 {
            model_name: "us.anthropic.claude-opus-4-7",
            constructor_name: us_anthropic_claude_opus_4_7,
            display_name: "Claude Opus 4.7 (US)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UsAnthropicClaudeSonnet4520250929V10 {
            model_name: "us.anthropic.claude-sonnet-4-5-20250929-v1:0",
            constructor_name: us_anthropic_claude_sonnet_4_5_20250929_v1_0,
            display_name: "Claude Sonnet 4.5 (US)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UsAnthropicClaudeSonnet46 {
            model_name: "us.anthropic.claude-sonnet-4-6",
            constructor_name: us_anthropic_claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6 (US)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UsDeepseekR1V10 {
            model_name: "us.deepseek.r1-v1:0",
            constructor_name: us_deepseek_r1_v1_0,
            display_name: "DeepSeek-R1 (US)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UsMetaLlama4Maverick17bInstructV10 {
            model_name: "us.meta.llama4-maverick-17b-instruct-v1:0",
            constructor_name: us_meta_llama4_maverick_17b_instruct_v1_0,
            display_name: "Llama 4 Maverick 17B Instruct (US)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        UsMetaLlama4Scout17bInstructV10 {
            model_name: "us.meta.llama4-scout-17b-instruct-v1:0",
            constructor_name: us_meta_llama4_scout_17b_instruct_v1_0,
            display_name: "Llama 4 Scout 17B Instruct (US)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        WriterPalmyraX4V10 {
            model_name: "writer.palmyra-x4-v1:0",
            constructor_name: writer_palmyra_x4_v1_0,
            display_name: "Palmyra X4",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        WriterPalmyraX5V10 {
            model_name: "writer.palmyra-x5-v1:0",
            constructor_name: writer_palmyra_x5_v1_0,
            display_name: "Palmyra X5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm47 {
            model_name: "zai.glm-4.7",
            constructor_name: zai_glm_4_7,
            display_name: "GLM-4.7",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm47Flash {
            model_name: "zai.glm-4.7-flash",
            constructor_name: zai_glm_4_7_flash,
            display_name: "GLM-4.7-Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm5 {
            model_name: "zai.glm-5",
            constructor_name: zai_glm_5,
            display_name: "GLM-5",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
