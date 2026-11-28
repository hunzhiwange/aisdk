//! This module provides the Cedarllm provider, wrapping OpenAI Chat Completions for Cedarllm requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    CedarllmProviderSettings,
    CedarllmProviderSettingsBuilder,
    "Cedarllm",
    "https://api.cedarllm.com/v1",
    "CEDARLLM_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Cedarllm,
    CedarllmBuilder,
    CedarllmProviderSettings,
    "cedarllm"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Cedarllm);
