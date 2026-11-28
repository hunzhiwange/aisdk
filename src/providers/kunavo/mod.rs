//! This module provides the Kunavo provider, wrapping OpenAI Chat Completions for Kunavo requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    KunavoProviderSettings,
    KunavoProviderSettingsBuilder,
    "Kunavo",
    "https://api.kunavo.com/v1",
    "KUNAVO_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Kunavo, KunavoBuilder, KunavoProviderSettings, "kunavo");

// Generate the language model implementation
crate::openai_compatible_language_model!(Kunavo);
