use bevy::prelude::*;

use crate::consts::GAME_MARGIN;

pub fn get_window_size(windows: &Query<&Window>) -> Vec2 {
    let window = windows.single();
    Vec2::new(window.width(), window.height())
}

fn get_min_size(window_size: Vec2) -> f32 {
    window_size.x.min(window_size.y)
}

pub fn get_scale(window_size: Vec2) -> f32 {
    let min_size = window_size.x.min(window_size.y);
    let original_min_size = 900.0;

    min_size / original_min_size
}

pub fn get_board_size(window_size: Vec2) -> f32 {
    let min_size = get_min_size(window_size);
    let scale = get_scale(window_size);
    let margin = GAME_MARGIN * scale;

    min_size - margin * 2.0
}

pub fn get_cell_size(board_size: f32) -> f32 {
    board_size / 3.0
}
