//! This module provides the Zerogpu provider, wrapping OpenAI Chat Completions for Zerogpu requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ZerogpuProviderSettings,
    ZerogpuProviderSettingsBuilder,
    "Zerogpu",
    "https://api.zerogpu.ai/v1",
    "ZEROGPU_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Zerogpu, ZerogpuBuilder, ZerogpuProviderSettings, "zerogpu");

// Generate the language model implementation
crate::openai_compatible_language_model!(Zerogpu);
