//! This module provides the SuperbotGg provider, wrapping OpenAI Chat Completions for SuperbotGg requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    SuperbotGgProviderSettings,
    SuperbotGgProviderSettingsBuilder,
    "SuperbotGg",
    "https://api.superbot.gg/v1",
    "SUPERBOT_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    SuperbotGg,
    SuperbotGgBuilder,
    SuperbotGgProviderSettings,
    "superbot-gg"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(SuperbotGg);
