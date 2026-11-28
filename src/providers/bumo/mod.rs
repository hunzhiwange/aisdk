//! This module provides the Bumo provider, wrapping OpenAI Chat Completions for Bumo requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    BumoProviderSettings,
    BumoProviderSettingsBuilder,
    "Bumo",
    "https://router.bumo.ai/v1",
    "BUMO_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Bumo, BumoBuilder, BumoProviderSettings, "bumo");

// Generate the language model implementation
crate::openai_compatible_language_model!(Bumo);
