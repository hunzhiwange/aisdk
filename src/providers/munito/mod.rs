//! This module provides the Munito provider, wrapping OpenAI Chat Completions for Munito requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    MunitoProviderSettings,
    MunitoProviderSettingsBuilder,
    "Munito",
    "https://api.munito.ai/inference/v1",
    "MUNITO_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Munito, MunitoBuilder, MunitoProviderSettings, "munito");

// Generate the language model implementation
crate::openai_compatible_language_model!(Munito);
