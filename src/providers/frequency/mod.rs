//! This module provides the Frequency provider, wrapping OpenAI Chat Completions for Frequency requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    FrequencyProviderSettings,
    FrequencyProviderSettingsBuilder,
    "Frequency",
    "https://llm.scalefrequency.com/v2",
    "FREQUENCY_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Frequency,
    FrequencyBuilder,
    FrequencyProviderSettings,
    "frequency"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Frequency);
