use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::{area, lines, shapes};

#[derive(Component)]
pub struct ResizeMarker;

pub fn resize_listener(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<WindowResized>,
    windows: Query<&Window>,
    query: Query<Entity, With<ResizeMarker>>,
) {
    for event in events.read() {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        lines::draw_rectangle(&mut commands, &windows);
        lines::draw_lines(&mut commands, &windows);

        shapes::draw_shapes(&mut commands, &windows);
        area::draw_rectangle(&mut commands, &windows, &mut meshes, &mut materials);

        println!("Window resized to: {} {}", event.width, event.height);
    }
}
