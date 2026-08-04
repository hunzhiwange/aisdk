//! Capabilities for tinfoil models.
//!
//! This module defines model types and their capabilities for tinfoil providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::tinfoil::Tinfoil;

model_capabilities! {
    provider: Tinfoil,
    models: {
        Gemma431b {
            model_name: "gemma4-31b",
            constructor_name: gemma4_31b,
            display_name: "Gemma 4 31B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52 {
            model_name: "glm-5-2",
            constructor_name: glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "gpt-oss-120b",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOssSafeguard120b {
            model_name: "gpt-oss-safeguard-120b",
            constructor_name: gpt_oss_safeguard_120b,
            display_name: "gpt-oss-safeguard-120b",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK26 {
            model_name: "kimi-k2-6",
            constructor_name: kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Llama3370b {
            model_name: "llama3-3-70b",
            constructor_name: llama3_3_70b,
            display_name: "Llama-3.3-70B-Instruct",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        NomicEmbedText {
            model_name: "nomic-embed-text",
            constructor_name: nomic_embed_text,
            display_name: "Nomic Embed Text v1.5",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
    }
}
