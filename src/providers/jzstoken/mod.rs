//! This module provides the Jzstoken provider, wrapping OpenAI Chat Completions for Jzstoken requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    JzstokenProviderSettings,
    JzstokenProviderSettingsBuilder,
    "Jzstoken",
    "https://jzstoken.com/api/v1",
    "JZSTOKEN_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Jzstoken,
    JzstokenBuilder,
    JzstokenProviderSettings,
    "jzstoken"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Jzstoken);
