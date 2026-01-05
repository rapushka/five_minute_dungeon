use crate::physics::{Grounded, JumpImpulse, MovementAcceleration, MovementAction, PhysicsObject};
use crate::physics::character_controller::ControllableCharacterBundle;
use crate::prelude::*;

const MAX_SPEED: Scalar = 1_500.0;
const MOVEMENT_ACCELERATION: Scalar = 3_000.0;
const MOVEMENT_DAMPING: Scalar = 10.0;
const JUMP_IMPULSE: Scalar = 600.0;
const MAX_SLOPE_DEGREE: Scalar = 30.0;
const JUMP_CUT: Scalar = 0.4;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct InputReceiver;

#[derive(Component)]
pub struct JumpCutMultiplier(pub Scalar);

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Capsule2d::new(10.0, 15.0));
    let material = materials.add(Color::srgb(0.2, 0.7, 0.9));

    commands.spawn((
        Player,
        (   // rendering
            Mesh2d(shape),
            MeshMaterial2d(material),
            Transform::from_xyz(0.0, -100.0, 0.0),
        ),
        (   // character controller
            InputReceiver,
            PhysicsObject,
            ControllableCharacterBundle::new(Collider::capsule(12.5, 20.0), AvianVec::NEG_Y * 1_500.0) // TODO: isn't this number too big?
                .with_movement(
                    MOVEMENT_ACCELERATION,
                    MOVEMENT_DAMPING,
                    JUMP_IMPULSE,
                    MAX_SLOPE_DEGREE.to_radians(),
                ),
            JumpCutMultiplier(JUMP_CUT),
        )
    ));
}

pub fn keyboard_input(
    mut movement_writer: MessageWriter<MovementAction>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let left = keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let direction = horizontal as Scalar;

    if direction != 0.0 {
        movement_writer.write(MovementAction::Move(direction));
    }

    if keyboard.just_pressed(KeyCode::Space) {
        movement_writer.write(MovementAction::Jump);
    }

    if keyboard.just_released(KeyCode::Space) {
        movement_writer.write(MovementAction::JumpRelease);
    }
}

pub fn move_player(
    mut characters: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        &JumpCutMultiplier,
        Has<Grounded>,
    ), With<InputReceiver>>,
    mut movement_reader: MessageReader<MovementAction>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for event in movement_reader.read() {
        for (acceleration, jump_impulse, mut velocity, jump_cut, is_grounded) in &mut characters {
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
                MovementAction::JumpRelease => {
                    if velocity.y > 0.0 {
                        velocity.y *= jump_cut.0;
                    }
                }
            }
        }
    }
}