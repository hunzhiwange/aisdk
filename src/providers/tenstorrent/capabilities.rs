//! Capabilities for tenstorrent models.
//!
//! This module defines model types and their capabilities for tenstorrent providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::tenstorrent::Tenstorrent;

model_capabilities! {
    provider: Tenstorrent,
    models: {
        MinimaxaiMinimaxM31Flash {
            model_name: "MiniMaxAI/MiniMax-M3.1-Flash",
            constructor_name: minimaxai_minimax_m3_1_flash,
            display_name: "MiniMax-M3.1-Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen3827b {
            model_name: "Qwen/Qwen3.8-27B",
            constructor_name: qwen_qwen3_8_27b,
            display_name: "Qwen3.8 27B",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        ZaiOrgGlm53 {
            model_name: "zai-org/GLM-5.3",
            constructor_name: zai_org_glm_5_3,
            display_name: "GLM-5.3",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZaiOrgGlm53Flash {
            model_name: "zai-org/GLM-5.3-Flash",
            constructor_name: zai_org_glm_5_3_flash,
            display_name: "GLM-5.3-Flash",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
    }
}
