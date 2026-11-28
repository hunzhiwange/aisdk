//! This module provides the Outerlabs provider, wrapping OpenAI Chat Completions for Outerlabs requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    OuterlabsProviderSettings,
    OuterlabsProviderSettingsBuilder,
    "Outerlabs",
    "https://api.outerlabs.ai/v1",
    "OUTERLABS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Outerlabs,
    OuterlabsBuilder,
    OuterlabsProviderSettings,
    "outerlabs"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Outerlabs);
