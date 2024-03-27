use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{consts, resize::ResizeMarker, window};

// use crate::{consts, window};

pub const MARK_MARGIN: f32 = 10.0;
const MARK_WIDTH: f32 = 35.0;

pub fn draw_circle(
    commands: &mut Commands,
    cell_size: f32,
    line_width: f32,
    margin: f32,
    color: Color,
) {
    let radius = (cell_size) / 2.0 - line_width - margin;
    let offset_x = 1.0 * cell_size;
    let offset_y = -1.0 * cell_size; // 左に1マス分のオフセット

    let shape = shapes::Circle {
        radius,
        ..default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            spatial: SpatialBundle {
                transform: Transform::from_xyz(offset_x, offset_y, 0.),
                ..default()
            },
            ..default()
        },
        Stroke::new(color, line_width),
        ResizeMarker,
    ));
}

pub fn draw_cross(
    commands: &mut Commands,
    cell_size: f32,
    line_width: f32,
    margin: f32,
    color: Color,
) {
    let half_size = (cell_size) / 2.0 - margin - line_width * 3.0 / 4.0;

    // バツの形を正しく描画するために、開始点と終了点を修正します。
    let line_1_start = Vec2::new(half_size, half_size);
    let line_1_end = Vec2::new(-half_size, -half_size);

    let line_2_start = Vec2::new(half_size, -half_size);
    let line_2_end = Vec2::new(-half_size, half_size);

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Line(line_1_start, line_1_end)),
            // spatial: SpatialBundle {
            //     transform: Transform::from_xyz(-offset_x, -offset_x, 0.),
            //     ..default()
            // },
            ..default()
        },
        Stroke::new(color, line_width),
        ResizeMarker,
    ));
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Line(line_2_start, line_2_end)),
            // spatial: SpatialBundle {
            //     transform: Transform::from_xyz(-offset_x, -offset_x, 0.),
            //     ..default()
            // },
            ..default()
        },
        Stroke::new(color, line_width),
        ResizeMarker,
    ));
}

pub fn draw_shapes(commands: &mut Commands, windows: &Query<&Window>) {
    let window_size = window::get_window_size(windows);
    let board_size = window::get_board_size(window_size);
    let cell_size = window::get_cell_size(board_size);
    let window_size = window::get_window_size(windows);
    let scale = window::get_scale(window_size);
    let line_width = MARK_WIDTH * scale;
    let margin = MARK_MARGIN * scale;
    let color = consts::get_color();

    draw_circle(commands, cell_size, line_width, margin, color);
    draw_cross(commands, cell_size, line_width, margin, color);
}
