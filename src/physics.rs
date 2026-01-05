use crate::prelude::*;
use avian2d::math::Scalar;

pub use handle_kinematic_collisions::*;
use crate::player::InputReceiver;

pub mod character_controller;
pub mod movement_bundle;
pub mod handle_kinematic_collisions;

#[derive(Message)]
pub enum MovementAction {
    Move(Scalar),
    Jump,
    JumpRelease,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct MovementAcceleration(pub Scalar);

#[derive(Component)]
pub struct MovementDamping(pub Scalar);

#[derive(Component)]
pub struct JumpImpulse(pub Scalar);

// Gravity that is manually applied to Kinematic RigidBodies
#[derive(Component)]
pub struct KinematicGravity(pub AvianVec);

#[derive(Component)]
pub struct MaxSlopeAngle(pub Scalar);

#[derive(Component)]
pub struct PhysicsObject;

pub fn apply_kinematic_gravity(
    mut objects: Query<(&KinematicGravity, &mut LinearVelocity), With<PhysicsObject>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (gravity, mut velocity) in &mut objects {
        velocity.0 += gravity.0 * delta_time;
    }
}

pub fn check_grounded(
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
    (rotation * -hit.normal2).angle_to(AvianVec::Y).abs() <= angle
}

pub fn damp_linear_movement(
    mut moving_entities: Query<(&MovementDamping, &mut LinearVelocity)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (damp, mut velocity) in &mut moving_entities {
        velocity.x *= 1.0 / (1.0 + damp.0 * delta_time);
    }
}