use avian2d::math::Scalar;
use crate::prelude::*;

pub mod character_controller;
pub mod movement_bundle;

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

#[derive(Component)]
pub struct MaxSlopeAngle(pub Scalar);