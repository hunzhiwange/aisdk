//! Capabilities for uncensuredai models.
//!
//! This module defines model types and their capabilities for uncensuredai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::uncensuredai::Uncensuredai;

model_capabilities! {
    provider: Uncensuredai,
    models: {
        DeepseekV41FlashAbliterated {
            model_name: "deepseek-v4.1-flash-abliterated",
            constructor_name: deepseek_v4_1_flash_abliterated,
            display_name: "DeepSeek V4.1 Flash Abliterated",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Abliterated {
            model_name: "glm-5.3-abliterated",
            constructor_name: glm_5_3_abliterated,
            display_name: "GLM 5.3 Abliterated",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53FlashAbliterated {
            model_name: "glm-5.3-flash-abliterated",
            constructor_name: glm_5_3_flash_abliterated,
            display_name: "GLM 5.3 Flash Abliterated",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
