//! This module provides the Phase provider, wrapping OpenAI Chat Completions for Phase requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    PhaseProviderSettings,
    PhaseProviderSettingsBuilder,
    "Phase",
    "https://router.phase.xyz/v1",
    "PHASE_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(Phase, PhaseBuilder, PhaseProviderSettings, "phase");

// Generate the language model implementation
crate::openai_compatible_language_model!(Phase);
