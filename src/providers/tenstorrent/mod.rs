//! This module provides the Tenstorrent provider, wrapping OpenAI Chat Completions for Tenstorrent requests.

pub mod capabilities;

// Generate the settings module
crate::openai_compatible_settings!(
    TenstorrentProviderSettings,
    TenstorrentProviderSettingsBuilder,
    "Tenstorrent",
    "https://console.tenstorrent.com/v1",
    "TENSTORRENT_API_KEY"
);

// Generate the provider struct and builder
crate::openai_compatible_provider!(
    Tenstorrent,
    TenstorrentBuilder,
    TenstorrentProviderSettings,
    "tenstorrent"
);

// Generate the language model implementation
crate::openai_compatible_language_model!(Tenstorrent);
