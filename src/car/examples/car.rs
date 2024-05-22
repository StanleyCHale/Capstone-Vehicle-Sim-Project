use bevy::prelude::*;
use bevy_integrator::{SimTime, Solver};
use car::{
    build::{build_car, car_startup_system, update_engine_audio, update_engine_speed, CarList},
    control::ControlType,
    egui_main_menu::EguiMainMenuPlugin,
    environment::build_environment,
    //main_menu::MainMenuPlugin, // Use the main menu plugin
    setup::{camera_setup, simulation_setup},
};
use rigid_body::plugin::RigidBodyPlugin;

// Main function
fn main() {
    // Create cars
    let mut car_definitions = Vec::new();
    car_definitions.push(build_car([0., 4., 0.], ControlType::WASD, 0));
    car_definitions.push(build_car([0., 0., 0.], ControlType::Arrow, 1)); // COMMENT THIS OUT IF YOU ONLY WANT 1 CAR

    let players = CarList {
        cars: car_definitions,
    };

    // Create App
    App::new()
        //.add_plugins(MainMenuPlugin)
        .add_plugins(RigidBodyPlugin {
            time: SimTime::new(0.002, 0.0, None),
            solver: Solver::RK4,
            simulation_setup: vec![simulation_setup],
            environment_setup: vec![camera_setup],
            name: "car_demo".to_string(),
        })
        .add_plugins(EguiMainMenuPlugin)
        .insert_resource(players)
        .add_systems(Startup, car_startup_system)
        .add_systems(Startup, build_environment)
        .add_systems(Update, (update_engine_speed, update_engine_audio))
        .run();
}
