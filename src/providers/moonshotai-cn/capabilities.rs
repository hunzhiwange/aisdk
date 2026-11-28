//! Capabilities for moonshotai_cn models.
//!
//! This module defines model types and their capabilities for moonshotai_cn providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::moonshotai_cn::MoonshotaiCn;

model_capabilities! {
    provider: MoonshotaiCn,
    models: {
        KimiK3 {
            model_name: "kimi-k3",
            constructor_name: kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK31Pro {
            model_name: "kimi-k3.1-pro",
            constructor_name: kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK31ProHighspeed {
            model_name: "kimi-k3.1-pro-highspeed",
            constructor_name: kimi_k3_1_pro_highspeed,
            display_name: "Kimi K3.1 Pro Highspeed",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
