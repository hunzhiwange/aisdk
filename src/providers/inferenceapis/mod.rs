//! This module provides the Inferenceapis provider, wrapping OpenAI Chat Completions for Inferenceapis requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    InferenceapisProviderSettings,
    InferenceapisProviderSettingsBuilder,
    "Inferenceapis",
    "https://api.inferenceapis.com/v1",
    "INFERENCEAPIS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Inferenceapis,
    InferenceapisBuilder,
    InferenceapisProviderSettings,
    "inferenceapis"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Inferenceapis);
