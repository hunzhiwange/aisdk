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
        AccountsFireworksModelsDeepseekV4Flash {
            model_name: "accounts/fireworks/models/deepseek-v4-flash",
            constructor_name: accounts_fireworks_models_deepseek_v4_flash,
            display_name: "DeepSeek V4 Flash",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsDeepseekV4Flash0731 {
            model_name: "accounts/fireworks/models/deepseek-v4-flash-0731",
            constructor_name: accounts_fireworks_models_deepseek_v4_flash_0731,
            display_name: "DeepSeek V4 Flash 0731",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsDeepseekV4Pro {
            model_name: "accounts/fireworks/models/deepseek-v4-pro",
            constructor_name: accounts_fireworks_models_deepseek_v4_pro,
            display_name: "DeepSeek V4 Pro",
            capabilities: [ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsGlm5p2 {
            model_name: "accounts/fireworks/models/glm-5p2",
            constructor_name: accounts_fireworks_models_glm_5p2,
            display_name: "GLM 5.2",
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
        AccountsFireworksModelsKimiK2p6 {
            model_name: "accounts/fireworks/models/kimi-k2p6",
            constructor_name: accounts_fireworks_models_kimi_k2p6,
            display_name: "Kimi K2.6",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsKimiK2p7Code {
            model_name: "accounts/fireworks/models/kimi-k2p7-code",
            constructor_name: accounts_fireworks_models_kimi_k2p7_code,
            display_name: "Kimi K2.7 Code",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsKimiK3 {
            model_name: "accounts/fireworks/models/kimi-k3",
            constructor_name: accounts_fireworks_models_kimi_k3,
            display_name: "Kimi K3",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsMinimaxM2p7 {
            model_name: "accounts/fireworks/models/minimax-m2p7",
            constructor_name: accounts_fireworks_models_minimax_m2p7,
            display_name: "MiniMax-M2.7",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksModelsMinimaxM3 {
            model_name: "accounts/fireworks/models/minimax-m3",
            constructor_name: accounts_fireworks_models_minimax_m3,
            display_name: "MiniMax-M3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        AccountsFireworksModelsQwen3p7Plus {
            model_name: "accounts/fireworks/models/qwen3p7-plus",
            constructor_name: accounts_fireworks_models_qwen3p7_plus,
            display_name: "Qwen 3.7 Plus",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksRoutersGlm5p2Fast {
            model_name: "accounts/fireworks/routers/glm-5p2-fast",
            constructor_name: accounts_fireworks_routers_glm_5p2_fast,
            display_name: "GLM 5.2 Fast",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksRoutersKimiK2p6Fast {
            model_name: "accounts/fireworks/routers/kimi-k2p6-fast",
            constructor_name: accounts_fireworks_routers_kimi_k2p6_fast,
            display_name: "Kimi K2.6 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksRoutersKimiK2p6Turbo {
            model_name: "accounts/fireworks/routers/kimi-k2p6-turbo",
            constructor_name: accounts_fireworks_routers_kimi_k2p6_turbo,
            display_name: "Kimi K2.6 Turbo",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksRoutersKimiK2p7CodeFast {
            model_name: "accounts/fireworks/routers/kimi-k2p7-code-fast",
            constructor_name: accounts_fireworks_routers_kimi_k2p7_code_fast,
            display_name: "Kimi K2.7 Code Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        AccountsFireworksRoutersKimiK3Fast {
            model_name: "accounts/fireworks/routers/kimi-k3-fast",
            constructor_name: accounts_fireworks_routers_kimi_k3_fast,
            display_name: "Kimi K3 Fast",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
