//! Trazo: A drawing city builder

// Document all public API
#![warn(missing_docs)]
// Lints
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

#[macro_use]
extern crate macro_rules_attribute;

use bevy::prelude::*;

pub mod camera;
pub mod data;
pub mod helpers;
#[cfg(feature = "menu")]
pub mod menu;
pub mod prelude;
pub mod states;

/// The base plugin for the game. It recursively adds all of the plugins
/// declared in submodules as well as the default plugin collection.
/// A plugin in bevy allows you to extend the [App] at the start of the game,
/// adding systems, resources and other plugins.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let window_plugin = WindowPlugin {
            primary_window: Some(Window {
                title: "Trazo".into(),
                name: Some("trazo".into()),
                canvas: Some("#bevy".into()),
                prevent_default_event_handling: false,
                resolution: (1500., 1000.).into(),
                ..default()
            }),
            ..default()
        };

        let log_plugin = bevy::log::LogPlugin {
            filter: "warn,wgpu=error,trazo=debug".into(),
            ..default()
        };

        app.add_plugins((
            DefaultPlugins.set(window_plugin).set(log_plugin),
            camera::plugin,
            data::plugin,
            helpers::plugin,
            states::plugin,
        ));

        #[cfg(feature = "menu")]
        app.add_plugins(menu::plugin);

        #[cfg(feature = "embedded")]
        app.add_plugins(bevy_embedded_assets::EmbeddedAssetPlugin::default());
    }
}
