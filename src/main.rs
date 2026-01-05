// bevy's query can be big and that's okay
#![allow(clippy::type_complexity)]

use avian2d::math::Vector;
use bevy::render::pipelined_rendering::PipelinedRenderingPlugin;

use crate::physics::MovementAction;
use crate::prelude::*;

mod prelude;
mod physics;
mod level;
mod room;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build()
            // fixes "couldn't get swap chain texture" on linux
            .disable::<PipelinedRenderingPlugin>()
        )
        .add_plugins(PhysicsPlugins::default().with_length_unit(200.0))

        .insert_resource(Gravity(Vector::NEG_Y * 1_000.0))

        .add_message::<MovementAction>()

        .add_systems(Startup, (
            spawn_camera,
            level::spawn_level,
            player::spawn_player,
        ))
        .add_systems(Update, (
            player::keyboard_input,
            physics::check_grounded,
            physics::apply_kinematic_gravity,
            player::move_player,
            physics::damp_linear_movement,
        ).chain())
        .add_systems(PhysicsSchedule, (
            physics::handle_kinematic_collisions.in_set(NarrowPhaseSystems::Last),
        ))

        .run();
}

pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}