use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::{
    lines::{self, LineMarker, RectangleMarker},
    shapes::{self, CircleMarker, CrossMarker},
};

pub fn resize_listener(
    mut commands: Commands,
    mut events: EventReader<WindowResized>,
    windows: Query<&Window>,
    rectangle_query: Query<Entity, With<RectangleMarker>>,
    line_query: Query<Entity, With<LineMarker>>,
    circle_query: Query<Entity, With<CircleMarker>>,
    cross_query: Query<Entity, With<CrossMarker>>,
) {
    for event in events.read() {
        lines::draw_rectangle(&mut commands, &windows, &rectangle_query);
        lines::draw_lines(&mut commands, &windows, &line_query);

        shapes::draw_shapes(&mut commands, &windows, &circle_query, &cross_query);

        println!("Window resized to: {} {}", event.width, event.height);
    }
}
