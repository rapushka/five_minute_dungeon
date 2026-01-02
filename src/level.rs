use crate::prelude::*;

pub fn spawn_level(
    mut commands: Commands,
) {
    commands.spawn((
        new_sprite(vec2(1100.0, 50.0)),
        Transform::from_xyz(0.0, -175.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(1100.0, 50.0),
    ));
}

fn new_sprite(size: Vec2) -> Sprite {
    Sprite {
        color: Color::srgb(0.7, 0.7, 0.8),
        custom_size: Some(size),
        ..default()
    }
}