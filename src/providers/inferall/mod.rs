//! This module provides the Inferall provider, wrapping OpenAI Chat Completions for Inferall requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    InferallProviderSettings,
    InferallProviderSettingsBuilder,
    "Inferall",
    "https://api.inferall.ai/v1",
    "INFERALL_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Inferall,
    InferallBuilder,
    InferallProviderSettings,
    "inferall"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Inferall);
