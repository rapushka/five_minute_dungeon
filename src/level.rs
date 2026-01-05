use bevy::asset::RenderAssetUsages;
use bevy::mesh::PrimitiveTopology;
use crate::prelude::*;

pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_platform(&mut commands, vec2(1_100.0, 50.0), vec2(0.0, -175.0));
    spawn_platform(&mut commands, vec2(300.0, 25.0), vec2(175.0, -35.0));
    spawn_platform(&mut commands, vec2(300.0, 25.0), vec2(-175.0, 0.0));
    spawn_platform(&mut commands, vec2(150.0, 80.0), vec2(475.0, -110.0));
    spawn_platform(&mut commands, vec2(150.0, 80.0), vec2(-475.0, -110.0));

    spawn_ramp_1(&mut commands, &mut meshes, &mut materials);
    spawn_ramp_2(&mut commands, &mut meshes, &mut materials);
}

fn spawn_ramp_1(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let mut ramp_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());

    ramp_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-125.0, 80.0, 0.0], [-125.0, 0.0, 0.0], [125.0, 0.0, 0.0]],
    );

    let ramp_collider = Collider::triangle(
        AvianVec::new(-125.0, 80.0),
        AvianVec::NEG_X * 125.0,
        AvianVec::X * 125.0,
    );

    commands.spawn((
        Mesh2d(meshes.add(ramp_mesh)),
        MeshMaterial2d(materials.add(Color::srgb(0.4, 0.4, 0.5))),
        Transform::from_xyz(-275.0, -150.0, 0.0),
        RigidBody::Static,
        ramp_collider,
    ));
}

fn spawn_ramp_2(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
    let mut ramp_mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());

    ramp_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[20.0, -40.0, 0.0], [20.0, 40.0, 0.0], [-20.0, -40.0, 0.0]],
    );

    let ramp_collider = Collider::triangle(
        AvianVec::new(20.0, -40.0),
        AvianVec::new(20.0, 40.0),
        AvianVec::new(-20.0, -40.0),
    );

    commands.spawn((
        Mesh2d(meshes.add(ramp_mesh)),
        MeshMaterial2d(materials.add(Color::srgb(0.4, 0.4, 0.5))),
        Transform::from_xyz(380.0, -110.0, 0.0),
        RigidBody::Static,
        ramp_collider,
    ));
}

fn spawn_platform(commands: &mut Commands, size: Vec2, position: Vec2) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.8),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.0),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
    ));
}