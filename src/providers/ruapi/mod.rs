//! This module provides the Ruapi provider, wrapping OpenAI Chat Completions for Ruapi requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    RuapiProviderSettings,
    RuapiProviderSettingsBuilder,
    "Ruapi",
    "https://www.ruapi.ai/v1",
    "RUAPI_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Ruapi, RuapiBuilder, RuapiProviderSettings, "ruapi");

// Generate the language model implementation
crate::openai_compatible_language_model!(Ruapi);
