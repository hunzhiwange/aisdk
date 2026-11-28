//! This module provides the Therouter provider, wrapping OpenAI Chat Completions for Therouter requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    TherouterProviderSettings,
    TherouterProviderSettingsBuilder,
    "Therouter",
    "https://api.therouter.ai/v1",
    "THEROUTER_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Therouter,
    TherouterBuilder,
    TherouterProviderSettings,
    "therouter"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Therouter);
