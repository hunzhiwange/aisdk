//! This module provides the Lithosai provider, wrapping OpenAI Chat Completions for Lithosai requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    LithosaiProviderSettings,
    LithosaiProviderSettingsBuilder,
    "Lithosai",
    "https://api.lithosai.cloud/v1",
    "LITHOSAI_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Lithosai,
    LithosaiBuilder,
    LithosaiProviderSettings,
    "lithosai"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Lithosai);
