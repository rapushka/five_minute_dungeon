use avian2d::math::{Scalar, Vector};
use crate::physics::movement_bundle::MovementBundle;
use crate::prelude::*;

#[derive(Bundle)]
pub struct CharacterControllerBundle {
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        let ground_caster = ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
            .with_max_distance(10.0);

        Self {
            body: RigidBody::Dynamic,
            collider,
            ground_caster,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping);
        self
    }
}