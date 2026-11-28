//! This module provides the Refuseless provider, wrapping OpenAI Chat Completions for Refuseless requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    RefuselessProviderSettings,
    RefuselessProviderSettingsBuilder,
    "Refuseless",
    "https://api.refuseless.com/v1",
    "REFUSELESS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Refuseless,
    RefuselessBuilder,
    RefuselessProviderSettings,
    "refuseless"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Refuseless);
