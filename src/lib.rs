#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod line;
mod wasm;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_prototype_lyon::prelude::*;

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn run() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((wasm::get_plugins(), ShapePlugin))
        .add_systems(Startup, (setup, line::draw_rectangle, line::draw_lines))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
