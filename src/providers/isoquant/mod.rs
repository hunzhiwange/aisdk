//! This module provides the Isoquant provider, wrapping OpenAI Chat Completions for Isoquant requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    IsoquantProviderSettings,
    IsoquantProviderSettingsBuilder,
    "Isoquant",
    "https://api.isoquant.ai/v1",
    "ISOQUANT_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Isoquant,
    IsoquantBuilder,
    IsoquantProviderSettings,
    "isoquant"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Isoquant);
