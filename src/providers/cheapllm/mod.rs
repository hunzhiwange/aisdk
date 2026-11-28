//! This module provides the Cheapllm provider, wrapping OpenAI Chat Completions for Cheapllm requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    CheapllmProviderSettings,
    CheapllmProviderSettingsBuilder,
    "Cheapllm",
    "https://cheapllm.shop/v1",
    "CHEAPLLM_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Cheapllm,
    CheapllmBuilder,
    CheapllmProviderSettings,
    "cheapllm"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Cheapllm);
