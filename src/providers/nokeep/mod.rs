//! This module provides the Nokeep provider, wrapping OpenAI Chat Completions for Nokeep requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    NokeepProviderSettings,
    NokeepProviderSettingsBuilder,
    "Nokeep",
    "https://nokeep.ai/v1",
    "NOKEEP_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Nokeep, NokeepBuilder, NokeepProviderSettings, "nokeep");

// Generate the language model implementation
crate::openai_compatible_language_model!(Nokeep);
