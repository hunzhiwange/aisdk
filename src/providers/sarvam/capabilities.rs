//! Capabilities for sarvam models.
//!
//! This module defines model types and their capabilities for sarvam providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::sarvam::Sarvam;

model_capabilities! {
    provider: Sarvam,
    models: {
        Deepseekv41Flash1017 {
            model_name: "deepseekv4.1-flash-1017",
            constructor_name: deepseekv4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Deepseekv41Flash1017Interactive {
            model_name: "deepseekv4.1-flash-1017-interactive",
            constructor_name: deepseekv4_1_flash_1017_interactive,
            display_name: "DeepSeek V4.1 Flash 1017 Interactive",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma4 {
            model_name: "gemma4",
            constructor_name: gemma4,
            display_name: "Gemma 4 31B IT",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53 {
            model_name: "glm5.3",
            constructor_name: glm5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Flash {
            model_name: "glm5.3-flash",
            constructor_name: glm5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Sarvam105b {
            model_name: "sarvam-105b",
            constructor_name: sarvam_105b,
            display_name: "Sarvam-105B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Sarvam105bConversations {
            model_name: "sarvam-105b-conversations",
            constructor_name: sarvam_105b_conversations,
            display_name: "Sarvam-105B Conversations",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
