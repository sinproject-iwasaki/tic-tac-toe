use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{consts, lines, window};

// use crate::{consts, window};

const MARK_MARGIN: f32 = 10.0;
const MARK_WIDTH: f32 = 35.0;

#[derive(Component)]
pub struct CircleMarker;

pub fn draw_circle(
    commands: &mut Commands,
    query: &Query<Entity, With<CircleMarker>>,
    cell_size: f32,
    line_width: f32,
    color: Color,
    scale: f32,
) {
    query.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });

    let line_width = line_width * scale;
    let margin_scale = MARK_MARGIN * scale;
    let radius = (cell_size) / 2.0 - line_width - margin_scale;
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
        CircleMarker,
    ));
}

#[derive(Component)]
pub struct CrossMarker;

pub fn draw_cross(
    commands: &mut Commands,
    query: &Query<Entity, With<CrossMarker>>,
    cell_size: f32,
    line_width: f32,
    color: Color,
    scale: f32,
) {
    query.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });

    let mark_margin = MARK_MARGIN * scale;
    let line_width = line_width * scale;
    let half_size = (cell_size) / 2.0 - mark_margin - line_width * 3.0 / 4.0;

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
        CrossMarker,
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
        CrossMarker,
    ));
}

pub fn draw_shapes(
    commands: &mut Commands,
    windows: &Query<&Window>,
    circle_query: &Query<Entity, With<CircleMarker>>,
    cross_query: &Query<Entity, With<CrossMarker>>,
) {
    let (cell_size, _) = window::get_sizes(windows);
    let color = consts::get_color();
    let window_size = window::get_window_size(windows);
    let scale = lines::get_scale(window_size);

    draw_circle(commands, circle_query, cell_size, MARK_WIDTH, color, scale);
    draw_cross(commands, cross_query, cell_size, MARK_WIDTH, color, scale);
}
