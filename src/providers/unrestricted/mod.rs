//! This module provides the Unrestricted provider, wrapping OpenAI Chat Completions for Unrestricted requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    UnrestrictedProviderSettings,
    UnrestrictedProviderSettingsBuilder,
    "Unrestricted",
    "https://api.unrestricted.sh/v1",
    "UNRESTRICTED_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Unrestricted,
    UnrestrictedBuilder,
    UnrestrictedProviderSettings,
    "unrestricted"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Unrestricted);
