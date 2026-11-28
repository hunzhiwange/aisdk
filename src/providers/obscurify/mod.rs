//! This module provides the Obscurify provider, wrapping OpenAI Chat Completions for Obscurify requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ObscurifyProviderSettings,
    ObscurifyProviderSettingsBuilder,
    "Obscurify",
    "https://obscurify.ai/v1",
    "OBSCURIFY_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Obscurify,
    ObscurifyBuilder,
    ObscurifyProviderSettings,
    "obscurify"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Obscurify);
