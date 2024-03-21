#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod wasm;

use bevy::{asset::AssetMetaCheck, prelude::*, sprite::MaterialMesh2dBundle};

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn run() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(wasm::get_plugins())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(Color::PURPLE),
        ..default()
    });
}
