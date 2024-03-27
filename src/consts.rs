use bevy::prelude::*;

pub const DEFAULT_WIDTH: f32 = 1200.;
pub const DEFAULT_HEIGHT: f32 = 900.;

const GAME_COLOR: (u8, u8, u8) = (143, 193, 243);

pub const GAME_MARGIN: f32 = 65.0;

pub fn get_color() -> Color {
    Color::rgb_u8(GAME_COLOR.0, GAME_COLOR.1, GAME_COLOR.2)
}
