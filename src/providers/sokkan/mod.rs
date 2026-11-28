//! This module provides the Sokkan provider, wrapping OpenAI Chat Completions for Sokkan requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    SokkanProviderSettings,
    SokkanProviderSettingsBuilder,
    "Sokkan",
    "https://router.sokkan.ch/v1",
    "SOKKAN_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Sokkan, SokkanBuilder, SokkanProviderSettings, "sokkan");

// Generate the language model implementation
crate::openai_compatible_language_model!(Sokkan);
