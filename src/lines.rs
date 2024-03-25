use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::consts::{self, LINE_MARGIN};
use crate::window;

const RECTANGLE_MARGIN: f32 = 30.0;
const RECTANGLE_WIDTH: f32 = 10.0;

const LINE_WIDTH: f32 = 20.0;

#[derive(Component)]
pub struct RectangleMarker;

pub fn get_scale(window_size: Vec2) -> f32 {
    let min_size = window_size.x.min(window_size.y);
    let original_min_size = 900.0;

    min_size / original_min_size
}

pub fn draw_rectangle(
    commands: &mut Commands,
    windows: &Query<&Window>,
    query: &Query<Entity, With<RectangleMarker>>,
) {
    query.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });

    let window_size = window::get_window_size(windows);
    let scale = get_scale(window_size);
    let width = window_size.x - RECTANGLE_MARGIN * 2.0 * scale;
    let height = window_size.y - RECTANGLE_MARGIN * 2.0 * scale;

    let shape = shapes::Rectangle {
        extents: Vec2::new(width, height),
        origin: shapes::RectangleOrigin::Center,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Stroke::new(Color::WHITE, RECTANGLE_WIDTH * scale),
        RectangleMarker,
    ));
}

#[derive(Component)]
pub struct LineMarker;

fn spawn_line(
    commands: &mut Commands,
    start: Vec2,
    end: Vec2,
    color: Color,
    line_width: f32,
    scale: f32,
) {
    let shape = shapes::Line(start, end);
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Stroke::new(color, line_width * scale),
        LineMarker,
    ));
}

fn draw_grid_lines(
    commands: &mut Commands,
    is_vertical: bool,
    margin: f32,
    cell_size: f32,
    half_size: f32,
    line_width: f32,
    scale: f32,
) {
    let margin = margin * scale;

    for i in 1..3 {
        let (start, end) = if is_vertical {
            let x = margin + cell_size * i as f32 - half_size;
            (
                Vec2::new(x, margin - half_size),
                Vec2::new(x, half_size - margin),
            )
        } else {
            let y = margin + cell_size * i as f32 - half_size;
            (
                Vec2::new(margin - half_size, y),
                Vec2::new(half_size - margin, y),
            )
        };

        spawn_line(commands, start, end, consts::get_color(), line_width, scale);
    }
}

pub fn draw_lines(
    commands: &mut Commands,
    windows: &Query<&Window>,
    query: &Query<Entity, With<LineMarker>>,
) {
    query.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });

    let (cell_size, half_size) = window::get_sizes(windows);
    let scale = get_scale(window::get_window_size(windows));

    draw_grid_lines(
        commands,
        true,
        LINE_MARGIN,
        cell_size,
        half_size,
        LINE_WIDTH,
        scale,
    );

    draw_grid_lines(
        commands,
        false,
        LINE_MARGIN,
        cell_size,
        half_size,
        LINE_WIDTH,
        scale,
    );
}
