use crate::config;
use bevy::{app::PluginGroupBuilder, prelude::*, window::*};

#[cfg(not(target_family = "wasm"))]
pub(crate) fn plugins() -> PluginGroupBuilder {
    use bevy_embedded_assets::EmbeddedAssetPlugin;

    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Fullscreen,
                title: config::Title::TITLE.into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        })
        .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
}

#[cfg(target_family = "wasm")]
pub(crate) fn plugins() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: config::Title::TITLE.into(),
            resolution: (1536., 864.).into(),
            ..Default::default()
        }),
        ..Default::default()
    })
}
