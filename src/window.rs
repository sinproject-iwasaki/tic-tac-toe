use bevy::prelude::*;

use crate::consts::LINE_MARGIN;

pub fn get_window_size(windows: &Query<&Window>) -> Vec2 {
    let window = windows.single();
    Vec2::new(window.width(), window.height())
}

pub fn get_sizes(windows: Query<&Window>) -> (f32, f32) {
    let window_size = get_window_size(&windows);
    let min_size = window_size.x.min(window_size.y);
    let cell_size = ((min_size - LINE_MARGIN * 2.0) / 3.0).floor();
    let half_size = min_size / 2.0;

    (cell_size, half_size)
}
