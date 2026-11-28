//! Capabilities for huawei_maas models.
//!
//! This module defines model types and their capabilities for huawei_maas providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::huawei_maas::HuaweiMaas;

model_capabilities! {
    provider: HuaweiMaas,
    models: {
        DeepseekV4Flash {
            model_name: "deepseek-v4-flash",
            constructor_name: deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash {
            model_name: "deepseek-v4.1-flash",
            constructor_name: deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash1017 {
            model_name: "deepseek-v4.1-flash-1017",
            constructor_name: deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm51 {
            model_name: "glm-5.1",
            constructor_name: glm_5_1,
            display_name: "GLM-5.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52 {
            model_name: "glm-5.2",
            constructor_name: glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53 {
            model_name: "glm-5.3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Openpangu20Flash {
            model_name: "openpangu-2.0-flash",
            constructor_name: openpangu_2_0_flash,
            display_name: "openPangu 2.0 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Openpangu20Pro {
            model_name: "openpangu-2.0-pro",
            constructor_name: openpangu_2_0_pro,
            display_name: "openPangu 2.0 Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
