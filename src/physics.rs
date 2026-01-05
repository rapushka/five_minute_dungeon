use crate::prelude::*;
use avian2d::math::Scalar;

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
pub struct MaxSlopeAngle(pub Scalar); // TODO: need?

fn apply_movement_damping(
    time: Res<Time>,
    mut query: Query<(&MovementDamping, &mut LinearVelocity)>,
) {
    let delta_time = time.delta_secs();

    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= 1.0 / (1.0 + damping_factor.0 * delta_time);
    }
}