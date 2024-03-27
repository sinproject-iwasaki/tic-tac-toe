use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::window;
use crate::{lines::LINE_WIDTH, resize::ResizeMarker, shapes::MARK_MARGIN};

#[derive(PartialEq, Eq)]
enum ButtonsStatus {
    Hovered,
    Pressed,
    Released,
}

#[derive(Component)]
pub struct ButtonMarker(ButtonsStatus);

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
        ButtonMarker(ButtonsStatus::Released),
    ));
}

pub fn button_events(
    mut cursor_evr: EventReader<CursorMoved>,
    mut tap_marker_query: Query<
        (
            Entity,
            &Transform,
            &Handle<ColorMaterial>,
            &mut ButtonMarker,
        ),
        With<ButtonMarker>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    let window_size = window::get_window_size(&windows);
    let board_size = window::get_board_size(window_size);
    let cell_size = window::get_cell_size(board_size);
    let scale = window::get_scale(window_size);
    let margin = MARK_MARGIN * scale;
    let line_width = LINE_WIDTH * scale;
    let mark_size = cell_size - line_width - margin * 2.0;

    let is_pressed = mouse_button_input.pressed(MouseButton::Left);

    for ev in cursor_evr.read() {
        for (_, transform, material_handle, mut tap_marker) in tap_marker_query.iter_mut() {
            let cursor_pos = ev.position;
            let cursor_pos = Vec2::new(
                cursor_pos.x - window_size.x / 2.0,
                -cursor_pos.y + window_size.y / 2.0,
            );

            let marker_pos = transform.translation;

            let is_hovered: bool = (cursor_pos.x - marker_pos.x).abs() < mark_size / 2.0
                && (cursor_pos.y - marker_pos.y).abs() < mark_size / 2.0;

            if tap_marker.0 != ButtonsStatus::Pressed && is_hovered && is_pressed {
                tap_marker.0 = ButtonsStatus::Pressed;

                if let Some(material) = materials.get_mut(material_handle) {
                    *material = ColorMaterial::from(Color::rgb_u8(128, 255, 128));
                }
            } else if tap_marker.0 != ButtonsStatus::Hovered && is_hovered && !is_pressed {
                tap_marker.0 = ButtonsStatus::Hovered;

                if let Some(material) = materials.get_mut(material_handle) {
                    *material = ColorMaterial::from(Color::rgb_u8(0, 255, 0));
                }
            } else if tap_marker.0 != ButtonsStatus::Released && !is_hovered {
                tap_marker.0 = ButtonsStatus::Released;

                if let Some(material) = materials.get_mut(material_handle) {
                    *material = ColorMaterial::from(Color::rgb_u8(255, 0, 128));
                    // 色を緑に変更
                }
            }
        }
    }
}
