//! This module provides the HuaweiMaas provider, wrapping OpenAI Chat Completions for HuaweiMaas requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    HuaweiMaasProviderSettings,
    HuaweiMaasProviderSettingsBuilder,
    "HuaweiMaas",
    "https://api.modelarts-maas.com/openai/v1",
    "MAAS_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    HuaweiMaas,
    HuaweiMaasBuilder,
    HuaweiMaasProviderSettings,
    "huawei-maas"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(HuaweiMaas);
