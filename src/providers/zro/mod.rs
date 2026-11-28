//! This module provides the Zro provider, wrapping OpenAI Chat Completions for Zro requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ZroProviderSettings,
    ZroProviderSettingsBuilder,
    "Zro",
    "https://zro.moonmath.ai/v1",
    "ZRO_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Zro, ZroBuilder, ZroProviderSettings, "zro");

// Generate the language model implementation
crate::openai_compatible_language_model!(Zro);
