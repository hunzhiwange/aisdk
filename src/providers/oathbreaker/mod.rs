//! This module provides the Oathbreaker provider, wrapping OpenAI Chat Completions for Oathbreaker requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    OathbreakerProviderSettings,
    OathbreakerProviderSettingsBuilder,
    "Oathbreaker",
    "https://oathbreaker.ai/v1",
    "OATHBREAKER_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Oathbreaker,
    OathbreakerBuilder,
    OathbreakerProviderSettings,
    "oathbreaker"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Oathbreaker);
