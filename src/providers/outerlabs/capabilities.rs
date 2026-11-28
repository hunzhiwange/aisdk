//! Capabilities for outerlabs models.
//!
//! This module defines model types and their capabilities for outerlabs providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::outerlabs::Outerlabs;

model_capabilities! {
    provider: Outerlabs,
    models: {
        ClaudeFable5 {
            model_name: "claude-fable-5",
            constructor_name: claude_fable_5,
            display_name: "Claude Fable 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeFable51 {
            model_name: "claude-fable-5-1",
            constructor_name: claude_fable_5_1,
            display_name: "Claude Fable 5.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeFable52 {
            model_name: "claude-fable-5-2",
            constructor_name: claude_fable_5_2,
            display_name: "Claude Fable 5.2",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku45 {
            model_name: "claude-haiku-4-5",
            constructor_name: claude_haiku_4_5,
            display_name: "Claude Haiku 4.5 (latest)",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeHaiku55 {
            model_name: "claude-haiku-5-5",
            constructor_name: claude_haiku_5_5,
            display_name: "Claude Haiku 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus5 {
            model_name: "claude-opus-5",
            constructor_name: claude_opus_5,
            display_name: "Claude Opus 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeOpus55 {
            model_name: "claude-opus-5-5",
            constructor_name: claude_opus_5_5,
            display_name: "Claude Opus 5.5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet46 {
            model_name: "claude-sonnet-4-6",
            constructor_name: claude_sonnet_4_6,
            display_name: "Claude Sonnet 4.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet5 {
            model_name: "claude-sonnet-5",
            constructor_name: claude_sonnet_5,
            display_name: "Claude Sonnet 5",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ClaudeSonnet55 {
            model_name: "claude-sonnet-5-5",
            constructor_name: claude_sonnet_5_5,
            display_name: "Claude Sonnet 5.5",
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
        Gpt61Terra {
            model_name: "gpt-6.1-terra",
            constructor_name: gpt_6_1_terra,
            display_name: "GPT-6.1 Terra",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
