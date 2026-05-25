//! Capabilities for xai models.
//!
//! This module defines model types and their capabilities for xai providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::xai::XAI;

model_capabilities! {
    provider: XAI,
    models: {
        Grok4200309NonReasoning {
            model_name: "grok-4.20-0309-non-reasoning",
            constructor_name: grok_4_20_0309_non_reasoning,
            display_name: "Grok 4.20 (Non-Reasoning)",
            capabilities: [ImageInputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok4200309Reasoning {
            model_name: "grok-4.20-0309-reasoning",
            constructor_name: grok_4_20_0309_reasoning,
            display_name: "Grok 4.20 (Reasoning)",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        Grok420MultiAgent0309 {
            model_name: "grok-4.20-multi-agent-0309",
            constructor_name: grok_4_20_multi_agent_0309,
            display_name: "Grok 4.20 Multi-Agent",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport]
        },
        Grok43 {
            model_name: "grok-4.3",
            constructor_name: grok_4_3,
            display_name: "Grok 4.3",
            capabilities: [ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GrokBuild01 {
            model_name: "grok-build-0.1",
            constructor_name: grok_build_0_1,
            display_name: "Grok Build 0.1",
            capabilities: [ImageInputSupport, ReasoningSupport, StructuredOutputSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        GrokImagineImage {
            model_name: "grok-imagine-image",
            constructor_name: grok_imagine_image,
            display_name: "Grok Imagine Image",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        GrokImagineImageQuality {
            model_name: "grok-imagine-image-quality",
            constructor_name: grok_imagine_image_quality,
            display_name: "Grok Imagine Image Quality",
            capabilities: [ImageInputSupport, ImageOutputSupport, TextInputSupport]
        },
        GrokImagineVideo {
            model_name: "grok-imagine-video",
            constructor_name: grok_imagine_video,
            display_name: "Grok Imagine Video",
            capabilities: [ImageInputSupport, TextInputSupport, VideoOutputSupport]
        },
    }
}
