//! This module provides the Anonrouter provider, wrapping OpenAI Chat Completions for Anonrouter requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    AnonrouterProviderSettings,
    AnonrouterProviderSettingsBuilder,
    "Anonrouter",
    "https://api.anonrouter.ai/v1",
    "ANONROUTER_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Anonrouter,
    AnonrouterBuilder,
    AnonrouterProviderSettings,
    "anonrouter"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Anonrouter);
