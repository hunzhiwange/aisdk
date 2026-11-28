//! This module provides the Lyceum provider, wrapping OpenAI Chat Completions for Lyceum requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    LyceumProviderSettings,
    LyceumProviderSettingsBuilder,
    "Lyceum",
    "https://api.lyceum.technology/openai/v1",
    "LYCEUM_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Lyceum, LyceumBuilder, LyceumProviderSettings, "lyceum");

// Generate the language model implementation
crate::openai_compatible_language_model!(Lyceum);
