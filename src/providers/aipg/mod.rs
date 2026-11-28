//! This module provides the Aipg provider, wrapping OpenAI Chat Completions for Aipg requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    AipgProviderSettings,
    AipgProviderSettingsBuilder,
    "Aipg",
    "https://api.aipowergrid.io/v1",
    "AIPG_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Aipg, AipgBuilder, AipgProviderSettings, "aipg");

// Generate the language model implementation
crate::openai_compatible_language_model!(Aipg);
