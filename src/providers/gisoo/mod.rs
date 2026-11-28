//! This module provides the Gisoo provider, wrapping OpenAI Chat Completions for Gisoo requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    GisooProviderSettings,
    GisooProviderSettingsBuilder,
    "Gisoo",
    "https://gisoo.pro/api/v1",
    "GISOO_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Gisoo, GisooBuilder, GisooProviderSettings, "gisoo");

// Generate the language model implementation
crate::openai_compatible_language_model!(Gisoo);
