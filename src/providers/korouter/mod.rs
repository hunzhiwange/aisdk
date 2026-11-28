//! This module provides the Korouter provider, wrapping OpenAI Chat Completions for Korouter requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    KorouterProviderSettings,
    KorouterProviderSettingsBuilder,
    "Korouter",
    "https://korouter.ai/v1",
    "KOROUTER_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Korouter,
    KorouterBuilder,
    KorouterProviderSettings,
    "korouter"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Korouter);
