use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::window;
use crate::{lines::LINE_WIDTH, resize::ResizeMarker, shapes::MARK_MARGIN};

pub fn draw_rectangle(
    commands: &mut Commands,
    windows: &Query<&Window>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let window_size = window::get_window_size(windows);
    let board_size = window::get_board_size(window_size);
    let cell_size = window::get_cell_size(board_size);
    let scale = window::get_scale(window_size);
    let margin = MARK_MARGIN * scale;
    let line_width = LINE_WIDTH * scale;
    let mark_size = cell_size - line_width - margin * 2.0;
    let offset_x = cell_size * -1.0;
    let offset_y = cell_size * 1.0;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(mark_size, mark_size))),
            material: materials.add(Color::rgb_u8(255, 0, 128)),
            transform: Transform::from_xyz(offset_x, offset_y, 0.),
            ..default()
        },
        ResizeMarker,
    ));
}
