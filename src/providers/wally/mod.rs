//! This module provides the Wally provider, wrapping OpenAI Chat Completions for Wally requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    WallyProviderSettings,
    WallyProviderSettingsBuilder,
    "Wally",
    "https://inference.runanywhere.ai/v1",
    "WALLY_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Wally, WallyBuilder, WallyProviderSettings, "wally");

// Generate the language model implementation
crate::openai_compatible_language_model!(Wally);
