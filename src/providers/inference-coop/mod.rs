//! This module provides the InferenceCoop provider, wrapping OpenAI Chat Completions for InferenceCoop requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    InferenceCoopProviderSettings,
    InferenceCoopProviderSettingsBuilder,
    "InferenceCoop",
    "https://gateway.inference.coop/v1",
    "INFERENCE_COOP_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    InferenceCoop,
    InferenceCoopBuilder,
    InferenceCoopProviderSettings,
    "inference-coop"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(InferenceCoop);
