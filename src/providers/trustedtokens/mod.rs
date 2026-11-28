//! This module provides the Trustedtokens provider, wrapping OpenAI Chat Completions for Trustedtokens requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    TrustedtokensProviderSettings,
    TrustedtokensProviderSettingsBuilder,
    "Trustedtokens",
    "https://api.trustedtokens.eu/v1",
    "TRUSTEDTOKENS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Trustedtokens,
    TrustedtokensBuilder,
    TrustedtokensProviderSettings,
    "trustedtokens"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Trustedtokens);
