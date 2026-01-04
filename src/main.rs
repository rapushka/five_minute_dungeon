use avian2d::math::{Scalar, Vector};
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;

use crate::level::spawn_level;
use crate::physics::character_controller::*;
use crate::physics::*;
use crate::prelude::*;

mod prelude;
mod physics;
mod level;

const MAX_SPEED: Scalar = 1_500.0;
const MOVEMENT_ACCELERATION: Scalar = 3_000.0;
const MOVEMENT_DAMPING: Scalar = 10.0;
const JUMP_IMPULSE: Scalar = 400.0;
const MAX_SLOPE_DEGREE: Scalar = 30.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build()
            // fixes "couldn't get swap chain texture"
            .disable::<PipelinedRenderingPlugin>()
        )
        .add_plugins(PhysicsPlugins::default().with_length_unit(20.0))

        .add_message::<MovementAction>()

        .add_systems(Startup, (
            spawn_camera,
            spawn_level,
            spawn_player,
        ))
        .add_systems(Update, (
            keyboard_input,
            check_grounded,
            move_player,
            damp_linear_movement,
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
                .with_movement(
                    MOVEMENT_ACCELERATION,
                    MOVEMENT_DAMPING,
                    JUMP_IMPULSE,
                    MAX_SLOPE_DEGREE.to_radians(),
                ),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            ColliderDensity(2.0),
            GravityScale(75.0),
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
        movement_writer.write(MovementAction::Jump);
    }
}

fn check_grounded(
    mut commands: Commands,
    mut characters: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<InputReceiver>,
    >,
) {
    for (entity, hits, rotation, max_angle) in &mut characters {
        let is_grounded = hits.iter().any(|hit| {
            match max_angle {
                Some(angle) => is_flat_enough(rotation, hit, angle.0),
                None => true,
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

fn is_flat_enough(rotation: &Rotation, hit: &ShapeHitData, angle: Scalar) -> bool {
    (rotation * -hit.normal2).angle_to(Vector::Y).abs() <= angle
}

fn move_player(
    mut receivers: Query<(
        &MovementAcceleration,
        &mut LinearVelocity,
        &JumpImpulse,
        Has<Grounded>,
    ), With<InputReceiver>>,
    mut movement_reader: MessageReader<MovementAction>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for event in movement_reader.read() {
        for (acceleration, mut velocity, jump_impulse, is_grounded) in &mut receivers {
            match event {
                MovementAction::Move(direction) => {
                    velocity.x += *direction * acceleration.0 * delta_time;
                    velocity.x = velocity.x.clamp(-MAX_SPEED, MAX_SPEED);
                }
                MovementAction::Jump => {
                    if is_grounded {
                        velocity.y = jump_impulse.0;
                    }
                }
            }
        }
    }
}

fn damp_linear_movement(
    mut moving_entities: Query<(&MovementDamping, &mut LinearVelocity)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (damp, mut velocity) in &mut moving_entities {
        velocity.x *= 1.0 / (1.0 + damp.0 * delta_time);
    }
}