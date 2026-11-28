//! This module provides the Tokenlab provider, wrapping OpenAI Chat Completions for Tokenlab requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    TokenlabProviderSettings,
    TokenlabProviderSettingsBuilder,
    "Tokenlab",
    "https://api.tokenlab.sh/v1",
    "TOKENLAB_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Tokenlab,
    TokenlabBuilder,
    TokenlabProviderSettings,
    "tokenlab"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Tokenlab);
