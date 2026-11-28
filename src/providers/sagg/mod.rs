//! This module provides the Sagg provider, wrapping OpenAI Chat Completions for Sagg requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    SaggProviderSettings,
    SaggProviderSettingsBuilder,
    "Sagg",
    "https://api.privatedeskai.com/v1",
    "SAGG_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Sagg, SaggBuilder, SaggProviderSettings, "sagg");

// Generate the language model implementation
crate::openai_compatible_language_model!(Sagg);
