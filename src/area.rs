use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub fn draw_rectangle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rectangle = Rectangle::new(200., 200.);
    let shape = Mesh2dHandle(meshes.add(rectangle));

    commands.spawn(MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(Color::rgb_u8(255, 0, 128)),
        transform: Transform::from_xyz(-255., 250., 0.),
        ..default()
    });
}
