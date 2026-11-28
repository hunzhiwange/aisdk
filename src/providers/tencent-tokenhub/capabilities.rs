//! Capabilities for tencent_tokenhub models.
//!
//! This module defines model types and their capabilities for tencent_tokenhub providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::tencent_tokenhub::TencentTokenhub;

model_capabilities! {
    provider: TencentTokenhub,
    models: {
        DeepseekV4Pro {
            model_name: "deepseek-v4-pro",
            constructor_name: deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4Pro0813 {
            model_name: "deepseek-v4-pro-0813",
            constructor_name: deepseek_v4_pro_0813,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash {
            model_name: "deepseek-v4.1-flash",
            constructor_name: deepseek_v4_1_flash,
            display_name: "DeepSeek V4.1 Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash1017 {
            model_name: "deepseek-v4.1-flash-1017",
            constructor_name: deepseek_v4_1_flash_1017,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekFlash {
            model_name: "deepseek/deepseek-flash",
            constructor_name: deepseek_deepseek_flash,
            display_name: "DeepSeek V4.1 Flash 1017",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekDeepseekV4Pro {
            model_name: "deepseek/deepseek-v4-pro",
            constructor_name: deepseek_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro 0813",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm52 {
            model_name: "glm-5.2",
            constructor_name: glm_5_2,
            display_name: "GLM-5.2",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53 {
            model_name: "glm-5.3",
            constructor_name: glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Glm53Flash {
            model_name: "glm-5.3-flash",
            constructor_name: glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Hy3 {
            model_name: "hy3",
            constructor_name: hy3,
            display_name: "Hy3",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hy4 {
            model_name: "hy4",
            constructor_name: hy4,
            display_name: "Hy4",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Hy4Flash {
            model_name: "hy4-flash",
            constructor_name: hy4_flash,
            display_name: "Hy4 flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK26 {
            model_name: "kimi-k2.6",
            constructor_name: kimi_k2_6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK27Code {
            model_name: "kimi-k2.7-code",
            constructor_name: kimi_k2_7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK27CodeHighspeed {
            model_name: "kimi-k2.7-code-highspeed",
            constructor_name: kimi_k2_7_code_highspeed,
            display_name: "Kimi K2.7 Code Highspeed",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        KimiK28Preview {
            model_name: "kimi-k2.8-preview",
            constructor_name: kimi_k2_8_preview,
            display_name: "Kimi K2.8 Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
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
        MimoV25Pro {
            model_name: "mimo-v2.5-pro",
            constructor_name: mimo_v2_5_pro,
            display_name: "MiMo-V2.5-Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MimoV26Flash {
            model_name: "mimo-v2.6-flash",
            constructor_name: mimo_v2_6_flash,
            display_name: "MiMo-V2.6-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MimoV26Pro {
            model_name: "mimo-v2.6-pro",
            constructor_name: mimo_v2_6_pro,
            display_name: "MiMo-V2.6-Pro",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MinimaxM3 {
            model_name: "minimax-m3",
            constructor_name: minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MinimaxM31Flash {
            model_name: "minimax-m3.1-flash",
            constructor_name: minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Step5Preview {
            model_name: "step-5-preview",
            constructor_name: step_5_preview,
            display_name: "Step 5 Preview",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
