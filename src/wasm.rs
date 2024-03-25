use bevy::app::PluginGroupBuilder;
use bevy::{prelude::*, window::WindowResolution};

use crate::consts::{DEFAULT_HEIGHT, DEFAULT_WIDTH};

pub fn get_plugins() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)
                .with_scale_factor_override(1.0),
            ..default()
        }),
        ..default()
    })
}
