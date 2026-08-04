//! Capabilities for regolo_ai models.
//!
//! This module defines model types and their capabilities for regolo_ai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::regolo_ai::RegoloAi;

model_capabilities! {
    provider: RegoloAi,
    models: {
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "GPT-OSS-120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss20b {
            model_name: "gpt-oss-20b",
            constructor_name: gpt_oss_20b,
            display_name: "GPT-OSS-20B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama318bInstruct {
            model_name: "llama-3.1-8b-instruct",
            constructor_name: llama_3_1_8b_instruct,
            display_name: "Llama 3.1 8B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Llama3370bInstruct {
            model_name: "llama-3.3-70b-instruct",
            constructor_name: llama_3_3_70b_instruct,
            display_name: "Llama 3.3 70B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM25 {
            model_name: "minimax-m2.5",
            constructor_name: minimax_m2_5,
            display_name: "MiniMax 2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralSmall4119b {
            model_name: "mistral-small-4-119b",
            constructor_name: mistral_small_4_119b,
            display_name: "Mistral Small 4 119B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralSmall32 {
            model_name: "mistral-small3.2",
            constructor_name: mistral_small3_2,
            display_name: "Mistral Small 3.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenImage {
            model_name: "qwen-image",
            constructor_name: qwen_image,
            display_name: "Qwen-Image",
            capabilities: [ImageOutputSupport, TextInputSupport]
        },
        Qwen3CoderNext {
            model_name: "qwen3-coder-next",
            constructor_name: qwen3_coder_next,
            display_name: "Qwen3-Coder-Next",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Embedding8b {
            model_name: "qwen3-embedding-8b",
            constructor_name: qwen3_embedding_8b,
            display_name: "Qwen3-Embedding-8B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwen3Reranker4b {
            model_name: "qwen3-reranker-4b",
            constructor_name: qwen3_reranker_4b,
            display_name: "Qwen3-Reranker-4B",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        Qwen35122b {
            model_name: "qwen3.5-122b",
            constructor_name: qwen3_5_122b,
            display_name: "Qwen3.5-122B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen359b {
            model_name: "qwen3.5-9b",
            constructor_name: qwen3_5_9b,
            display_name: "Qwen3.5-9B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
