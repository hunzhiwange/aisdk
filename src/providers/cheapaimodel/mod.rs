//! This module provides the Cheapaimodel provider, wrapping OpenAI Chat Completions for Cheapaimodel requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    CheapaimodelProviderSettings,
    CheapaimodelProviderSettingsBuilder,
    "Cheapaimodel",
    "https://api.cheapaimodel.ai/v1",
    "CHEAPAIMODEL_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Cheapaimodel,
    CheapaimodelBuilder,
    CheapaimodelProviderSettings,
    "cheapaimodel"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Cheapaimodel);
