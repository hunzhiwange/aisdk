//! This module provides the Keln provider, wrapping OpenAI Chat Completions for Keln requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    KelnProviderSettings,
    KelnProviderSettingsBuilder,
    "Keln",
    "https://api.keln.ai/v1",
    "KELN_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Keln, KelnBuilder, KelnProviderSettings, "keln");

// Generate the language model implementation
crate::openai_compatible_language_model!(Keln);
