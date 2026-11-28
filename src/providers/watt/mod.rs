//! This module provides the Watt provider, wrapping OpenAI Chat Completions for Watt requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    WattProviderSettings,
    WattProviderSettingsBuilder,
    "Watt",
    "https://api.watt-cloud.com/v1",
    "WATT_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Watt, WattBuilder, WattProviderSettings, "watt");

// Generate the language model implementation
crate::openai_compatible_language_model!(Watt);
