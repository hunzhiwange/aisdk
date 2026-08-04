//! Capabilities for vercel models.
//!
//! This module defines model types and their capabilities for vercel providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::vercel::Vercel;

model_capabilities! {
    provider: Vercel,
    models: {
        AlibabaQwen314b {
            model_name: "alibaba/qwen-3-14b",
            constructor_name: alibaba_qwen_3_14b,
            display_name: "Qwen3-14B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3235b {
            model_name: "alibaba/qwen-3-235b",
            constructor_name: alibaba_qwen_3_235b,
            display_name: "Qwen3 235B A22B Instruct 2507",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen330b {
            model_name: "alibaba/qwen-3-30b",
            constructor_name: alibaba_qwen_3_30b,
            display_name: "Qwen3-30B-A3B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen332b {
            model_name: "alibaba/qwen-3-32b",
            constructor_name: alibaba_qwen_3_32b,
            display_name: "Qwen 3.32B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen36MaxPreview {
            model_name: "alibaba/qwen-3.6-max-preview",
            constructor_name: alibaba_qwen_3_6_max_preview,
            display_name: "Qwen 3.6 Max Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3235bA22bThinking {
            model_name: "alibaba/qwen3-235b-a22b-thinking",
            constructor_name: alibaba_qwen3_235b_a22b_thinking,
            display_name: "Qwen3 235B A22B Thinking 2507",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Coder {
            model_name: "alibaba/qwen3-coder",
            constructor_name: alibaba_qwen3_coder,
            display_name: "Qwen3 Coder 480B A35B Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Coder30bA3b {
            model_name: "alibaba/qwen3-coder-30b-a3b",
            constructor_name: alibaba_qwen3_coder_30b_a3b,
            display_name: "Qwen 3 Coder 30B A3B Instruct",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3CoderNext {
            model_name: "alibaba/qwen3-coder-next",
            constructor_name: alibaba_qwen3_coder_next,
            display_name: "Qwen3 Coder Next",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3CoderPlus {
            model_name: "alibaba/qwen3-coder-plus",
            constructor_name: alibaba_qwen3_coder_plus,
            display_name: "Qwen3 Coder Plus",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Embedding06b {
            model_name: "alibaba/qwen3-embedding-0.6b",
            constructor_name: alibaba_qwen3_embedding_0_6b,
            display_name: "Qwen3 Embedding 0.6B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AlibabaQwen3Embedding4b {
            model_name: "alibaba/qwen3-embedding-4b",
            constructor_name: alibaba_qwen3_embedding_4b,
            display_name: "Qwen3 Embedding 4B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AlibabaQwen3Embedding8b {
            model_name: "alibaba/qwen3-embedding-8b",
            constructor_name: alibaba_qwen3_embedding_8b,
            display_name: "Qwen3 Embedding 8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AlibabaQwen3Max {
            model_name: "alibaba/qwen3-max",
            constructor_name: alibaba_qwen3_max,
            display_name: "Qwen3 Max",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3MaxPreview {
            model_name: "alibaba/qwen3-max-preview",
            constructor_name: alibaba_qwen3_max_preview,
            display_name: "Qwen3 Max Preview",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3MaxThinking {
            model_name: "alibaba/qwen3-max-thinking",
            constructor_name: alibaba_qwen3_max_thinking,
            display_name: "Qwen 3 Max Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Next80bA3bInstruct {
            model_name: "alibaba/qwen3-next-80b-a3b-instruct",
            constructor_name: alibaba_qwen3_next_80b_a3b_instruct,
            display_name: "Qwen3 Next 80B A3B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Next80bA3bThinking {
            model_name: "alibaba/qwen3-next-80b-a3b-thinking",
            constructor_name: alibaba_qwen3_next_80b_a3b_thinking,
            display_name: "Qwen3 Next 80B A3B Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3Vl235bA22bInstruct {
            model_name: "alibaba/qwen3-vl-235b-a22b-instruct",
            constructor_name: alibaba_qwen3_vl_235b_a22b_instruct,
            display_name: "Qwen3 VL 235B A22B Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        AlibabaQwen3VlInstruct {
            model_name: "alibaba/qwen3-vl-instruct",
            constructor_name: alibaba_qwen3_vl_instruct,
            display_name: "Qwen3 VL Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3VlThinking {
            model_name: "alibaba/qwen3-vl-thinking",
            constructor_name: alibaba_qwen3_vl_thinking,
            display_name: "Qwen3 VL Thinking",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen35Flash {
            model_name: "alibaba/qwen3.5-flash",
            constructor_name: alibaba_qwen3_5_flash,
            display_name: "Qwen 3.5 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen35Plus {
            model_name: "alibaba/qwen3.5-plus",
            constructor_name: alibaba_qwen3_5_plus,
            display_name: "Qwen 3.5 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen3627b {
            model_name: "alibaba/qwen3.6-27b",
            constructor_name: alibaba_qwen3_6_27b,
            display_name: "Qwen 3.6 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen36Plus {
            model_name: "alibaba/qwen3.6-plus",
            constructor_name: alibaba_qwen3_6_plus,
            display_name: "Qwen 3.6 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen37Flash {
            model_name: "alibaba/qwen3.7-flash",
            constructor_name: alibaba_qwen3_7_flash,
            display_name: "Qwen 3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen37Max {
            model_name: "alibaba/qwen3.7-max",
            constructor_name: alibaba_qwen3_7_max,
            display_name: "Qwen 3.7 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen37Plus {
            model_name: "alibaba/qwen3.7-plus",
            constructor_name: alibaba_qwen3_7_plus,
            display_name: "Qwen 3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaQwen38Max {
            model_name: "alibaba/qwen3.8-max",
            constructor_name: alibaba_qwen3_8_max,
            display_name: "Qwen 3.8 Max",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AlibabaWanV25T2vPreview {
            model_name: "alibaba/wan-v2.5-t2v-preview",
            constructor_name: alibaba_wan_v2_5_t2v_preview,
            display_name: "Wan v2.5 Text-to-Video Preview",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        AlibabaWanV26I2v {
            model_name: "alibaba/wan-v2.6-i2v",
            constructor_name: alibaba_wan_v2_6_i2v,
            display_name: "Wan v2.6 Image-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        AlibabaWanV26I2vFlash {
            model_name: "alibaba/wan-v2.6-i2v-flash",
            constructor_name: alibaba_wan_v2_6_i2v_flash,
            display_name: "Wan v2.6 Image-to-Video Flash",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        AlibabaWanV26R2v {
            model_name: "alibaba/wan-v2.6-r2v",
            constructor_name: alibaba_wan_v2_6_r2v,
            display_name: "Wan v2.6 Reference-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        AlibabaWanV26R2vFlash {
            model_name: "alibaba/wan-v2.6-r2v-flash",
            constructor_name: alibaba_wan_v2_6_r2v_flash,
            display_name: "Wan v2.6 Reference-to-Video Flash",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        AlibabaWanV26T2v {
            model_name: "alibaba/wan-v2.6-t2v",
            constructor_name: alibaba_wan_v2_6_t2v,
            display_name: "Wan v2.6 Text-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        AlibabaWanV27R2v {
            model_name: "alibaba/wan-v2.7-r2v",
            constructor_name: alibaba_wan_v2_7_r2v,
            display_name: "Wan v2.7 Reference-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        AlibabaWanV27T2v {
            model_name: "alibaba/wan-v2.7-t2v",
            constructor_name: alibaba_wan_v2_7_t2v,
            display_name: "Wan v2.7 Text-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        AmazonNova2Lite {
            model_name: "amazon/nova-2-lite",
            constructor_name: amazon_nova_2_lite,
            display_name: "Nova 2 Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        AmazonNovaLite {
            model_name: "amazon/nova-lite",
            constructor_name: amazon_nova_lite,
            display_name: "Nova Lite",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AmazonNovaMicro {
            model_name: "amazon/nova-micro",
            constructor_name: amazon_nova_micro,
            display_name: "Nova Micro",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AmazonNovaPro {
            model_name: "amazon/nova-pro",
            constructor_name: amazon_nova_pro,
            display_name: "Nova Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AmazonTitanEmbedTextV2 {
            model_name: "amazon/titan-embed-text-v2",
            constructor_name: amazon_titan_embed_text_v2,
            display_name: "Titan Text Embeddings V2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        AnthropicClaude3Haiku {
            model_name: "anthropic/claude-3-haiku",
            constructor_name: anthropic_claude_3_haiku,
            display_name: "Claude Haiku 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeFable5 {
            model_name: "anthropic/claude-fable-5",
            constructor_name: anthropic_claude_fable_5,
            display_name: "Claude Fable 5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeHaiku45 {
            model_name: "anthropic/claude-haiku-4.5",
            constructor_name: anthropic_claude_haiku_4_5,
            display_name: "Claude Haiku 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus4 {
            model_name: "anthropic/claude-opus-4",
            constructor_name: anthropic_claude_opus_4,
            display_name: "Claude Opus 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus45 {
            model_name: "anthropic/claude-opus-4.5",
            constructor_name: anthropic_claude_opus_4_5,
            display_name: "Claude Opus 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus46 {
            model_name: "anthropic/claude-opus-4.6",
            constructor_name: anthropic_claude_opus_4_6,
            display_name: "Claude Opus 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus47 {
            model_name: "anthropic/claude-opus-4.7",
            constructor_name: anthropic_claude_opus_4_7,
            display_name: "Claude Opus 4.7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus48 {
            model_name: "anthropic/claude-opus-4.8",
            constructor_name: anthropic_claude_opus_4_8,
            display_name: "Claude Opus 4.8",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus48Fast {
            model_name: "anthropic/claude-opus-4.8-fast",
            constructor_name: anthropic_claude_opus_4_8_fast,
            display_name: "Claude Opus 4.8 (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus5 {
            model_name: "anthropic/claude-opus-5",
            constructor_name: anthropic_claude_opus_5,
            display_name: "Claude Opus 5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeOpus5Fast {
            model_name: "anthropic/claude-opus-5-fast",
            constructor_name: anthropic_claude_opus_5_fast,
            display_name: "Claude Opus 5 (Fast)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet4 {
            model_name: "anthropic/claude-sonnet-4",
            constructor_name: anthropic_claude_sonnet_4,
            display_name: "Claude Sonnet 4",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet45 {
            model_name: "anthropic/claude-sonnet-4.5",
            constructor_name: anthropic_claude_sonnet_4_5,
            display_name: "Claude Sonnet 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet46 {
            model_name: "anthropic/claude-sonnet-4.6",
            constructor_name: anthropic_claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AnthropicClaudeSonnet5 {
            model_name: "anthropic/claude-sonnet-5",
            constructor_name: anthropic_claude_sonnet_5,
            display_name: "Claude Sonnet 5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ArceeAiTrinityLargeThinking {
            model_name: "arcee-ai/trinity-large-thinking",
            constructor_name: arcee_ai_trinity_large_thinking,
            display_name: "Trinity Large Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ArceeAiTrinityMini {
            model_name: "arcee-ai/trinity-mini",
            constructor_name: arcee_ai_trinity_mini,
            display_name: "Trinity Mini",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        BflFlux2Flex {
            model_name: "bfl/flux-2-flex",
            constructor_name: bfl_flux_2_flex,
            display_name: "FLUX.2 [flex]",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFlux2Klein4b {
            model_name: "bfl/flux-2-klein-4b",
            constructor_name: bfl_flux_2_klein_4b,
            display_name: "FLUX.2 [klein] 4B",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFlux2Klein9b {
            model_name: "bfl/flux-2-klein-9b",
            constructor_name: bfl_flux_2_klein_9b,
            display_name: "FLUX.2 [klein] 9B",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFlux2Max {
            model_name: "bfl/flux-2-max",
            constructor_name: bfl_flux_2_max,
            display_name: "FLUX.2 [max]",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFlux2Pro {
            model_name: "bfl/flux-2-pro",
            constructor_name: bfl_flux_2_pro,
            display_name: "FLUX.2 [pro]",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFluxKontextMax {
            model_name: "bfl/flux-kontext-max",
            constructor_name: bfl_flux_kontext_max,
            display_name: "FLUX.1 Kontext Max",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFluxKontextPro {
            model_name: "bfl/flux-kontext-pro",
            constructor_name: bfl_flux_kontext_pro,
            display_name: "FLUX.1 Kontext Pro",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFluxPro10Fill {
            model_name: "bfl/flux-pro-1.0-fill",
            constructor_name: bfl_flux_pro_1_0_fill,
            display_name: "FLUX.1 Fill [pro]",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFluxPro11 {
            model_name: "bfl/flux-pro-1.1",
            constructor_name: bfl_flux_pro_1_1,
            display_name: "FLUX1.1 [pro]",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BflFluxPro11Ultra {
            model_name: "bfl/flux-pro-1.1-ultra",
            constructor_name: bfl_flux_pro_1_1_ultra,
            display_name: "FLUX1.1 [pro] Ultra",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BytedanceSeed16 {
            model_name: "bytedance/seed-1.6",
            constructor_name: bytedance_seed_1_6,
            display_name: "Seed 1.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BytedanceSeed18 {
            model_name: "bytedance/seed-1.8",
            constructor_name: bytedance_seed_1_8,
            display_name: "Seed 1.8",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        BytedanceSeedance20 {
            model_name: "bytedance/seedance-2.0",
            constructor_name: bytedance_seedance_2_0,
            display_name: "Seedance 2.0",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        BytedanceSeedance20Fast {
            model_name: "bytedance/seedance-2.0-fast",
            constructor_name: bytedance_seedance_2_0_fast,
            display_name: "Seedance 2.0 Fast",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        BytedanceSeedanceV10Pro {
            model_name: "bytedance/seedance-v1.0-pro",
            constructor_name: bytedance_seedance_v1_0_pro,
            display_name: "Seedance v1.0 Pro",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        BytedanceSeedanceV10ProFast {
            model_name: "bytedance/seedance-v1.0-pro-fast",
            constructor_name: bytedance_seedance_v1_0_pro_fast,
            display_name: "Seedance v1.0 Pro Fast",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        BytedanceSeedanceV15Pro {
            model_name: "bytedance/seedance-v1.5-pro",
            constructor_name: bytedance_seedance_v1_5_pro,
            display_name: "Seedance v1.5 Pro",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        BytedanceSeedream40 {
            model_name: "bytedance/seedream-4.0",
            constructor_name: bytedance_seedream_4_0,
            display_name: "Seedream 4.0",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BytedanceSeedream45 {
            model_name: "bytedance/seedream-4.5",
            constructor_name: bytedance_seedream_4_5,
            display_name: "Seedream 4.5",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BytedanceSeedream50Lite {
            model_name: "bytedance/seedream-5.0-lite",
            constructor_name: bytedance_seedream_5_0_lite,
            display_name: "Seedream 5.0 Lite",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        BytedanceSeedream50Pro {
            model_name: "bytedance/seedream-5.0-pro",
            constructor_name: bytedance_seedream_5_0_pro,
            display_name: "Seedream 5.0 Pro",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        CohereCommandA {
            model_name: "cohere/command-a",
            constructor_name: cohere_command_a,
            display_name: "Command A",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CohereEmbedV40 {
            model_name: "cohere/embed-v4.0",
            constructor_name: cohere_embed_v4_0,
            display_name: "Embed v4.0",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereRerankV35 {
            model_name: "cohere/rerank-v3.5",
            constructor_name: cohere_rerank_v3_5,
            display_name: "Cohere Rerank 3.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereRerankV4Fast {
            model_name: "cohere/rerank-v4-fast",
            constructor_name: cohere_rerank_v4_fast,
            display_name: "Cohere Rerank 4 Fast",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        CohereRerankV4Pro {
            model_name: "cohere/rerank-v4-pro",
            constructor_name: cohere_rerank_v4_pro,
            display_name: "Cohere Rerank 4 Pro",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekR1 {
            model_name: "deepseek/deepseek-r1",
            constructor_name: deepseek_deepseek_r1,
            display_name: "DeepSeek-R1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV3 {
            model_name: "deepseek/deepseek-v3",
            constructor_name: deepseek_deepseek_v3,
            display_name: "DeepSeek V3 0324",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV31 {
            model_name: "deepseek/deepseek-v3.1",
            constructor_name: deepseek_deepseek_v3_1,
            display_name: "DeepSeek-V3.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV31Terminus {
            model_name: "deepseek/deepseek-v3.1-terminus",
            constructor_name: deepseek_deepseek_v3_1_terminus,
            display_name: "DeepSeek V3.1 Terminus",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV32 {
            model_name: "deepseek/deepseek-v3.2",
            constructor_name: deepseek_deepseek_v3_2,
            display_name: "DeepSeek V3.2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        DeepseekDeepseekV32Thinking {
            model_name: "deepseek/deepseek-v3.2-thinking",
            constructor_name: deepseek_deepseek_v3_2_thinking,
            display_name: "DeepSeek V3.2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        DeepseekDeepseekV4Pro {
            model_name: "deepseek/deepseek-v4-pro",
            constructor_name: deepseek_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini25Flash {
            model_name: "google/gemini-2.5-flash",
            constructor_name: google_gemini_2_5_flash,
            display_name: "Gemini 2.5 Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini25FlashImage {
            model_name: "google/gemini-2.5-flash-image",
            constructor_name: google_gemini_2_5_flash_image,
            display_name: "Nano Banana (Gemini 2.5 Flash Image)",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini25FlashLite {
            model_name: "google/gemini-2.5-flash-lite",
            constructor_name: google_gemini_2_5_flash_lite,
            display_name: "Gemini 2.5 Flash Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini25Pro {
            model_name: "google/gemini-2.5-pro",
            constructor_name: google_gemini_2_5_pro,
            display_name: "Gemini 2.5 Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        GoogleGemini3Flash {
            model_name: "google/gemini-3-flash",
            constructor_name: google_gemini_3_flash,
            display_name: "Gemini 3 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini3ProImage {
            model_name: "google/gemini-3-pro-image",
            constructor_name: google_gemini_3_pro_image,
            display_name: "Nano Banana Pro",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini31FlashImage {
            model_name: "google/gemini-3.1-flash-image",
            constructor_name: google_gemini_3_1_flash_image,
            display_name: "Nano Banana 2",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini31FlashImagePreview {
            model_name: "google/gemini-3.1-flash-image-preview",
            constructor_name: google_gemini_3_1_flash_image_preview,
            display_name: "Gemini 3.1 Flash Image Preview (Nano Banana 2)",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini31FlashLite {
            model_name: "google/gemini-3.1-flash-lite",
            constructor_name: google_gemini_3_1_flash_lite,
            display_name: "Gemini 3.1 Flash Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini31FlashLiteImage {
            model_name: "google/gemini-3.1-flash-lite-image",
            constructor_name: google_gemini_3_1_flash_lite_image,
            display_name: "Gemini 3.1 Flash Lite Image (Nano Banana 2 Lite)",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemini31ProPreview {
            model_name: "google/gemini-3.1-pro-preview",
            constructor_name: google_gemini_3_1_pro_preview,
            display_name: "Gemini 3.1 Pro Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini35Flash {
            model_name: "google/gemini-3.5-flash",
            constructor_name: google_gemini_3_5_flash,
            display_name: "Gemini 3.5 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini35FlashLite {
            model_name: "google/gemini-3.5-flash-lite",
            constructor_name: google_gemini_3_5_flash_lite,
            display_name: "Gemini 3.5 Flash Lite",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemini36Flash {
            model_name: "google/gemini-3.6-flash",
            constructor_name: google_gemini_3_6_flash,
            display_name: "Gemini 3.6 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGeminiEmbedding001 {
            model_name: "google/gemini-embedding-001",
            constructor_name: google_gemini_embedding_001,
            display_name: "Gemini Embedding 001",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleGeminiEmbedding2 {
            model_name: "google/gemini-embedding-2",
            constructor_name: google_gemini_embedding_2,
            display_name: "Gemini Embedding 2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleGeminiOmniFlashPreview {
            model_name: "google/gemini-omni-flash-preview",
            constructor_name: google_gemini_omni_flash_preview,
            display_name: "Gemini Omni Flash Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        GoogleGemma426bA4bIt {
            model_name: "google/gemma-4-26b-a4b-it",
            constructor_name: google_gemma_4_26b_a4b_it,
            display_name: "Gemma 4 26B A4B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma431bIt {
            model_name: "google/gemma-4-31b-it",
            constructor_name: google_gemma_4_31b_it,
            display_name: "Gemma 4 31B IT",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleImagen40FastGenerate001 {
            model_name: "google/imagen-4.0-fast-generate-001",
            constructor_name: google_imagen_4_0_fast_generate_001,
            display_name: "Imagen 4 Fast",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        GoogleImagen40Generate001 {
            model_name: "google/imagen-4.0-generate-001",
            constructor_name: google_imagen_4_0_generate_001,
            display_name: "Imagen 4",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        GoogleImagen40UltraGenerate001 {
            model_name: "google/imagen-4.0-ultra-generate-001",
            constructor_name: google_imagen_4_0_ultra_generate_001,
            display_name: "Imagen 4 Ultra",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        GoogleTextEmbedding005 {
            model_name: "google/text-embedding-005",
            constructor_name: google_text_embedding_005,
            display_name: "Text Embedding 005",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleTextMultilingualEmbedding002 {
            model_name: "google/text-multilingual-embedding-002",
            constructor_name: google_text_multilingual_embedding_002,
            display_name: "Text Multilingual Embedding 002",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        GoogleVeo30FastGenerate001 {
            model_name: "google/veo-3.0-fast-generate-001",
            constructor_name: google_veo_3_0_fast_generate_001,
            display_name: "Veo 3.0 Fast Generate",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        GoogleVeo30Generate001 {
            model_name: "google/veo-3.0-generate-001",
            constructor_name: google_veo_3_0_generate_001,
            display_name: "Veo 3.0",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        GoogleVeo31FastGenerate001 {
            model_name: "google/veo-3.1-fast-generate-001",
            constructor_name: google_veo_3_1_fast_generate_001,
            display_name: "Veo 3.1 Fast Generate",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        GoogleVeo31Generate001 {
            model_name: "google/veo-3.1-generate-001",
            constructor_name: google_veo_3_1_generate_001,
            display_name: "Veo 3.1",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        GoogleVeo31LiteGenerate001 {
            model_name: "google/veo-3.1-lite-generate-001",
            constructor_name: google_veo_3_1_lite_generate_001,
            display_name: "Veo 3.1 Lite Generate",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        InceptionMercury2 {
            model_name: "inception/mercury-2",
            constructor_name: inception_mercury_2,
            display_name: "Mercury 2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InceptionMercuryCoderSmall {
            model_name: "inception/mercury-coder-small",
            constructor_name: inception_mercury_coder_small,
            display_name: "Mercury Coder Small Beta",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InclusionaiLing30FlashFree {
            model_name: "inclusionai/ling-3.0-flash-free",
            constructor_name: inclusionai_ling_3_0_flash_free,
            display_name: "Ling 3.0 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        InterfazeInterfazeBeta {
            model_name: "interfaze/interfaze-beta",
            constructor_name: interfaze_interfaze_beta,
            display_name: "Interfaze Beta",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        KlingaiKlingV25TurboI2v {
            model_name: "klingai/kling-v2.5-turbo-i2v",
            constructor_name: klingai_kling_v2_5_turbo_i2v,
            display_name: "Kling v2.5 Turbo Image-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV25TurboT2v {
            model_name: "klingai/kling-v2.5-turbo-t2v",
            constructor_name: klingai_kling_v2_5_turbo_t2v,
            display_name: "Kling v2.5 Turbo Text-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV26I2v {
            model_name: "klingai/kling-v2.6-i2v",
            constructor_name: klingai_kling_v2_6_i2v,
            display_name: "Kling v2.6 Image-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV26MotionControl {
            model_name: "klingai/kling-v2.6-motion-control",
            constructor_name: klingai_kling_v2_6_motion_control,
            display_name: "Kling v2.6 Motion Control",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV26T2v {
            model_name: "klingai/kling-v2.6-t2v",
            constructor_name: klingai_kling_v2_6_t2v,
            display_name: "Kling v2.6 Text-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV30I2v {
            model_name: "klingai/kling-v3.0-i2v",
            constructor_name: klingai_kling_v3_0_i2v,
            display_name: "Kling v3.0 Image-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV30MotionControl {
            model_name: "klingai/kling-v3.0-motion-control",
            constructor_name: klingai_kling_v3_0_motion_control,
            display_name: "Kling v3.0 Motion Control",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        KlingaiKlingV30T2v {
            model_name: "klingai/kling-v3.0-t2v",
            constructor_name: klingai_kling_v3_0_t2v,
            display_name: "Kling v3.0 Text-to-Video",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        KwaipilotKatCoderAirV25 {
            model_name: "kwaipilot/kat-coder-air-v2.5",
            constructor_name: kwaipilot_kat_coder_air_v2_5,
            display_name: "Kat Coder Air V2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KwaipilotKatCoderProV1 {
            model_name: "kwaipilot/kat-coder-pro-v1",
            constructor_name: kwaipilot_kat_coder_pro_v1,
            display_name: "KAT-Coder-Pro V1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        KwaipilotKatCoderProV2 {
            model_name: "kwaipilot/kat-coder-pro-v2",
            constructor_name: kwaipilot_kat_coder_pro_v2,
            display_name: "Kat Coder Pro V2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KwaipilotKatCoderProV25 {
            model_name: "kwaipilot/kat-coder-pro-v2.5",
            constructor_name: kwaipilot_kat_coder_pro_v2_5,
            display_name: "Kat Coder Pro V2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3170b {
            model_name: "meta/llama-3.1-70b",
            constructor_name: meta_llama_3_1_70b,
            display_name: "Llama 3.1 70B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama318b {
            model_name: "meta/llama-3.1-8b",
            constructor_name: meta_llama_3_1_8b,
            display_name: "Llama 3.1 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama3370b {
            model_name: "meta/llama-3.3-70b",
            constructor_name: meta_llama_3_3_70b,
            display_name: "Llama-3.3-70B-Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama4Maverick {
            model_name: "meta/llama-4-maverick",
            constructor_name: meta_llama_4_maverick,
            display_name: "Llama-4-Maverick-17B-128E-Instruct-FP8",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlama4Scout {
            model_name: "meta/llama-4-scout",
            constructor_name: meta_llama_4_scout,
            display_name: "Llama-4-Scout-17B-16E-Instruct-FP8",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaMuseSpark11 {
            model_name: "meta/muse-spark-1.1",
            constructor_name: meta_muse_spark_1_1,
            display_name: "Muse Spark 1.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxH3 {
            model_name: "minimax/minimax-h3",
            constructor_name: minimax_minimax_h3,
            display_name: "MiniMax H3",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
        MinimaxMinimaxM2 {
            model_name: "minimax/minimax-m2",
            constructor_name: minimax_minimax_m2,
            display_name: "MiniMax M2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM21 {
            model_name: "minimax/minimax-m2.1",
            constructor_name: minimax_minimax_m2_1,
            display_name: "MiniMax M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM21Lightning {
            model_name: "minimax/minimax-m2.1-lightning",
            constructor_name: minimax_minimax_m2_1_lightning,
            display_name: "MiniMax M2.1 Lightning",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM25 {
            model_name: "minimax/minimax-m2.5",
            constructor_name: minimax_minimax_m2_5,
            display_name: "MiniMax M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM25Highspeed {
            model_name: "minimax/minimax-m2.5-highspeed",
            constructor_name: minimax_minimax_m2_5_highspeed,
            display_name: "MiniMax M2.5 High Speed",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM27 {
            model_name: "minimax/minimax-m2.7",
            constructor_name: minimax_minimax_m2_7,
            display_name: "Minimax M2.7",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM27Highspeed {
            model_name: "minimax/minimax-m2.7-highspeed",
            constructor_name: minimax_minimax_m2_7_highspeed,
            display_name: "MiniMax M2.7 High Speed",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM3 {
            model_name: "minimax/minimax-m3",
            constructor_name: minimax_minimax_m3,
            display_name: "MiniMax M3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralCodestral {
            model_name: "mistral/codestral",
            constructor_name: mistral_codestral,
            display_name: "Codestral (latest)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralCodestralEmbed {
            model_name: "mistral/codestral-embed",
            constructor_name: mistral_codestral_embed,
            display_name: "Codestral Embed",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralDevstral2 {
            model_name: "mistral/devstral-2",
            constructor_name: mistral_devstral_2,
            display_name: "Devstral 2",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralDevstralSmall2 {
            model_name: "mistral/devstral-small-2",
            constructor_name: mistral_devstral_small_2,
            display_name: "Devstral Small 2",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMagistralMedium {
            model_name: "mistral/magistral-medium",
            constructor_name: mistral_magistral_medium,
            display_name: "Magistral Medium (latest)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMagistralSmall {
            model_name: "mistral/magistral-small",
            constructor_name: mistral_magistral_small,
            display_name: "Magistral Small",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMinistral14b {
            model_name: "mistral/ministral-14b",
            constructor_name: mistral_ministral_14b,
            display_name: "Ministral 14B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralMinistral3b {
            model_name: "mistral/ministral-3b",
            constructor_name: mistral_ministral_3b,
            display_name: "Ministral 3B (latest)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMinistral8b {
            model_name: "mistral/ministral-8b",
            constructor_name: mistral_ministral_8b,
            display_name: "Ministral 8B (latest)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistralEmbed {
            model_name: "mistral/mistral-embed",
            constructor_name: mistral_mistral_embed,
            display_name: "Mistral Embed",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MistralMistralLarge3 {
            model_name: "mistral/mistral-large-3",
            constructor_name: mistral_mistral_large_3,
            display_name: "Mistral Large 3",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport]
        },
        MistralMistralMedium {
            model_name: "mistral/mistral-medium",
            constructor_name: mistral_mistral_medium,
            display_name: "Mistral Medium 3.1",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistralMedium35 {
            model_name: "mistral/mistral-medium-3.5",
            constructor_name: mistral_mistral_medium_3_5,
            display_name: "Mistral Medium Latest",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistralNemo {
            model_name: "mistral/mistral-nemo",
            constructor_name: mistral_mistral_nemo,
            display_name: "Mistral Nemo",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralMistralSmall {
            model_name: "mistral/mistral-small",
            constructor_name: mistral_mistral_small,
            display_name: "Mistral Small (latest)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralPixtral12b {
            model_name: "mistral/pixtral-12b",
            constructor_name: mistral_pixtral_12b,
            display_name: "Pixtral 12B",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2 {
            model_name: "moonshotai/kimi-k2",
            constructor_name: moonshotai_kimi_k2,
            display_name: "Kimi K2 Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK2Thinking {
            model_name: "moonshotai/kimi-k2-thinking",
            constructor_name: moonshotai_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK25 {
            model_name: "moonshotai/kimi-k2.5",
            constructor_name: moonshotai_kimi_k2_5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK26 {
            model_name: "moonshotai/kimi-k2.6",
            constructor_name: moonshotai_kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK27Code {
            model_name: "moonshotai/kimi-k2.7-code",
            constructor_name: moonshotai_kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK27CodeHighspeed {
            model_name: "moonshotai/kimi-k2.7-code-highspeed",
            constructor_name: moonshotai_kimi_k2_7_code_highspeed,
            display_name: "Kimi K2.7 Code High Speed",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK3 {
            model_name: "moonshotai/kimi-k3",
            constructor_name: moonshotai_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK3Fast {
            model_name: "moonshotai/kimi-k3-fast",
            constructor_name: moonshotai_kimi_k3_fast,
            display_name: "Kimi K3 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MorphMorphV3Fast {
            model_name: "morph/morph-v3-fast",
            constructor_name: morph_morph_v3_fast,
            display_name: "Morph v3 Fast",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        MorphMorphV3Large {
            model_name: "morph/morph-v3-large",
            constructor_name: morph_morph_v3_large,
            display_name: "Morph v3 Large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3Nano30bA3b {
            model_name: "nvidia/nemotron-3-nano-30b-a3b",
            constructor_name: nvidia_nemotron_3_nano_30b_a3b,
            display_name: "Nemotron 3 Nano 30B A3B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3Super120bA12b {
            model_name: "nvidia/nemotron-3-super-120b-a12b",
            constructor_name: nvidia_nemotron_3_super_120b_a12b,
            display_name: "NVIDIA Nemotron 3 Super 120B A12B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        NvidiaNemotron3Ultra550bA55b {
            model_name: "nvidia/nemotron-3-ultra-550b-a55b",
            constructor_name: nvidia_nemotron_3_ultra_550b_a55b,
            display_name: "Nemotron 3 Ultra",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronNano12bV2Vl {
            model_name: "nvidia/nemotron-nano-12b-v2-vl",
            constructor_name: nvidia_nemotron_nano_12b_v2_vl,
            display_name: "Nvidia Nemotron Nano 12B V2 VL",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NvidiaNemotronNano9bV2 {
            model_name: "nvidia/nemotron-nano-9b-v2",
            constructor_name: nvidia_nemotron_nano_9b_v2,
            display_name: "Nvidia Nemotron Nano 9B V2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt35Turbo {
            model_name: "openai/gpt-3.5-turbo",
            constructor_name: openai_gpt_3_5_turbo,
            display_name: "GPT-3.5 Turbo",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt41 {
            model_name: "openai/gpt-4.1",
            constructor_name: openai_gpt_4_1,
            display_name: "GPT-4.1",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt41Mini {
            model_name: "openai/gpt-4.1-mini",
            constructor_name: openai_gpt_4_1_mini,
            display_name: "GPT-4.1 mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4o {
            model_name: "openai/gpt-4o",
            constructor_name: openai_gpt_4o,
            display_name: "GPT-4o",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oMini {
            model_name: "openai/gpt-4o-mini",
            constructor_name: openai_gpt_4o_mini,
            display_name: "GPT-4o mini",
            capabilities: [ImageInputSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt4oMiniSearchPreview {
            model_name: "openai/gpt-4o-mini-search-preview",
            constructor_name: openai_gpt_4o_mini_search_preview,
            display_name: "GPT 4o Mini Search Preview",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiGpt4oMiniTranscribe {
            model_name: "openai/gpt-4o-mini-transcribe",
            constructor_name: openai_gpt_4o_mini_transcribe,
            display_name: "GPT-4o mini Transcribe",
            capabilities: [AudioInputSupport, TextOutputSupport]
        },
        OpenaiGpt4oTranscribe {
            model_name: "openai/gpt-4o-transcribe",
            constructor_name: openai_gpt_4o_transcribe,
            display_name: "GPT-4o Transcribe",
            capabilities: [AudioInputSupport, TextOutputSupport]
        },
        OpenaiGpt5 {
            model_name: "openai/gpt-5",
            constructor_name: openai_gpt_5,
            display_name: "GPT-5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Codex {
            model_name: "openai/gpt-5-codex",
            constructor_name: openai_gpt_5_codex,
            display_name: "GPT-5-Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Mini {
            model_name: "openai/gpt-5-mini",
            constructor_name: openai_gpt_5_mini,
            display_name: "GPT-5 Mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Nano {
            model_name: "openai/gpt-5-nano",
            constructor_name: openai_gpt_5_nano,
            display_name: "GPT-5 Nano",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt5Pro {
            model_name: "openai/gpt-5-pro",
            constructor_name: openai_gpt_5_pro,
            display_name: "GPT-5 pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Codex {
            model_name: "openai/gpt-5.1-codex",
            constructor_name: openai_gpt_5_1_codex,
            display_name: "GPT-5.1-Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
            display_name: "GPT-5.1 Codex mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Instant {
            model_name: "openai/gpt-5.1-instant",
            constructor_name: openai_gpt_5_1_instant,
            display_name: "GPT-5.1 Instant",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt51Thinking {
            model_name: "openai/gpt-5.1-thinking",
            constructor_name: openai_gpt_5_1_thinking,
            display_name: "GPT 5.1 Thinking",
            capabilities: [ImageInputSupport, ImageOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52 {
            model_name: "openai/gpt-5.2",
            constructor_name: openai_gpt_5_2,
            display_name: "GPT-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Codex {
            model_name: "openai/gpt-5.2-codex",
            constructor_name: openai_gpt_5_2_codex,
            display_name: "GPT-5.2-Codex",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt52Pro {
            model_name: "openai/gpt-5.2-pro",
            constructor_name: openai_gpt_5_2_pro,
            display_name: "GPT 5.2 ",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt53Chat {
            model_name: "openai/gpt-5.3-chat",
            constructor_name: openai_gpt_5_3_chat,
            display_name: "GPT-5.3 Chat",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
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
        OpenaiGpt54Pro {
            model_name: "openai/gpt-5.4-pro",
            constructor_name: openai_gpt_5_4_pro,
            display_name: "GPT 5.4 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt55 {
            model_name: "openai/gpt-5.5",
            constructor_name: openai_gpt_5_5,
            display_name: "GPT 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt55Pro {
            model_name: "openai/gpt-5.5-pro",
            constructor_name: openai_gpt_5_5_pro,
            display_name: "GPT 5.5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Luna {
            model_name: "openai/gpt-5.6-luna",
            constructor_name: openai_gpt_5_6_luna,
            display_name: "GPT 5.6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Sol {
            model_name: "openai/gpt-5.6-sol",
            constructor_name: openai_gpt_5_6_sol,
            display_name: "GPT 5.6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGpt56Terra {
            model_name: "openai/gpt-5.6-terra",
            constructor_name: openai_gpt_5_6_terra,
            display_name: "GPT 5.6 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptImage1 {
            model_name: "openai/gpt-image-1",
            constructor_name: openai_gpt_image_1,
            display_name: "GPT Image 1",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        OpenaiGptImage1Mini {
            model_name: "openai/gpt-image-1-mini",
            constructor_name: openai_gpt_image_1_mini,
            display_name: "GPT Image 1 Mini",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        OpenaiGptImage15 {
            model_name: "openai/gpt-image-1.5",
            constructor_name: openai_gpt_image_1_5,
            display_name: "GPT Image 1.5",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        OpenaiGptImage2 {
            model_name: "openai/gpt-image-2",
            constructor_name: openai_gpt_image_2,
            display_name: "GPT Image 2",
            capabilities: [ImageOutputSupport, TextInputSupport]
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
        OpenaiGptOssSafeguard20b {
            model_name: "openai/gpt-oss-safeguard-20b",
            constructor_name: openai_gpt_oss_safeguard_20b,
            display_name: "gpt-oss-safeguard-20b",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiGptRealtime15 {
            model_name: "openai/gpt-realtime-1.5",
            constructor_name: openai_gpt_realtime_1_5,
            display_name: "GPT-Realtime-1.5",
            capabilities: [AudioInputSupport, AudioOutputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGptRealtime2 {
            model_name: "openai/gpt-realtime-2",
            constructor_name: openai_gpt_realtime_2,
            display_name: "gpt-realtime-2",
            capabilities: [AudioInputSupport, AudioOutputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGptRealtime21 {
            model_name: "openai/gpt-realtime-2.1",
            constructor_name: openai_gpt_realtime_2_1,
            display_name: "gpt-realtime-2.1",
            capabilities: [AudioInputSupport, AudioOutputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGptRealtimeMini {
            model_name: "openai/gpt-realtime-mini",
            constructor_name: openai_gpt_realtime_mini,
            display_name: "GPT-Realtime mini",
            capabilities: [AudioInputSupport, AudioOutputSupport, TextInputSupport, TextOutputSupport]
        },
        OpenaiGptRealtimeWhisper {
            model_name: "openai/gpt-realtime-whisper",
            constructor_name: openai_gpt_realtime_whisper,
            display_name: "gpt-realtime-whisper",
            capabilities: [AudioInputSupport, TextOutputSupport]
        },
        OpenaiO3 {
            model_name: "openai/o3",
            constructor_name: openai_o3,
            display_name: "o3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3DeepResearch {
            model_name: "openai/o3-deep-research",
            constructor_name: openai_o3_deep_research,
            display_name: "o3-deep-research",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiO3Pro {
            model_name: "openai/o3-pro",
            constructor_name: openai_o3_pro,
            display_name: "o3 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        OpenaiTextEmbedding3Large {
            model_name: "openai/text-embedding-3-large",
            constructor_name: openai_text_embedding_3_large,
            display_name: "text-embedding-3-large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiTextEmbedding3Small {
            model_name: "openai/text-embedding-3-small",
            constructor_name: openai_text_embedding_3_small,
            display_name: "text-embedding-3-small",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiTextEmbeddingAda002 {
            model_name: "openai/text-embedding-ada-002",
            constructor_name: openai_text_embedding_ada_002,
            display_name: "text-embedding-ada-002",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        OpenaiTts1 {
            model_name: "openai/tts-1",
            constructor_name: openai_tts_1,
            display_name: "TTS-1",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        OpenaiTts1Hd {
            model_name: "openai/tts-1-hd",
            constructor_name: openai_tts_1_hd,
            display_name: "TTS-1 HD",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        OpenaiWhisper1 {
            model_name: "openai/whisper-1",
            constructor_name: openai_whisper_1,
            display_name: "Whisper",
            capabilities: [AudioInputSupport, TextOutputSupport]
        },
        PerplexityPplxEmbedV106b {
            model_name: "perplexity/pplx-embed-v1-0.6b",
            constructor_name: perplexity_pplx_embed_v1_0_6b,
            display_name: "Embed v1 0.6b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        PerplexityPplxEmbedV14b {
            model_name: "perplexity/pplx-embed-v1-4b",
            constructor_name: perplexity_pplx_embed_v1_4b,
            display_name: "Embed v1 4b",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        PerplexitySonar {
            model_name: "perplexity/sonar",
            constructor_name: perplexity_sonar,
            display_name: "Sonar",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PerplexitySonarPro {
            model_name: "perplexity/sonar-pro",
            constructor_name: perplexity_sonar_pro,
            display_name: "Sonar Pro",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PerplexitySonarReasoningPro {
            model_name: "perplexity/sonar-reasoning-pro",
            constructor_name: perplexity_sonar_reasoning_pro,
            display_name: "Sonar Reasoning Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        PoolsideLagunaS21 {
            model_name: "poolside/laguna-s-2.1",
            constructor_name: poolside_laguna_s_2_1,
            display_name: "Laguna S 2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        PoolsideLagunaS21Free {
            model_name: "poolside/laguna-s-2.1-free",
            constructor_name: poolside_laguna_s_2_1_free,
            display_name: "Laguna S 2.1 Free",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ProdiaFluxFastSchnell {
            model_name: "prodia/flux-fast-schnell",
            constructor_name: prodia_flux_fast_schnell,
            display_name: "Flux Schnell",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        QuiveraiArrow11 {
            model_name: "quiverai/arrow-1.1",
            constructor_name: quiverai_arrow_1_1,
            display_name: "Arrow 1.1",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV2 {
            model_name: "recraft/recraft-v2",
            constructor_name: recraft_recraft_v2,
            display_name: "Recraft V2",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV3 {
            model_name: "recraft/recraft-v3",
            constructor_name: recraft_recraft_v3,
            display_name: "Recraft V3",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV4 {
            model_name: "recraft/recraft-v4",
            constructor_name: recraft_recraft_v4,
            display_name: "Recraft V4",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV4Pro {
            model_name: "recraft/recraft-v4-pro",
            constructor_name: recraft_recraft_v4_pro,
            display_name: "Recraft V4 Pro",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV41 {
            model_name: "recraft/recraft-v4.1",
            constructor_name: recraft_recraft_v4_1,
            display_name: "Recraft V4.1",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV41Pro {
            model_name: "recraft/recraft-v4.1-pro",
            constructor_name: recraft_recraft_v4_1_pro,
            display_name: "Recraft V4.1 Pro",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV41Utility {
            model_name: "recraft/recraft-v4.1-utility",
            constructor_name: recraft_recraft_v4_1_utility,
            display_name: "Recraft V4.1 Utility",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        RecraftRecraftV41UtilityPro {
            model_name: "recraft/recraft-v4.1-utility-pro",
            constructor_name: recraft_recraft_v4_1_utility_pro,
            display_name: "Recraft V4.1 Utility Pro",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        SakanaFuguUltra {
            model_name: "sakana/fugu-ultra",
            constructor_name: sakana_fugu_ultra,
            display_name: "Fugu Ultra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        StepfunStep35Flash {
            model_name: "stepfun/step-3.5-flash",
            constructor_name: stepfun_step_3_5_flash,
            display_name: "StepFun 3.5 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        StepfunStep37Flash {
            model_name: "stepfun/step-3.7-flash",
            constructor_name: stepfun_step_3_7_flash,
            display_name: "Step 3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TencentHy3 {
            model_name: "tencent/hy3",
            constructor_name: tencent_hy3,
            display_name: "Hy3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInkling {
            model_name: "thinkingmachines/inkling",
            constructor_name: thinkingmachines_inkling,
            display_name: "Inkling",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ThinkingmachinesInklingSmall {
            model_name: "thinkingmachines/inkling-small",
            constructor_name: thinkingmachines_inkling_small,
            display_name: "Inkling Small",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        VoyageRerank25 {
            model_name: "voyage/rerank-2.5",
            constructor_name: voyage_rerank_2_5,
            display_name: "Voyage Rerank 2.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageRerank25Lite {
            model_name: "voyage/rerank-2.5-lite",
            constructor_name: voyage_rerank_2_5_lite,
            display_name: "Voyage Rerank 2.5 Lite",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyage3Large {
            model_name: "voyage/voyage-3-large",
            constructor_name: voyage_voyage_3_large,
            display_name: "voyage-3-large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyage35 {
            model_name: "voyage/voyage-3.5",
            constructor_name: voyage_voyage_3_5,
            display_name: "voyage-3.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyage35Lite {
            model_name: "voyage/voyage-3.5-lite",
            constructor_name: voyage_voyage_3_5_lite,
            display_name: "voyage-3.5-lite",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyage4 {
            model_name: "voyage/voyage-4",
            constructor_name: voyage_voyage_4,
            display_name: "voyage-4",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyage4Large {
            model_name: "voyage/voyage-4-large",
            constructor_name: voyage_voyage_4_large,
            display_name: "voyage-4-large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyage4Lite {
            model_name: "voyage/voyage-4-lite",
            constructor_name: voyage_voyage_4_lite,
            display_name: "voyage-4-lite",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyageCode2 {
            model_name: "voyage/voyage-code-2",
            constructor_name: voyage_voyage_code_2,
            display_name: "voyage-code-2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyageCode3 {
            model_name: "voyage/voyage-code-3",
            constructor_name: voyage_voyage_code_3,
            display_name: "voyage-code-3",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyageFinance2 {
            model_name: "voyage/voyage-finance-2",
            constructor_name: voyage_voyage_finance_2,
            display_name: "voyage-finance-2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        VoyageVoyageLaw2 {
            model_name: "voyage/voyage-law-2",
            constructor_name: voyage_voyage_law_2,
            display_name: "voyage-law-2",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        XaiGrok41FastNonReasoning {
            model_name: "xai/grok-4.1-fast-non-reasoning",
            constructor_name: xai_grok_4_1_fast_non_reasoning,
            display_name: "Grok 4.1 Fast Non-Reasoning",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok41FastReasoning {
            model_name: "xai/grok-4.1-fast-reasoning",
            constructor_name: xai_grok_4_1_fast_reasoning,
            display_name: "Grok 4.1 Fast Reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok420MultiAgent {
            model_name: "xai/grok-4.20-multi-agent",
            constructor_name: xai_grok_4_20_multi_agent,
            display_name: "Grok 4.20 Multi-Agent",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok420MultiAgentBeta {
            model_name: "xai/grok-4.20-multi-agent-beta",
            constructor_name: xai_grok_4_20_multi_agent_beta,
            display_name: "Grok 4.20 Multi Agent Beta",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok420NonReasoning {
            model_name: "xai/grok-4.20-non-reasoning",
            constructor_name: xai_grok_4_20_non_reasoning,
            display_name: "Grok 4.20 Non-Reasoning",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok420NonReasoningBeta {
            model_name: "xai/grok-4.20-non-reasoning-beta",
            constructor_name: xai_grok_4_20_non_reasoning_beta,
            display_name: "Grok 4.20 Beta Non-Reasoning",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok420Reasoning {
            model_name: "xai/grok-4.20-reasoning",
            constructor_name: xai_grok_4_20_reasoning,
            display_name: "Grok 4.20 Reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok420ReasoningBeta {
            model_name: "xai/grok-4.20-reasoning-beta",
            constructor_name: xai_grok_4_20_reasoning_beta,
            display_name: "Grok 4.20 Beta Reasoning",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok43 {
            model_name: "xai/grok-4.3",
            constructor_name: xai_grok_4_3,
            display_name: "Grok 4.3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrok45 {
            model_name: "xai/grok-4.5",
            constructor_name: xai_grok_4_5,
            display_name: "Grok 4.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrokBuild01 {
            model_name: "xai/grok-build-0.1",
            constructor_name: xai_grok_build_0_1,
            display_name: "Grok Build 0.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XaiGrokImagineImage {
            model_name: "xai/grok-imagine-image",
            constructor_name: xai_grok_imagine_image,
            display_name: "Grok Imagine Image",
            capabilities: [ImageOutputSupport, TextInputSupport, TextOutputSupport]
        },
        XaiGrokImagineVideo {
            model_name: "xai/grok-imagine-video",
            constructor_name: xai_grok_imagine_video,
            display_name: "Grok Imagine",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        XaiGrokImagineVideo15 {
            model_name: "xai/grok-imagine-video-1.5",
            constructor_name: xai_grok_imagine_video_1_5,
            display_name: "Grok Imagine Video 1.5",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        XaiGrokImagineVideo15Preview {
            model_name: "xai/grok-imagine-video-1.5-preview",
            constructor_name: xai_grok_imagine_video_1_5_preview,
            display_name: "Grok Imagine Video 1.5 Preview",
            capabilities: [TextInputSupport, VideoOutputSupport]
        },
        XaiGrokStt {
            model_name: "xai/grok-stt",
            constructor_name: xai_grok_stt,
            display_name: "Grok STT",
            capabilities: [AudioInputSupport, TextOutputSupport]
        },
        XaiGrokTts {
            model_name: "xai/grok-tts",
            constructor_name: xai_grok_tts,
            display_name: "Grok TTS",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        XaiGrokVoiceThinkFast10 {
            model_name: "xai/grok-voice-think-fast-1.0",
            constructor_name: xai_grok_voice_think_fast_1_0,
            display_name: "Grok Voice Think Fast 1.0",
            capabilities: [AudioInputSupport, AudioOutputSupport, TextInputSupport, TextOutputSupport]
        },
        XaiGrokVoiceThinkFast20 {
            model_name: "xai/grok-voice-think-fast-2.0",
            constructor_name: xai_grok_voice_think_fast_2_0,
            display_name: "Grok Voice Think Fast 2.0",
            capabilities: [AudioInputSupport, AudioOutputSupport, TextInputSupport, TextOutputSupport]
        },
        XiaomiMimoV25 {
            model_name: "xiaomi/mimo-v2.5",
            constructor_name: xiaomi_mimo_v2_5,
            display_name: "MiMo M2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV25Pro {
            model_name: "xiaomi/mimo-v2.5-pro",
            constructor_name: xiaomi_mimo_v2_5_pro,
            display_name: "MiMo V2.5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm45 {
            model_name: "zai/glm-4.5",
            constructor_name: zai_glm_4_5,
            display_name: "GLM 4.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm45Air {
            model_name: "zai/glm-4.5-air",
            constructor_name: zai_glm_4_5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm45v {
            model_name: "zai/glm-4.5v",
            constructor_name: zai_glm_4_5v,
            display_name: "GLM 4.5V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm46 {
            model_name: "zai/glm-4.6",
            constructor_name: zai_glm_4_6,
            display_name: "GLM 4.6",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm46v {
            model_name: "zai/glm-4.6v",
            constructor_name: zai_glm_4_6v,
            display_name: "GLM-4.6V",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm46vFlash {
            model_name: "zai/glm-4.6v-flash",
            constructor_name: zai_glm_4_6v_flash,
            display_name: "GLM-4.6V-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm47 {
            model_name: "zai/glm-4.7",
            constructor_name: zai_glm_4_7,
            display_name: "GLM 4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm47Flash {
            model_name: "zai/glm-4.7-flash",
            constructor_name: zai_glm_4_7_flash,
            display_name: "GLM 4.7 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm47Flashx {
            model_name: "zai/glm-4.7-flashx",
            constructor_name: zai_glm_4_7_flashx,
            display_name: "GLM 4.7 FlashX",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm5 {
            model_name: "zai/glm-5",
            constructor_name: zai_glm_5,
            display_name: "GLM-5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm5Turbo {
            model_name: "zai/glm-5-turbo",
            constructor_name: zai_glm_5_turbo,
            display_name: "GLM 5 Turbo",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm51 {
            model_name: "zai/glm-5.1",
            constructor_name: zai_glm_5_1,
            display_name: "GLM 5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm52 {
            model_name: "zai/glm-5.2",
            constructor_name: zai_glm_5_2,
            display_name: "GLM 5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm52Fast {
            model_name: "zai/glm-5.2-fast",
            constructor_name: zai_glm_5_2_fast,
            display_name: "GLM 5.2 Fast",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiGlm5vTurbo {
            model_name: "zai/glm-5v-turbo",
            constructor_name: zai_glm_5v_turbo,
            display_name: "GLM 5V Turbo",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
