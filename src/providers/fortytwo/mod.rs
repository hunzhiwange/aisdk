//! This module provides the Fortytwo provider, wrapping OpenAI Chat Completions for Fortytwo requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    FortytwoProviderSettings,
    FortytwoProviderSettingsBuilder,
    "Fortytwo",
    "https://api.fortytwo.network/v1",
    "FORTYTWO_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Fortytwo,
    FortytwoBuilder,
    FortytwoProviderSettings,
    "fortytwo"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Fortytwo);
