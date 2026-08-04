//! Capabilities for xiaomi_token_plan_cn models.
//!
//! This module defines model types and their capabilities for xiaomi_token_plan_cn providers.
//! Users can implement additional traits on custom models.

use crate::core::capabilities::*;
use crate::model_capabilities;
use crate::providers::xiaomi_token_plan_cn::XiaomiTokenPlanCn;

model_capabilities! {
    provider: XiaomiTokenPlanCn,
    models: {
        MimoV2Tts {
            model_name: "mimo-v2-tts",
            constructor_name: mimo_v2_tts,
            display_name: "MiMo-V2-TTS",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        MimoV25 {
            model_name: "mimo-v2.5",
            constructor_name: mimo_v2_5,
            display_name: "MiMo-V2.5",
            capabilities: [AudioInputSupport, ImageInputSupport, ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport, VideoInputSupport]
        },
        MimoV25Pro {
            model_name: "mimo-v2.5-pro",
            constructor_name: mimo_v2_5_pro,
            display_name: "MiMo-V2.5-Pro",
            capabilities: [ReasoningSupport, TextInputSupport, TextOutputSupport, ToolCallSupport]
        },
        MimoV25Tts {
            model_name: "mimo-v2.5-tts",
            constructor_name: mimo_v2_5_tts,
            display_name: "MiMo-V2.5-TTS",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        MimoV25TtsVoiceclone {
            model_name: "mimo-v2.5-tts-voiceclone",
            constructor_name: mimo_v2_5_tts_voiceclone,
            display_name: "MiMo-V2.5-TTS-VoiceClone",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
        MimoV25TtsVoicedesign {
            model_name: "mimo-v2.5-tts-voicedesign",
            constructor_name: mimo_v2_5_tts_voicedesign,
            display_name: "MiMo-V2.5-TTS-VoiceDesign",
            capabilities: [AudioOutputSupport, TextInputSupport]
        },
    }
}
