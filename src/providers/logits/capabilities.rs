//! Capabilities for logits models.
//!
//! This module defines model types and their capabilities for logits providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::logits::Logits;

model_capabilities! {
    provider: Logits,
    models: {
        DeepseekDeepseekV41Flash {
            model_name: "deepseek/deepseek-v4.1-flash",
            constructor_name: deepseek_deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MoonshotaiKimiK31Pro {
            model_name: "moonshotai/kimi-k3.1-pro",
            constructor_name: moonshotai_kimi_k3_1_pro,
            display_name: "Kimi K3.1 Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
