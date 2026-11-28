//! This module provides the Cloudcode provider, wrapping OpenAI Chat Completions for Cloudcode requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    CloudcodeProviderSettings,
    CloudcodeProviderSettingsBuilder,
    "Cloudcode",
    "https://api.cloudcode.one/v1",
    "CLOUDCODE_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Cloudcode,
    CloudcodeBuilder,
    CloudcodeProviderSettings,
    "cloudcode"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Cloudcode);
