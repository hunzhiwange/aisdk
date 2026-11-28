//! This module provides the Aistupidlevel provider, wrapping OpenAI Chat Completions for Aistupidlevel requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    AistupidlevelProviderSettings,
    AistupidlevelProviderSettingsBuilder,
    "Aistupidlevel",
    "https://aistupidlevel.info/v1",
    "AISTUPIDLEVEL_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Aistupidlevel,
    AistupidlevelBuilder,
    AistupidlevelProviderSettings,
    "aistupidlevel"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Aistupidlevel);
