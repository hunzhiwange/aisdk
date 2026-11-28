//! This module provides the Humain provider, wrapping OpenAI Chat Completions for Humain requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    HumainProviderSettings,
    HumainProviderSettingsBuilder,
    "Humain",
    "https://api.node.humainaic.com/v1",
    "HUMAIN_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Humain, HumainBuilder, HumainProviderSettings, "humain");

// Generate the language model implementation
crate::openai_compatible_language_model!(Humain);
