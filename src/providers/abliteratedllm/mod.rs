//! This module provides the Abliteratedllm provider, wrapping OpenAI Chat Completions for Abliteratedllm requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    AbliteratedllmProviderSettings,
    AbliteratedllmProviderSettingsBuilder,
    "Abliteratedllm",
    "https://api.abliteratedllm.dev/v1",
    "ABLITERATEDLLM_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Abliteratedllm,
    AbliteratedllmBuilder,
    AbliteratedllmProviderSettings,
    "abliteratedllm"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Abliteratedllm);
