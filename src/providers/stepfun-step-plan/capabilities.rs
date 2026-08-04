//! Capabilities for stepfun_step_plan models.
//!
//! This module defines model types and their capabilities for stepfun_step_plan providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::stepfun_step_plan::StepfunStepPlan;

model_capabilities! {
    provider: StepfunStepPlan,
    models: {
        Step35Flash {
            model_name: "step-3.5-flash",
            constructor_name: step_3_5_flash,
            display_name: "Step 3.5 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Step35Flash2603 {
            model_name: "step-3.5-flash-2603",
            constructor_name: step_3_5_flash_2603,
            display_name: "Step 3.5 Flash 2603",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Step37Flash {
            model_name: "step-3.7-flash",
            constructor_name: step_3_7_flash,
            display_name: "Step 3.7 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        StepRouterV1 {
            model_name: "step-router-v1",
            constructor_name: step_router_v1,
            display_name: "Step Router v1",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
