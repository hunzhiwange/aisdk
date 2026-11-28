//! This module provides the ConfidentialAi provider, wrapping OpenAI Chat Completions for ConfidentialAi requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ConfidentialAiProviderSettings,
    ConfidentialAiProviderSettingsBuilder,
    "ConfidentialAi",
    "https://api.confidential.ai/v1",
    "CONFIDENTIAL_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    ConfidentialAi,
    ConfidentialAiBuilder,
    ConfidentialAiProviderSettings,
    "confidential-ai"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(ConfidentialAi);
