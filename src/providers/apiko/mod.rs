//! This module provides the Apiko provider, wrapping OpenAI Chat Completions for Apiko requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ApikoProviderSettings,
    ApikoProviderSettingsBuilder,
    "Apiko",
    "https://api.apiko.app/v1",
    "APIKO_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Apiko, ApikoBuilder, ApikoProviderSettings, "apiko");

// Generate the language model implementation
crate::openai_compatible_language_model!(Apiko);
