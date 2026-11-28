//! This module provides the Nodecode provider, wrapping OpenAI Chat Completions for Nodecode requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    NodecodeProviderSettings,
    NodecodeProviderSettingsBuilder,
    "Nodecode",
    "https://api.nodecode.ai/v1",
    "NODECODE_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Nodecode,
    NodecodeBuilder,
    NodecodeProviderSettings,
    "nodecode"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Nodecode);
