use avian2d::math::Scalar;
use crate::prelude::*;

pub mod character_controller;
pub mod movement_bundle;

#[derive(Message)]
pub enum MovementAction {
    Move(Scalar),
    Jump,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct MovementAcceleration(Scalar);

#[derive(Component)]
pub struct MovementDamping(Scalar);