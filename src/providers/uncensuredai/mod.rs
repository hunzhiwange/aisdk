//! This module provides the Uncensuredai provider, wrapping OpenAI Chat Completions for Uncensuredai requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    UncensuredaiProviderSettings,
    UncensuredaiProviderSettingsBuilder,
    "Uncensuredai",
    "https://uncensuredai.com/v1",
    "UNCENSURED_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Uncensuredai,
    UncensuredaiBuilder,
    UncensuredaiProviderSettings,
    "uncensuredai"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Uncensuredai);
