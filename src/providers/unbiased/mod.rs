//! This module provides the Unbiased provider, wrapping OpenAI Chat Completions for Unbiased requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    UnbiasedProviderSettings,
    UnbiasedProviderSettingsBuilder,
    "Unbiased",
    "https://api.unbiased.ai/v1",
    "UNBIASED_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Unbiased,
    UnbiasedBuilder,
    UnbiasedProviderSettings,
    "unbiased"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Unbiased);
