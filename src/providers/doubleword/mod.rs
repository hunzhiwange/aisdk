//! This module provides the Doubleword provider, wrapping OpenAI Chat Completions for Doubleword requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    DoublewordProviderSettings,
    DoublewordProviderSettingsBuilder,
    "Doubleword",
    "https://api.doubleword.ai/v1",
    "DOUBLEWORD_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Doubleword,
    DoublewordBuilder,
    DoublewordProviderSettings,
    "doubleword"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Doubleword);
