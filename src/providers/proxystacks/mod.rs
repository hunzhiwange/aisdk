//! This module provides the Proxystacks provider, wrapping OpenAI Chat Completions for Proxystacks requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ProxystacksProviderSettings,
    ProxystacksProviderSettingsBuilder,
    "Proxystacks",
    "https://proxystacks.com/v1",
    "RELAY_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Proxystacks,
    ProxystacksBuilder,
    ProxystacksProviderSettings,
    "proxystacks"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Proxystacks);
