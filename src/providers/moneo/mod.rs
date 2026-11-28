//! This module provides the Moneo provider, wrapping OpenAI Chat Completions for Moneo requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    MoneoProviderSettings,
    MoneoProviderSettingsBuilder,
    "Moneo",
    "https://api.moneo.one/v1",
    "MONEO_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Moneo, MoneoBuilder, MoneoProviderSettings, "moneo");

// Generate the language model implementation
crate::openai_compatible_language_model!(Moneo);
