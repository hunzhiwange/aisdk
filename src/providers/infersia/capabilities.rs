//! Capabilities for infersia models.
//!
//! This module defines model types and their capabilities for infersia providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::infersia::Infersia;

model_capabilities! {
    provider: Infersia,
    models: {
        DeepseekDeepseekV41Flash1017 {
            model_name: "deepseek/deepseek-v4.1-flash-1017",
            constructor_name: deepseek_deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxMinimaxM31Flash {
            model_name: "minimax/minimax-m3.1-flash",
            constructor_name: minimax_minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ReflectionBeam10501bA23b {
            model_name: "reflection/beam-1.0-501b-a23b",
            constructor_name: reflection_beam_1_0_501b_a23b,
            display_name: "Beam 1.0 501B A23B",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        TencentHy4Preview {
            model_name: "tencent/hy4-preview",
            constructor_name: tencent_hy4_preview,
            display_name: "Hy4 preview",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        XiaomiMimoV26Flash {
            model_name: "xiaomi/mimo-v2.6-flash",
            constructor_name: xiaomi_mimo_v2_6_flash,
            display_name: "MiMo-V2.6-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        XiaomiMimoV26Pro {
            model_name: "xiaomi/mimo-v2.6-pro",
            constructor_name: xiaomi_mimo_v2_6_pro,
            display_name: "MiMo-V2.6-Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
