//! This module provides the Hubris provider, wrapping OpenAI Chat Completions for Hubris requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    HubrisProviderSettings,
    HubrisProviderSettingsBuilder,
    "Hubris",
    "https://api.hubris.pw/v1",
    "HUBRIS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Hubris, HubrisBuilder, HubrisProviderSettings, "hubris");

// Generate the language model implementation
crate::openai_compatible_language_model!(Hubris);
