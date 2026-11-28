//! This module provides the Autarcho provider, wrapping OpenAI Chat Completions for Autarcho requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    AutarchoProviderSettings,
    AutarchoProviderSettingsBuilder,
    "Autarcho",
    "https://api.autarcho.com/v1",
    "AUTARCHO_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Autarcho,
    AutarchoBuilder,
    AutarchoProviderSettings,
    "autarcho"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Autarcho);
