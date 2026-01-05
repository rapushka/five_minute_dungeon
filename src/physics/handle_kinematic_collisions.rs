use avian2d::prelude::{ColliderOf, Collisions, LinearVelocity, RigidBody, Sensor};
use avian2d::physics_transform::Position;
use avian2d::math::{Scalar, Vector as AvianVec};
use crate::physics::MaxSlopeAngle;
use crate::physics::PhysicsObject;
use crate::prelude::{Query, Res, Time, With, Without};

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This system handles collision response for kinematic character controllers
/// by pushing them along their contact normals by the current penetration depth,
/// and applying velocity corrections in order to snap to slopes, slide along walls,
/// and predict collisions using speculative contacts.
pub fn handle_kinematic_collisions(
    collisions: Collisions,
    bodies: Query<&RigidBody>,
    colliders: Query<&ColliderOf, Without<Sensor>>,
    mut objects: Query<
        (&mut Position, &mut LinearVelocity, Option<&MaxSlopeAngle>),
        (With<RigidBody>, With<PhysicsObject>),
    >,
    time: Res<Time>,
) {
    for contacts in collisions.iter() {
        let Ok([&ColliderOf { body: rb1 }, &ColliderOf { body: rb2 }])
            = colliders.get_many([contacts.collider1, contacts.collider2])
        else {
            continue;
        };

        let is_first: bool;

        let object_rb: RigidBody;
        let is_other_dynamic: bool;

        let (mut position, mut linear_velocity, max_slope_angle)
            = if let Ok(object) = objects.get_mut(rb1) {
            is_first = true;
            object_rb = *bodies.get(rb1).unwrap();
            is_other_dynamic = bodies.get(rb2).is_ok_and(|rb| rb.is_dynamic());
            object
        } else if let Ok(object) = objects.get_mut(rb2) {
            is_first = false;
            object_rb = *bodies.get(rb2).unwrap();
            is_other_dynamic = bodies.get(rb1).is_ok_and(|rb| rb.is_dynamic());
            object
        } else {
            continue;
        };

        if !object_rb.is_kinematic() {
            continue;
        }

        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first { -manifold.normal } else { manifold.normal };

            let mut deepest_penetration: Scalar = Scalar::MIN;

            for contact in manifold.points.iter() {
                if contact.penetration > 0.0 {
                    position.0 += normal * contact.penetration;
                }
                deepest_penetration = deepest_penetration.max(contact.penetration);
            }

            // For now, this system only handles velocity corrections for collisions against static geometry.
            if is_other_dynamic {
                continue;
            }

            let slope_angle = normal.angle_to(AvianVec::Y);
            let is_climbable = max_slope_angle.is_some_and(|angle| slope_angle.abs() <= angle.0);

            if deepest_penetration > 0.0 {
                // If the slope is climbable, snap the velocity so that the character
                // up and down the surface smoothly.
                if is_climbable {
                    // Points either left or right depending on which side the normal is leaning on.
                    let normal_direction_x
                        = normal.reject_from_normalized(AvianVec::Y).normalize_or_zero();

                    let linear_velocity_x = linear_velocity.dot(normal_direction_x);

                    // Snap the Y speed based on the speed at which the character is moving
                    // up or down the slope, and how steep the slope is.
                    //
                    // A 2D visualization of the slope, the contact normal, and the velocity components:
                    //
                    //             ╱
                    //     normal ╱
                    // *         ╱
                    // │   *    ╱   velocity_x
                    // │       * - - - - - -
                    // │           *       | velocity_y
                    // │               *   |
                    // *───────────────────*

                    let max_y_speed = -linear_velocity_x * slope_angle.tan();
                    linear_velocity.y = linear_velocity.y.max(max_y_speed);
                } else {
                    // The character is intersecting an unclimbable object, like a wall.
                    // We want the character to slide along the surface, similarly to
                    // a collide-and-slide algorithm.

                    // Don't apply an impulse if the character is moving away from the surface.
                    if linear_velocity.dot(normal) > 0.0 {
                        continue;
                    }

                    // Slide along the surface, rejecting the velocity along the contact normal.
                    let impulse = linear_velocity.reject_from_normalized(normal);
                    linear_velocity.0 = impulse;
                }
            } else {
                // The character is not yet intersecting the other object,
                // but the narrow phase detected a speculative collision.
                //
                // We need to push back the part of the velocity
                // that would cause penetration within the next frame.

                let normal_speed = linear_velocity.dot(normal);

                // Don't apply an impulse if the character is moving away from the surface.
                if normal_speed > 0.0 {
                    continue;
                }

                // Compute the impulse to apply.
                let impulse_magnitude = normal_speed - (deepest_penetration / time.delta_secs());
                let mut impulse = impulse_magnitude * normal;

                // Apply the impulse differently depending on the slope angle.
                if is_climbable {
                    // Avoid sliding down slopes.
                    linear_velocity.y -= impulse.y.min(0.0);
                } else {
                    // Avoid climbing up walls.
                    impulse.y = impulse.y.max(0.0);
                    linear_velocity.0 -= impulse;
                }
            }
        }
    }
}