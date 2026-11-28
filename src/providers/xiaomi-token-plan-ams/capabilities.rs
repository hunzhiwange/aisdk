//! Capabilities for xiaomi_token_plan_ams models.
//!
//! This module defines model types and their capabilities for xiaomi_token_plan_ams providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::xiaomi_token_plan_ams::XiaomiTokenPlanAms;

model_capabilities! {
    provider: XiaomiTokenPlanAms,
    models: {
        MimoV26Flash {
            model_name: "mimo-v2.6-flash",
            constructor_name: mimo_v2_6_flash,
            display_name: "MiMo-V2.6-Flash",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MimoV26Pro {
            model_name: "mimo-v2.6-pro",
            constructor_name: mimo_v2_6_pro,
            display_name: "MiMo-V2.6-Pro",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
