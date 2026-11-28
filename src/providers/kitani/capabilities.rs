//! Capabilities for kitani models.
//!
//! This module defines model types and their capabilities for kitani providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::kitani::Kitani;

model_capabilities! {
    provider: Kitani,
    models: {
        DeepseekDeepseekV4Pro0813 {
            model_name: "deepseek/deepseek-v4-pro-0813",
            constructor_name: deepseek_deepseek_v4_pro_0813,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash1017 {
            model_name: "deepseek/deepseek-v4.1-flash-1017",
            constructor_name: deepseek_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV41Flash1017Thinking {
            model_name: "deepseek/deepseek-v4.1-flash-1017-thinking",
            constructor_name: deepseek_deepseek_v4_1_flash_1017_thinking,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GoogleGemma412bIt {
            model_name: "google/gemma-4-12b-it",
            constructor_name: google_gemma_4_12b_it,
            display_name: "Gemma 4 12B IT",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MetaLlamaLlama318bInstruct {
            model_name: "meta-llama/llama-3.1-8b-instruct",
            constructor_name: meta_llama_llama_3_1_8b_instruct,
            display_name: "Llama-3.1-8B-Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMinistral8b2512 {
            model_name: "mistralai/ministral-8b-2512",
            constructor_name: mistralai_ministral_8b_2512,
            display_name: "Ministral 3 8B 2512",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MistralaiMistralNemoInstruct2407 {
            model_name: "mistralai/mistral-nemo-instruct-2407",
            constructor_name: mistralai_mistral_nemo_instruct_2407,
            display_name: "Mistral Nemo",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53 {
            model_name: "z-ai/glm-5.3",
            constructor_name: z_ai_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53Flash {
            model_name: "z-ai/glm-5.3-flash",
            constructor_name: z_ai_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
