//! This module provides the TeapotRouter provider, wrapping OpenAI Chat Completions for TeapotRouter requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    TeapotRouterProviderSettings,
    TeapotRouterProviderSettingsBuilder,
    "TeapotRouter",
    "https://api.teapotrouter.com/v1",
    "TEAPOT_ROUTER_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    TeapotRouter,
    TeapotRouterBuilder,
    TeapotRouterProviderSettings,
    "teapot-router"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(TeapotRouter);
