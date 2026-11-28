//! This module provides the Steadyrouter provider, wrapping OpenAI Chat Completions for Steadyrouter requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    SteadyrouterProviderSettings,
    SteadyrouterProviderSettingsBuilder,
    "Steadyrouter",
    "https://api.steadyrouter.com/v1",
    "STEADYROUTER_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Steadyrouter,
    SteadyrouterBuilder,
    SteadyrouterProviderSettings,
    "steadyrouter"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Steadyrouter);
