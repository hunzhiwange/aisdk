//! This module provides the TencentTokenPlanIntl provider, wrapping OpenAI Chat Completions for TencentTokenPlanIntl requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    TencentTokenPlanIntlProviderSettings,
    TencentTokenPlanIntlProviderSettingsBuilder,
    "TencentTokenPlanIntl",
    "https://tokenhub-intl.tencentcloudmaas.com/plan/v3",
    "TENCENT_TOKEN_PLAN_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    TencentTokenPlanIntl,
    TencentTokenPlanIntlBuilder,
    TencentTokenPlanIntlProviderSettings,
    "tencent-token-plan-intl"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(TencentTokenPlanIntl);
