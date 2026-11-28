//! This module provides the Byteplus provider, wrapping OpenAI Chat Completions for Byteplus requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    ByteplusProviderSettings,
    ByteplusProviderSettingsBuilder,
    "Byteplus",
    "https://ark.ap-southeast.bytepluses.com/api/v3",
    "BYTEPLUS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Byteplus,
    ByteplusBuilder,
    ByteplusProviderSettings,
    "byteplus"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Byteplus);
