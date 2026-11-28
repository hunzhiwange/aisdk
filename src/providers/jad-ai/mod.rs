//! This module provides the JadAi provider, wrapping OpenAI Chat Completions for JadAi requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    JadAiProviderSettings,
    JadAiProviderSettingsBuilder,
    "JadAi",
    "https://jadai.freesrv.com/v1",
    "JAD_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(JadAi, JadAiBuilder, JadAiProviderSettings, "jad-ai");

// Generate the language model implementation
crate::openai_compatible_language_model!(JadAi);
