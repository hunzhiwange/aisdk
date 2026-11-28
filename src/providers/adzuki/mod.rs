//! This module provides the Adzuki provider, wrapping OpenAI Chat Completions for Adzuki requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    AdzukiProviderSettings,
    AdzukiProviderSettingsBuilder,
    "Adzuki",
    "https://adzukiai.com/v1",
    "ADZUKI_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Adzuki, AdzukiBuilder, AdzukiProviderSettings, "adzuki");

// Generate the language model implementation
crate::openai_compatible_language_model!(Adzuki);
