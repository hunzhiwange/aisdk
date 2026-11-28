//! This module provides the Verda provider, wrapping OpenAI Chat Completions for Verda requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    VerdaProviderSettings,
    VerdaProviderSettingsBuilder,
    "Verda",
    "https://inference.verda.com/v1",
    "VERDA_INFERENCE_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Verda, VerdaBuilder, VerdaProviderSettings, "verda");

// Generate the language model implementation
crate::openai_compatible_language_model!(Verda);
