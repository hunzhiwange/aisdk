//! This module provides the Adaption provider, wrapping OpenAI Chat Completions for Adaption requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    AdaptionProviderSettings,
    AdaptionProviderSettingsBuilder,
    "Adaption",
    "https://api.prod.adaptionlabs.ai/api/v1",
    "ADAPTION_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Adaption,
    AdaptionBuilder,
    AdaptionProviderSettings,
    "adaption"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Adaption);
