//! This module provides the Phala provider, wrapping OpenAI Chat Completions for Phala requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    PhalaProviderSettings,
    PhalaProviderSettingsBuilder,
    "Phala",
    "https://inference.phala.com/v1",
    "PHALA_AI_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Phala, PhalaBuilder, PhalaProviderSettings, "phala");

// Generate the language model implementation
crate::openai_compatible_language_model!(Phala);
