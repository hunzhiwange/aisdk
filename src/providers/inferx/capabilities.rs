//! Capabilities for inferx models.
//!
//! This module defines model types and their capabilities for inferx providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::inferx::Inferx;

model_capabilities! {
    provider: Inferx,
    models: {
        GoogleGemma431bItFp8 {
            model_name: "google/gemma-4-31b-it-fp8",
            constructor_name: google_gemma_4_31b_it_fp8,
            display_name: "Gemma 4 31B IT FP8",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        QwenQwen35122bA10bNvfp4 {
            model_name: "qwen/qwen3.5-122b-a10b-nvfp4",
            constructor_name: qwen_qwen3_5_122b_a10b_nvfp4,
            display_name: "Qwen3.5 122B A10B NVFP4",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3627bFp8 {
            model_name: "qwen/qwen3.6-27b-fp8",
            constructor_name: qwen_qwen3_6_27b_fp8,
            display_name: "Qwen3.6 27B FP8",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        QwenQwen3635bA3bFp8 {
            model_name: "qwen/qwen3.6-35b-a3b-fp8",
            constructor_name: qwen_qwen3_6_35b_a3b_fp8,
            display_name: "Qwen3.6 35B A3B FP8",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        Qwen3CoderNextFp8 {
            model_name: "qwen3-coder-next-fp8",
            constructor_name: qwen3_coder_next_fp8,
            display_name: "Qwen3 Coder Next FP8",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Qwen3CoderNextFp81m {
            model_name: "qwen3-coder-next-fp8-1m",
            constructor_name: qwen3_coder_next_fp8_1m,
            display_name: "Qwen3 Coder Next FP8 1M",
            capabilities: [StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
    }
}
