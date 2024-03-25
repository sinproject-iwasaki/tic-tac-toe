use bevy::prelude::*;

const GAME_COLOR: (u8, u8, u8) = (143, 193, 243);

pub const LINE_MARGIN: f32 = 65.0;

pub fn get_color() -> Color {
    Color::rgb_u8(GAME_COLOR.0, GAME_COLOR.1, GAME_COLOR.2)
}
