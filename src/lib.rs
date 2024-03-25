#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod consts;
mod lines;
mod shapes;
mod wasm;
mod window;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_prototype_lyon::prelude::*;

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn run() {
    let plugins = wasm::get_plugins();

    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((plugins, ShapePlugin))
        .add_systems(
            Startup,
            (
                setup,
                lines::draw_rectangle,
                lines::draw_lines,
                shapes::draw_shapes,
            ),
        )
        .run();
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup(mut commands: Commands) {
    spawn_camera(&mut commands);
}
