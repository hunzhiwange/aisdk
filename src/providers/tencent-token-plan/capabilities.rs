//! Capabilities for tencent_token_plan models.
//!
//! This module defines model types and their capabilities for tencent_token_plan providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::tencent_token_plan::TencentTokenPlan;

model_capabilities! {
    provider: TencentTokenPlan,
    models: {
        Hy3 {
            model_name: "hy3",
            constructor_name: hy3,
            display_name: "Hy3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
