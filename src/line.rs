use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const MARGIN_RECTANGLE: f32 = 30.0;
const RECTANGLE_WIDTH: f32 = 10.0;

const MARGIN_LINES: f32 = 65.0;
const LINE_WIDTH: f32 = 20.0;

fn get_window_size(windows: &Query<&Window>) -> Vec2 {
    let window = windows.single();
    Vec2::new(window.width(), window.height())
}

pub fn draw_rectangle(mut commands: Commands, windows: Query<&Window>) {
    let window_size = get_window_size(&windows);
    let width = window_size.x - MARGIN_RECTANGLE * 2.0;
    let height = window_size.y - MARGIN_RECTANGLE * 2.0;

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
    let line = shapes::Line(start, end);
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&line),
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

        spawn_line(
            commands,
            start,
            end,
            Color::rgb_u8(143, 193, 243),
            line_width,
        );
    }
}

pub fn draw_lines(mut commands: Commands, windows: Query<&Window>) {
    let window_size = get_window_size(&windows);
    let min_size = window_size.x.min(window_size.y);
    let cell_size = ((min_size - MARGIN_LINES * 2.0) / 3.0).floor();
    let half_size = min_size / 2.0;

    draw_grid_lines(
        &mut commands,
        true,
        MARGIN_LINES,
        cell_size,
        half_size,
        LINE_WIDTH,
    );
    draw_grid_lines(
        &mut commands,
        false,
        MARGIN_LINES,
        cell_size,
        half_size,
        LINE_WIDTH,
    );
}
