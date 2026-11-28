//! This module provides the Tokenify provider, wrapping OpenAI Chat Completions for Tokenify requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    TokenifyProviderSettings,
    TokenifyProviderSettingsBuilder,
    "Tokenify",
    "https://api.tokenify.dev/v1",
    "TOKENIFY_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Tokenify,
    TokenifyBuilder,
    TokenifyProviderSettings,
    "tokenify"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Tokenify);
