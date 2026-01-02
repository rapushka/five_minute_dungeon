use avian2d::math::Scalar;
use crate::physics::{MovementAcceleration, MovementDamping};
use crate::prelude::*;

#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDamping,
    // TODO: jump impulse
    // TODO: max slope angle
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDamping(damping),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9)
    }
}