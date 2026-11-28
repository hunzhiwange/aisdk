//! This module provides the Infersia provider, wrapping OpenAI Chat Completions for Infersia requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    InfersiaProviderSettings,
    InfersiaProviderSettingsBuilder,
    "Infersia",
    "https://api.infersia.com/v1",
    "INFERSIA_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Infersia,
    InfersiaBuilder,
    InfersiaProviderSettings,
    "infersia"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Infersia);
