use avian2d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())

        .add_systems(Startup, setup)

        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let player_shape = meshes.add(Capsule2d::new(25.0, 50.0));
    let player_material = materials.add(Color::srgba(1.0, 0.25, 0.25, 1.0));

    commands.spawn((
        Player,
        ( // rendering
          Mesh2d(player_shape),
          MeshMaterial2d(player_material)
        ),
        ( // physics
          RigidBody::Dynamic,
          Collider::capsule(25.0, 50.0),
          GravityScale(1.0),
          LinearVelocity::ZERO,
        ),
    ))
    ;
}
