use bevy::prelude::*;
use bevy_integrator::{GameState, PhysicsSchedule, PhysicsSet};

use crate::{
    build::CarState,
    control::user_control_system,
    physics::{
        brake_wheel_system, driven_wheel_lookup_system, steering_curvature_system, steering_system,
        suspension_system,
    },
    tire::point_tire_system,
};

use cameras::{
    camera_az_el::{self, camera_builder},
    control::camera_parent_system,
};

// STATE
// Enum for the physics system state during setup
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PhysicsSystemState {
    #[default]
    Starting,
    Finished,
}

pub fn simulation_setup(app: &mut App) {
    app.add_systems(
        PhysicsSchedule,
        (steering_system, steering_curvature_system).in_set(PhysicsSet::Pre),
    )
    .add_systems(
        PhysicsSchedule,
        (
            suspension_system,
            point_tire_system,
            driven_wheel_lookup_system,
            brake_wheel_system,
        )
            .in_set(PhysicsSet::Evaluate),
    )
    .add_systems(
        Update,
        (user_control_system,).run_if(in_state(CarState::Finished)),
    );
}

pub fn camera_setup(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::InGame),
        camera_builder(
            Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            },
            -90.0_f32.to_radians(),
            10.0_f32.to_radians(),
            20.,
            camera_az_el::UpDirection::Z,
        ),
    )
    .add_systems(
        Update,
        (camera_az_el::az_el_camera, camera_parent_system).run_if(in_state(CarState::Finished)),
    ); // setup the camera
}
