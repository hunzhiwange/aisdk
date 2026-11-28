//! This module provides the Modular provider, wrapping OpenAI Chat Completions for Modular requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ModularProviderSettings,
    ModularProviderSettingsBuilder,
    "Modular",
    "https://api.modular.com/v1",
    "MODULAR_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Modular, ModularBuilder, ModularProviderSettings, "modular");

// Generate the language model implementation
crate::openai_compatible_language_model!(Modular);
