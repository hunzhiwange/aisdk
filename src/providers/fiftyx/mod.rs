//! This module provides the Fiftyx provider, wrapping OpenAI Chat Completions for Fiftyx requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    FiftyxProviderSettings,
    FiftyxProviderSettingsBuilder,
    "Fiftyx",
    "https://api.ai.50x.energy/v1",
    "FIFTYX_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Fiftyx, FiftyxBuilder, FiftyxProviderSettings, "fiftyx");

// Generate the language model implementation
crate::openai_compatible_language_model!(Fiftyx);
