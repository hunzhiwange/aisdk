//! This module provides the Paraloncloud provider, wrapping OpenAI Chat Completions for Paraloncloud requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ParaloncloudProviderSettings,
    ParaloncloudProviderSettingsBuilder,
    "Paraloncloud",
    "https://paraloncloud.com/v1",
    "PARALONCLOUD_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Paraloncloud,
    ParaloncloudBuilder,
    ParaloncloudProviderSettings,
    "paraloncloud"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Paraloncloud);
