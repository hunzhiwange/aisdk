//! This module provides the Xxapi provider, wrapping OpenAI Chat Completions for Xxapi requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    XxapiProviderSettings,
    XxapiProviderSettingsBuilder,
    "Xxapi",
    "https://xxapi.ai/v1",
    "XXAPI_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Xxapi, XxapiBuilder, XxapiProviderSettings, "xxapi");

// Generate the language model implementation
crate::openai_compatible_language_model!(Xxapi);
