use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn draw_rectangle(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();
    let margin = 30.0;

    let width = window.width() - margin * 2.0;
    let height = window.height() - margin * 2.0; // ウインドウの端からの隙間

    let shape = shapes::Rectangle {
        extents: Vec2::new(width, height),       // 幅400、高さ200の長方形
        origin: shapes::RectangleOrigin::Center, // 中心点を原点とする
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Stroke::new(Color::WHITE, 10.0),
    ));
}

pub fn draw_lines(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();
    let margin = 65.0;
    let line_width = 20.0;
    let min_size = window.width().min(window.height());
    let cell_size = ((min_size - margin * 2.0) / 3.0).floor(); // 横幅と高さの短い方を基準にセルのサイズを計算
    let half_size = min_size / 2.0;

    // 縦線を引く
    for i in 1..3 {
        let x = margin + cell_size * i as f32 - half_size;
        let start = Vec2::new(x, margin - half_size);
        let end = Vec2::new(x, half_size - margin);

        let line = shapes::Line(start, end);
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&line),
                ..default()
            },
            Stroke::new(Color::rgb_u8(143, 193, 243), line_width),
        ));
    }

    // 横線を引く
    for i in 1..3 {
        let y = margin + cell_size * i as f32 - half_size;
        let start = Vec2::new(margin - half_size, y);
        let end = Vec2::new(half_size - margin, y);

        let line = shapes::Line(start, end);
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&line),
                ..default()
            },
            Stroke::new(Color::rgb_u8(143, 193, 243), line_width),
        ));
    }
}
