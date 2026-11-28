//! This module provides the Graphn provider, wrapping OpenAI Chat Completions for Graphn requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    GraphnProviderSettings,
    GraphnProviderSettingsBuilder,
    "Graphn",
    "https://model.graphn.ai/v1",
    "GRAPHN_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Graphn, GraphnBuilder, GraphnProviderSettings, "graphn");

// Generate the language model implementation
crate::openai_compatible_language_model!(Graphn);
