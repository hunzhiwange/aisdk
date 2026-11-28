//! Capabilities for openai models.
//!
//! This module defines model types and their capabilities for openai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::openai::OpenAI;

model_capabilities! {
    provider: OpenAI,
    models: {
        Gpt52 {
            model_name: "gpt-5.2",
            constructor_name: gpt_5_2,
            display_name: "GPT-5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt52Pro {
            model_name: "gpt-5.2-pro",
            constructor_name: gpt_5_2_pro,
            display_name: "GPT-5.2 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54 {
            model_name: "gpt-5.4",
            constructor_name: gpt_5_4,
            display_name: "GPT-5.4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54Mini {
            model_name: "gpt-5.4-mini",
            constructor_name: gpt_5_4_mini,
            display_name: "GPT-5.4 mini",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt54Pro {
            model_name: "gpt-5.4-pro",
            constructor_name: gpt_5_4_pro,
            display_name: "GPT-5.4 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt55 {
            model_name: "gpt-5.5",
            constructor_name: gpt_5_5,
            display_name: "GPT-5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt55Pro {
            model_name: "gpt-5.5-pro",
            constructor_name: gpt_5_5_pro,
            display_name: "GPT-5.5 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56 {
            model_name: "gpt-5.6",
            constructor_name: gpt_5_6,
            display_name: "GPT-5.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Luna {
            model_name: "gpt-5.6-luna",
            constructor_name: gpt_5_6_luna,
            display_name: "GPT-5.6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Sol {
            model_name: "gpt-5.6-sol",
            constructor_name: gpt_5_6_sol,
            display_name: "GPT-5.6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Terra {
            model_name: "gpt-5.6-terra",
            constructor_name: gpt_5_6_terra,
            display_name: "GPT-5.6 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Astra {
            model_name: "gpt-6-astra",
            constructor_name: gpt_6_astra,
            display_name: "GPT-6 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6AstraHealthcare {
            model_name: "gpt-6-astra-healthcare",
            constructor_name: gpt_6_astra_healthcare,
            display_name: "GPT-6 Astra Healthcare",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Luna {
            model_name: "gpt-6-luna",
            constructor_name: gpt_6_luna,
            display_name: "GPT-6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt6Sol {
            model_name: "gpt-6-sol",
            constructor_name: gpt_6_sol,
            display_name: "GPT-6 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt61Sol {
            model_name: "gpt-6.1-sol",
            constructor_name: gpt_6_1_sol,
            display_name: "GPT-6.1 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt61SolHealthcare {
            model_name: "gpt-6.1-sol-healthcare",
            constructor_name: gpt_6_1_sol_healthcare,
            display_name: "GPT-6.1 Sol Healthcare",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt61Terra {
            model_name: "gpt-6.1-terra",
            constructor_name: gpt_6_1_terra,
            display_name: "GPT-6.1 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt62Astra {
            model_name: "gpt-6.2-astra",
            constructor_name: gpt_6_2_astra,
            display_name: "GPT-6.2 Astra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt62AstraHealthcare {
            model_name: "gpt-6.2-astra-healthcare",
            constructor_name: gpt_6_2_astra_healthcare,
            display_name: "GPT-6.2 Astra Healthcare",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt62Sol {
            model_name: "gpt-6.2-sol",
            constructor_name: gpt_6_2_sol,
            display_name: "GPT-6.2 Sol",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt62SolHealthcare {
            model_name: "gpt-6.2-sol-healthcare",
            constructor_name: gpt_6_2_sol_healthcare,
            display_name: "GPT-6.2 Sol Healthcare",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptDaybreakBlueLatest {
            model_name: "gpt-daybreak-blue-latest",
            constructor_name: gpt_daybreak_blue_latest,
            display_name: "Daybreak Blue",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptDaybreakRedLatest {
            model_name: "gpt-daybreak-red-latest",
            constructor_name: gpt_daybreak_red_latest,
            display_name: "Daybreak Red",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TextEmbedding3Large {
            model_name: "text-embedding-3-large",
            constructor_name: text_embedding_3_large,
            display_name: "text-embedding-3-large",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TextEmbedding3Small {
            model_name: "text-embedding-3-small",
            constructor_name: text_embedding_3_small,
            display_name: "text-embedding-3-small",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
        TextEmbeddingAda002 {
            model_name: "text-embedding-ada-002",
            constructor_name: text_embedding_ada_002,
            display_name: "text-embedding-ada-002",
            capabilities: [TextInputSupport, TextOutputSupport]
        },
    }
}
