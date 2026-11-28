//! This module provides the MunitoEu provider, wrapping OpenAI Chat Completions for MunitoEu requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    MunitoEuProviderSettings,
    MunitoEuProviderSettingsBuilder,
    "MunitoEu",
    "https://api.eu.munito.ai/inference/v1",
    "MUNITO_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    MunitoEu,
    MunitoEuBuilder,
    MunitoEuProviderSettings,
    "munito-eu"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(MunitoEu);
