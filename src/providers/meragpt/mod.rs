//! This module provides the Meragpt provider, wrapping OpenAI Chat Completions for Meragpt requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    MeragptProviderSettings,
    MeragptProviderSettingsBuilder,
    "Meragpt",
    "https://meragpt.com/v1",
    "MERAGPT_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Meragpt, MeragptBuilder, MeragptProviderSettings, "meragpt");

// Generate the language model implementation
crate::openai_compatible_language_model!(Meragpt);
