//! This module provides the Tokenwatt provider, wrapping OpenAI Chat Completions for Tokenwatt requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    TokenwattProviderSettings,
    TokenwattProviderSettingsBuilder,
    "Tokenwatt",
    "https://api.tokenwatt.io/v1",
    "TOKENWATT_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Tokenwatt,
    TokenwattBuilder,
    TokenwattProviderSettings,
    "tokenwatt"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Tokenwatt);
