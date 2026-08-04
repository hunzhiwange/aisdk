//! Capabilities for tencent_coding_plan models.
//!
//! This module defines model types and their capabilities for tencent_coding_plan providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::tencent_coding_plan::TencentCodingPlan;

model_capabilities! {
    provider: TencentCodingPlan,
    models: {
        Glm5 {
            model_name: "glm-5",
            constructor_name: glm_5,
            display_name: "GLM-5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hunyuan20Instruct {
            model_name: "hunyuan-2.0-instruct",
            constructor_name: hunyuan_2_0_instruct,
            display_name: "Tencent HY 2.0 Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hunyuan20Thinking {
            model_name: "hunyuan-2.0-thinking",
            constructor_name: hunyuan_2_0_thinking,
            display_name: "Tencent HY 2.0 Think",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HunyuanT1 {
            model_name: "hunyuan-t1",
            constructor_name: hunyuan_t1,
            display_name: "Hunyuan-T1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        HunyuanTurbos {
            model_name: "hunyuan-turbos",
            constructor_name: hunyuan_turbos,
            display_name: "Hunyuan-TurboS",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK25 {
            model_name: "kimi-k2.5",
            constructor_name: kimi_k2_5,
            display_name: "Kimi-K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MinimaxM25 {
            model_name: "minimax-m2.5",
            constructor_name: minimax_m2_5,
            display_name: "MiniMax-M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TcCodeLatest {
            model_name: "tc-code-latest",
            constructor_name: tc_code_latest,
            display_name: "Auto",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
