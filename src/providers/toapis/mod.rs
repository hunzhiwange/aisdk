//! This module provides the Toapis provider, wrapping OpenAI Chat Completions for Toapis requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ToapisProviderSettings,
    ToapisProviderSettingsBuilder,
    "Toapis",
    "https://toapis.com/v1",
    "TOAPIS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Toapis, ToapisBuilder, ToapisProviderSettings, "toapis");

// Generate the language model implementation
crate::openai_compatible_language_model!(Toapis);
