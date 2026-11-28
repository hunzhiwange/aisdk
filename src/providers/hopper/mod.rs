//! This module provides the Hopper provider, wrapping OpenAI Chat Completions for Hopper requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    HopperProviderSettings,
    HopperProviderSettingsBuilder,
    "Hopper",
    "https://api.withhopper.com/v1",
    "HOPPER_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Hopper, HopperBuilder, HopperProviderSettings, "hopper");

// Generate the language model implementation
crate::openai_compatible_language_model!(Hopper);
