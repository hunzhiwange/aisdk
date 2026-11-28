//! This module provides the HostingerRouter provider, wrapping OpenAI Chat Completions for HostingerRouter requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    HostingerRouterProviderSettings,
    HostingerRouterProviderSettingsBuilder,
    "HostingerRouter",
    "https://router.hostinger.com/v1",
    "HOSTINGER_ROUTER_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    HostingerRouter,
    HostingerRouterBuilder,
    HostingerRouterProviderSettings,
    "hostinger-router"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(HostingerRouter);
