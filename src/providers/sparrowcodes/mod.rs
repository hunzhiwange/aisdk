//! This module provides the Sparrowcodes provider, wrapping OpenAI Chat Completions for Sparrowcodes requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    SparrowcodesProviderSettings,
    SparrowcodesProviderSettingsBuilder,
    "Sparrowcodes",
    "https://api.sparrowcodes.com/v1",
    "SPARROWCODES_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Sparrowcodes,
    SparrowcodesBuilder,
    SparrowcodesProviderSettings,
    "sparrowcodes"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Sparrowcodes);
