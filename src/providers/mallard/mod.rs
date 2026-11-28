//! This module provides the Mallard provider, wrapping OpenAI Chat Completions for Mallard requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    MallardProviderSettings,
    MallardProviderSettingsBuilder,
    "Mallard",
    "https://api.mallardai.co.uk/v1",
    "MALLARD_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Mallard, MallardBuilder, MallardProviderSettings, "mallard");

// Generate the language model implementation
crate::openai_compatible_language_model!(Mallard);
