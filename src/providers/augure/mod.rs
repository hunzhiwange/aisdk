//! This module provides the Augure provider, wrapping OpenAI Chat Completions for Augure requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    AugureProviderSettings,
    AugureProviderSettingsBuilder,
    "Augure",
    "https://api.augureai.ca/v1",
    "AUGURE_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Augure, AugureBuilder, AugureProviderSettings, "augure");

// Generate the language model implementation
crate::openai_compatible_language_model!(Augure);
