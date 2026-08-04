//! Capabilities for xiaomi models.
//!
//! This module defines model types and their capabilities for xiaomi providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::xiaomi::Xiaomi;

model_capabilities! {
    provider: Xiaomi,
    models: {
        MimoV25 {
            model_name: "mimo-v2.5",
            constructor_name: mimo_v2_5,
            display_name: "MiMo-V2.5",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MimoV25Pro {
            model_name: "mimo-v2.5-pro",
            constructor_name: mimo_v2_5_pro,
            display_name: "MiMo-V2.5-Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MimoV25ProUltraspeed {
            model_name: "mimo-v2.5-pro-ultraspeed",
            constructor_name: mimo_v2_5_pro_ultraspeed,
            display_name: "MiMo-V2.5-Pro-UltraSpeed",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
