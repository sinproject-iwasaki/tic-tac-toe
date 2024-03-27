use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::consts;
use crate::resize::ResizeMarker;
use crate::window;

const RECTANGLE_MARGIN: f32 = 30.0;
const RECTANGLE_WIDTH: f32 = 10.0;

pub const LINE_WIDTH: f32 = 20.0;

pub fn draw_rectangle(commands: &mut Commands, windows: &Query<&Window>) {
    let window_size = window::get_window_size(windows);
    let scale = window::get_scale(window_size);
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
        ResizeMarker,
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
        ResizeMarker,
    ));
}

fn draw_grid_lines(commands: &mut Commands, is_vertical: bool, board_size: f32, line_width: f32) {
    let half_size = board_size / 2.0;
    let cell_size = window::get_cell_size(board_size);

    for i in 1..3 {
        let i = i as f32;

        let (start, end) = if is_vertical {
            let x = -half_size + cell_size * i;
            (Vec2::new(x, -half_size), Vec2::new(x, half_size))
        } else {
            let y = -half_size + cell_size * i;
            (Vec2::new(-half_size, y), Vec2::new(half_size, y))
        };

        spawn_line(commands, start, end, consts::get_color(), line_width);
    }
}

pub fn draw_lines(commands: &mut Commands, windows: &Query<&Window>) {
    let window_size = window::get_window_size(windows);
    let board_size: f32 = window::get_board_size(window_size);
    let scale = window::get_scale(window_size);
    let line_width = LINE_WIDTH * scale;

    draw_grid_lines(commands, true, board_size, line_width);
    draw_grid_lines(commands, false, board_size, line_width);
}
