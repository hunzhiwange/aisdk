//! This module provides the Teamorouter provider, wrapping OpenAI Chat Completions for Teamorouter requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    TeamorouterProviderSettings,
    TeamorouterProviderSettingsBuilder,
    "Teamorouter",
    "https://api.teamorouter.com/v1",
    "TEAMOROUTER_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Teamorouter,
    TeamorouterBuilder,
    TeamorouterProviderSettings,
    "teamorouter"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Teamorouter);
