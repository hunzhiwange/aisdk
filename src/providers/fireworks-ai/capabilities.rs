//! Capabilities for fireworks_ai models.
//!
//! This module defines model types and their capabilities for fireworks_ai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::fireworks_ai::FireworksAi;

model_capabilities! {
    provider: FireworksAi,
    models: {
        AccountsFireworksModelsDeepseekV3p1 {
            model_name: "accounts/fireworks/models/deepseek-v3p1",
            constructor_name: accounts_fireworks_models_deepseek_v3p1,
            display_name: "DeepSeek V3.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsDeepseekV3p2 {
            model_name: "accounts/fireworks/models/deepseek-v3p2",
            constructor_name: accounts_fireworks_models_deepseek_v3p2,
            display_name: "DeepSeek V3.2",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsDeepseekV4Flash {
            model_name: "accounts/fireworks/models/deepseek-v4-flash",
            constructor_name: accounts_fireworks_models_deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsDeepseekV4Pro {
            model_name: "accounts/fireworks/models/deepseek-v4-pro",
            constructor_name: accounts_fireworks_models_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsGlm4p5 {
            model_name: "accounts/fireworks/models/glm-4p5",
            constructor_name: accounts_fireworks_models_glm_4p5,
            display_name: "GLM 4.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsGlm4p5Air {
            model_name: "accounts/fireworks/models/glm-4p5-air",
            constructor_name: accounts_fireworks_models_glm_4p5_air,
            display_name: "GLM 4.5 Air",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsGlm4p7 {
            model_name: "accounts/fireworks/models/glm-4p7",
            constructor_name: accounts_fireworks_models_glm_4p7,
            display_name: "GLM 4.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsGlm5 {
            model_name: "accounts/fireworks/models/glm-5",
            constructor_name: accounts_fireworks_models_glm_5,
            display_name: "GLM 5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsGlm5p1 {
            model_name: "accounts/fireworks/models/glm-5p1",
            constructor_name: accounts_fireworks_models_glm_5p1,
            display_name: "GLM 5.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsGptOss120b {
            model_name: "accounts/fireworks/models/gpt-oss-120b",
            constructor_name: accounts_fireworks_models_gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsGptOss20b {
            model_name: "accounts/fireworks/models/gpt-oss-20b",
            constructor_name: accounts_fireworks_models_gpt_oss_20b,
            display_name: "GPT OSS 20B",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsKimiK2Instruct {
            model_name: "accounts/fireworks/models/kimi-k2-instruct",
            constructor_name: accounts_fireworks_models_kimi_k2_instruct,
            display_name: "Kimi K2 Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsKimiK2Thinking {
            model_name: "accounts/fireworks/models/kimi-k2-thinking",
            constructor_name: accounts_fireworks_models_kimi_k2_thinking,
            display_name: "Kimi K2 Thinking",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsKimiK2p5 {
            model_name: "accounts/fireworks/models/kimi-k2p5",
            constructor_name: accounts_fireworks_models_kimi_k2p5,
            display_name: "Kimi K2.5",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AccountsFireworksModelsKimiK2p6 {
            model_name: "accounts/fireworks/models/kimi-k2p6",
            constructor_name: accounts_fireworks_models_kimi_k2p6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsMinimaxM2p1 {
            model_name: "accounts/fireworks/models/minimax-m2p1",
            constructor_name: accounts_fireworks_models_minimax_m2p1,
            display_name: "MiniMax-M2.1",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsMinimaxM2p5 {
            model_name: "accounts/fireworks/models/minimax-m2p5",
            constructor_name: accounts_fireworks_models_minimax_m2p5,
            display_name: "MiniMax-M2.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsMinimaxM2p7 {
            model_name: "accounts/fireworks/models/minimax-m2p7",
            constructor_name: accounts_fireworks_models_minimax_m2p7,
            display_name: "MiniMax-M2.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsQwen3p6Plus {
            model_name: "accounts/fireworks/models/qwen3p6-plus",
            constructor_name: accounts_fireworks_models_qwen3p6_plus,
            display_name: "Qwen 3.6 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksRoutersKimiK2p5Turbo {
            model_name: "accounts/fireworks/routers/kimi-k2p5-turbo",
            constructor_name: accounts_fireworks_routers_kimi_k2p5_turbo,
            display_name: "Kimi K2.5 Turbo",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
