//! Capabilities for moneo models.
//!
//! This module defines model types and their capabilities for moneo providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::moneo::Moneo;

model_capabilities! {
    provider: Moneo,
    models: {
        ClaudeHaiku55 {
            model_name: "Claude Haiku 5.5",
            constructor_name: claude_haiku_5_5,
            display_name: "Claude Haiku 5.5",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MinimaxM27Web {
            model_name: "MiniMax-M2.7:web",
            constructor_name: minimax_m2_7_web,
            display_name: "MiniMax M2.7 (Web)",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV41Flash1017Uncensored {
            model_name: "deepseek-v4-1-flash-1017-uncensored",
            constructor_name: deepseek_v4_1_flash_1017_uncensored,
            display_name: "DeepSeek V4.1 Flash 1017 Uncensored",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        DeepseekV4FlashWeb {
            model_name: "deepseek-v4-flash:web",
            constructor_name: deepseek_v4_flash_web,
            display_name: "DeepSeek V4 Flash (Web)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gemma431bWeb {
            model_name: "gemma-4-31b:web",
            constructor_name: gemma_4_31b_web,
            display_name: "Gemma 4 31B (Web)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Gpt56Luna {
            model_name: "gpt-5-6-luna",
            constructor_name: gpt_5_6_luna,
            display_name: "GPT-5.6 Luna",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GptOss120b {
            model_name: "gpt-oss-120b",
            constructor_name: gpt_oss_120b,
            display_name: "GPT OSS 120B",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        KimiK27CodeWeb {
            model_name: "kimi-k2.7-code:web",
            constructor_name: kimi_k2_7_code_web,
            display_name: "Kimi K2.7 Code (Web)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        NvidiaNemotron35SuperVl120bA12b {
            model_name: "nvidia-nemotron-3-5-super-vl-120b-a12b",
            constructor_name: nvidia_nemotron_3_5_super_vl_120b_a12b,
            display_name: "Nemotron 3.5 Super VL 120B",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3235bWeb {
            model_name: "qwen3-235b:web",
            constructor_name: qwen3_235b_web,
            display_name: "Qwen3 235B (Web)",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3Coder480bA35bInstruct {
            model_name: "qwen3-coder-480b-a35b-instruct",
            constructor_name: qwen3_coder_480b_a35b_instruct,
            display_name: "Qwen3-Coder 480B-A35B Instruct",
            capabilities: [TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        ZAiGlm53Flash {
            model_name: "z-ai-glm-5-3-flash",
            constructor_name: z_ai_glm_5_3_flash,
            display_name: "GLM 5.3 Flash",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
