//! Capabilities for moonshotai models.
//!
//! This module defines model types and their capabilities for moonshotai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::moonshotai::Moonshotai;

model_capabilities! {
    provider: Moonshotai,
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
