//! This module provides the SteppingStone provider, wrapping OpenAI Chat Completions for SteppingStone requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    SteppingStoneProviderSettings,
    SteppingStoneProviderSettingsBuilder,
    "SteppingStone",
    "https://llm.stoney-cloud.com/v1",
    "STONEY_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    SteppingStone,
    SteppingStoneBuilder,
    SteppingStoneProviderSettings,
    "stepping-stone"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(SteppingStone);
