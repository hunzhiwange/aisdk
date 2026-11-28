//! This module provides the Kitani provider, wrapping OpenAI Chat Completions for Kitani requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    KitaniProviderSettings,
    KitaniProviderSettingsBuilder,
    "Kitani",
    "https://api.kitani.ai/v1",
    "KITANI_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Kitani, KitaniBuilder, KitaniProviderSettings, "kitani");

// Generate the language model implementation
crate::openai_compatible_language_model!(Kitani);
