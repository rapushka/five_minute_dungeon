use avian2d::math::Scalar;
use crate::level::spawn_level;
use crate::physics::character_controller::CharacterControllerBundle;
use crate::physics::{Grounded, MovementAcceleration, MovementAction};
use crate::prelude::*;

mod prelude;
mod physics;
mod level;

const MAX_SPEED: f32 = 250.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default().with_length_unit(20.0))

        .add_message::<MovementAction>()

        .add_systems(Startup, (
            spawn_camera,
            spawn_level,
            spawn_player,
        ))
        .add_systems(Update, (
            keyboard_input,
            // TODO: check grounded
            move_player,
            // TODO: damping
        ).chain())

        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct InputReceiver;

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Capsule2d::new(12.5, 20.0));
    let material = materials.add(Color::srgb(0.2, 0.7, 0.9));

    let acceleration = 1250.0;
    let damping = 5.0;

    commands.spawn((
        Player,
        InputReceiver,
        (   // rendering
            Mesh2d(shape),
            MeshMaterial2d(material),
            Transform::from_xyz(0.0, -100.0, 0.0),
        ),
        (   // physics
            CharacterControllerBundle::new(Collider::capsule(12.5, 20.0))
                .with_movement(acceleration, damping),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            ColliderDensity(2.0),
            GravityScale(1.5),
        ),
    ));
}

fn keyboard_input(
    mut movement_writer: MessageWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let direction = horizontal as Scalar;

    if direction != 0.0 {
        movement_writer.write(MovementAction::Move(direction));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        // TODO: jump
    }
}

fn move_player(
    mut receivers: Query<(
        &MovementAcceleration,
        // TODO: jump impulse
        &mut LinearVelocity,
        Has<Grounded>,
    ), With<InputReceiver>>,
    mut movement_reader: MessageReader<MovementAction>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for event in movement_reader.read() {
        for (acceleration, mut velocity, is_grounded) in &mut receivers {
            match event {
                MovementAction::Move(direction) => {
                    velocity.x += *direction * acceleration.0 * delta_time;
                    velocity.x = velocity.x.clamp(-MAX_SPEED, MAX_SPEED);
                }
                MovementAction::Jump => {
                    if is_grounded {
                        // LinearVelocity.y = jump_impulse.0;
                    }
                }
            }
        }
    }
}