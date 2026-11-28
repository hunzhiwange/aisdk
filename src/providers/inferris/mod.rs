//! This module provides the Inferris provider, wrapping OpenAI Chat Completions for Inferris requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    InferrisProviderSettings,
    InferrisProviderSettingsBuilder,
    "Inferris",
    "https://api.inferris.ai/v1",
    "INFERRIS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Inferris,
    InferrisBuilder,
    InferrisProviderSettings,
    "inferris"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Inferris);
