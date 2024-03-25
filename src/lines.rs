use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::consts::{self, LINE_MARGIN};
use crate::window;

const RECTANGLE_MARGIN: f32 = 30.0;
const RECTANGLE_WIDTH: f32 = 10.0;

const LINE_WIDTH: f32 = 20.0;

pub fn draw_rectangle(mut commands: Commands, windows: Query<&Window>) {
    let window_size = window::get_window_size(&windows);
    let width = window_size.x - RECTANGLE_MARGIN * 2.0;
    let height = window_size.y - RECTANGLE_MARGIN * 2.0;

    let shape = shapes::Rectangle {
        extents: Vec2::new(width, height),
        origin: shapes::RectangleOrigin::Center,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Stroke::new(Color::WHITE, RECTANGLE_WIDTH),
    ));
}

fn spawn_line(commands: &mut Commands, start: Vec2, end: Vec2, color: Color, line_width: f32) {
    let shape = shapes::Line(start, end);
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Stroke::new(color, line_width),
    ));
}

fn draw_grid_lines(
    commands: &mut Commands,
    is_vertical: bool,
    margin: f32,
    cell_size: f32,
    half_size: f32,
    line_width: f32,
) {
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

        spawn_line(commands, start, end, consts::get_color(), line_width);
    }
}

pub fn draw_lines(mut commands: Commands, windows: Query<&Window>) {
    let (cell_size, half_size) = window::get_sizes(windows);

    draw_grid_lines(
        &mut commands,
        true,
        LINE_MARGIN,
        cell_size,
        half_size,
        LINE_WIDTH,
    );

    draw_grid_lines(
        &mut commands,
        false,
        LINE_MARGIN,
        cell_size,
        half_size,
        LINE_WIDTH,
    );
}
