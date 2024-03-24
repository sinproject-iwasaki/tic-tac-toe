use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const RECTANGLE_MARGIN: f32 = 30.0;
const RECTANGLE_WIDTH: f32 = 10.0;

const LINE_MARGIN: f32 = 65.0;
const LINE_WIDTH: f32 = 20.0;

const MARK_MARGIN: f32 = 10.0;
const MARK_WIDTH: f32 = 35.0;

const GAME_COLOR: (u8, u8, u8) = (143, 193, 243);

fn get_window_size(windows: &Query<&Window>) -> Vec2 {
    let window = windows.single();
    Vec2::new(window.width(), window.height())
}

pub fn draw_rectangle(mut commands: Commands, windows: Query<&Window>) {
    let window_size = get_window_size(&windows);
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

        spawn_line(
            commands,
            start,
            end,
            Color::rgb_u8(GAME_COLOR.0, GAME_COLOR.1, GAME_COLOR.2),
            line_width,
        );
    }
}

fn get_sizes(windows: Query<&Window>) -> (f32, f32) {
    let window_size = get_window_size(&windows);
    let min_size = window_size.x.min(window_size.y);
    let cell_size = ((min_size - LINE_MARGIN * 2.0) / 3.0).floor();
    let half_size = min_size / 2.0;

    (cell_size, half_size)
}

pub fn draw_lines(mut commands: Commands, windows: Query<&Window>) {
    let (cell_size, half_size) = get_sizes(windows);

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

    let color = Color::rgb_u8(GAME_COLOR.0, GAME_COLOR.1, GAME_COLOR.2);

    draw_circle(&mut commands, cell_size, MARK_WIDTH, color);
    draw_cross(&mut commands, cell_size, MARK_WIDTH, color);
}

fn draw_circle(commands: &mut Commands, cell_size: f32, line_width: f32, color: Color) {
    let radius = (cell_size) / 2.0 - line_width - MARK_MARGIN;
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
    ));
}

fn draw_cross(commands: &mut Commands, cell_size: f32, line_width: f32, color: Color) {
    let half_size = (cell_size) / 2.0 - MARK_MARGIN - line_width * 3.0 / 4.0;

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
    ));
}
