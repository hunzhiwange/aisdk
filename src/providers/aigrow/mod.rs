//! This module provides the Aigrow provider, wrapping OpenAI Chat Completions for Aigrow requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    AigrowProviderSettings,
    AigrowProviderSettingsBuilder,
    "Aigrow",
    "https://aigrow.gg/v1",
    "AIGROW_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Aigrow, AigrowBuilder, AigrowProviderSettings, "aigrow");

// Generate the language model implementation
crate::openai_compatible_language_model!(Aigrow);
