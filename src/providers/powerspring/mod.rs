//! This module provides the Powerspring provider, wrapping OpenAI Chat Completions for Powerspring requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    PowerspringProviderSettings,
    PowerspringProviderSettingsBuilder,
    "Powerspring",
    "https://api.powerspring.eu/v1",
    "POWERSPRING_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Powerspring,
    PowerspringBuilder,
    PowerspringProviderSettings,
    "powerspring"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Powerspring);
