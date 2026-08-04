//! Capabilities for the_grid_ai models.
//!
//! This module defines model types and their capabilities for the_grid_ai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::the_grid_ai::TheGridAi;

model_capabilities! {
    provider: TheGridAi,
    models: {
        AgentMax {
            model_name: "agent-max",
            constructor_name: agent_max,
            display_name: "Agent Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AgentPrime {
            model_name: "agent-prime",
            constructor_name: agent_prime,
            display_name: "Agent Prime",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AgentStandard {
            model_name: "agent-standard",
            constructor_name: agent_standard,
            display_name: "Agent Standard",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CodeMax {
            model_name: "code-max",
            constructor_name: code_max,
            display_name: "Code Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CodePrime {
            model_name: "code-prime",
            constructor_name: code_prime,
            display_name: "Code Prime",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        CodeStandard {
            model_name: "code-standard",
            constructor_name: code_standard,
            display_name: "Code Standard",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TextMax {
            model_name: "text-max",
            constructor_name: text_max,
            display_name: "Text Max",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TextPrime {
            model_name: "text-prime",
            constructor_name: text_prime,
            display_name: "Text Prime",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TextStandard {
            model_name: "text-standard",
            constructor_name: text_standard,
            display_name: "Text Standard",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
