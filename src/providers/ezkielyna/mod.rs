//! This module provides the Ezkielyna provider, wrapping OpenAI Chat Completions for Ezkielyna requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    EzkielynaProviderSettings,
    EzkielynaProviderSettingsBuilder,
    "Ezkielyna",
    "https://api.ezkielyna.store/v1",
    "EZKIELYNA_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Ezkielyna,
    EzkielynaBuilder,
    EzkielynaProviderSettings,
    "ezkielyna"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Ezkielyna);
