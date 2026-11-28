//! This module provides the Dothoosh provider, wrapping OpenAI Chat Completions for Dothoosh requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    DothooshProviderSettings,
    DothooshProviderSettingsBuilder,
    "Dothoosh",
    "https://gateway.dothoosh.com/v1",
    "DOTHOOSH_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Dothoosh,
    DothooshBuilder,
    DothooshProviderSettings,
    "dothoosh"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Dothoosh);
