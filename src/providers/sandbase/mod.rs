//! This module provides the Sandbase provider, wrapping OpenAI Chat Completions for Sandbase requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    SandbaseProviderSettings,
    SandbaseProviderSettingsBuilder,
    "Sandbase",
    "https://api.sandbase.ai/v1",
    "SANDBASE_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Sandbase,
    SandbaseBuilder,
    SandbaseProviderSettings,
    "sandbase"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Sandbase);
