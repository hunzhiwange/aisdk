//! This module provides the CheapestToken provider, wrapping OpenAI Chat Completions for CheapestToken requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    CheapestTokenProviderSettings,
    CheapestTokenProviderSettingsBuilder,
    "CheapestToken",
    "https://api.cheapest-token.com/v1",
    "CHEAPEST_TOKEN_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    CheapestToken,
    CheapestTokenBuilder,
    CheapestTokenProviderSettings,
    "cheapest-token"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(CheapestToken);
