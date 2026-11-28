//! This module provides the Entrim provider, wrapping OpenAI Chat Completions for Entrim requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    EntrimProviderSettings,
    EntrimProviderSettingsBuilder,
    "Entrim",
    "https://api.entrim.ai/v1",
    "ENTRIM_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Entrim, EntrimBuilder, EntrimProviderSettings, "entrim");

// Generate the language model implementation
crate::openai_compatible_language_model!(Entrim);
