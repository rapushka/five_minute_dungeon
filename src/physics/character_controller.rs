use crate::physics::movement_bundle::MovementBundle;
use crate::physics::KinematicGravity;
use crate::prelude::*;
use avian2d::math::Scalar;

#[derive(Bundle)]
pub struct ControllableCharacterBundle {
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    gravity: KinematicGravity,
    movement: MovementBundle,
}

impl ControllableCharacterBundle {
    pub fn new(collider: Collider, gravity: AvianVec) -> Self {
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(AvianVec::ONE * 0.99, 10);

        let ground_caster = ShapeCaster::new(caster_shape, AvianVec::ZERO, 0.0, Dir2::NEG_Y)
            .with_max_distance(10.0);

        Self {
            body: RigidBody::Kinematic,
            collider,
            ground_caster,
            gravity: KinematicGravity(gravity),
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}