//! This module provides the Uplinkswitch provider, wrapping OpenAI Chat Completions for Uplinkswitch requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    UplinkswitchProviderSettings,
    UplinkswitchProviderSettingsBuilder,
    "Uplinkswitch",
    "https://api.uplinkswitch.com/v1",
    "UPLINKSWITCH_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Uplinkswitch,
    UplinkswitchBuilder,
    UplinkswitchProviderSettings,
    "uplinkswitch"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Uplinkswitch);
