//! This module provides the Primalabs provider, wrapping OpenAI Chat Completions for Primalabs requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    PrimalabsProviderSettings,
    PrimalabsProviderSettingsBuilder,
    "Primalabs",
    "https://api.primalabs.ai/v1",
    "PRIMALABS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Primalabs,
    PrimalabsBuilder,
    PrimalabsProviderSettings,
    "primalabs"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Primalabs);
