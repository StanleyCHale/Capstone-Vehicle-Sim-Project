use bevy::prelude::*;

// Some of the following code adapted from example code: https://github.com/johanhelsing/matchbox/tree/main/examples/bevy_ggrs

// Use the main menu plugin
use car::main_menu::MainMenuPlugin;
// Use the Game State


use bevy_integrator::{GameState, SimTime, Solver};
use car::{
    build::{build_car, car_startup_system, update_engine_audio, update_engine_speed, CarList},
    control::ControlType,
    environment::build_environment,
    setup::{camera_setup, simulation_setup},
};
use rigid_body::plugin::RigidBodyPlugin;

// Main function
/*
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
        .add_plugins(MainMenuPlugin)
        .add_plugins(RigidBodyPlugin {
            time: SimTime::new(0.002, 0.0, None),
            solver: Solver::RK4,
            simulation_setup: vec![simulation_setup],
            environment_setup: vec![camera_setup],
            name: "car_demo".to_string(),
        })
        .insert_resource(players)
        .add_systems(Startup, car_startup_system)
        .add_systems(Startup, build_environment)
        .add_systems(Update, (update_engine_speed, update_engine_audio))
        .run();
}
*/

// Main function
fn main() {
    // Create App
    App::new()
        .add_plugins(MainMenuPlugin)

        // TODO: Need to somehow seperate camera setup at this plugin to be called after the car is created when we enter the game state.
        // I am pretty sure plugins are added at compilation time so I am not sure how to do this.
        .add_plugins(RigidBodyPlugin {
            time: SimTime::new(0.002, 0.0, None),
            solver: Solver::RK4,
            simulation_setup: vec![simulation_setup],

            /* Crashes here since camera_setup requires a camera entity
            *  and the camera entity is not created yet (Done in build_car())
             */
            environment_setup: vec![camera_setup],
            name: "car_demo".to_string(),
        })
        .add_plugins(GameSetupPlugin)
        .run();
}

//System Set for InGame Setup
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct InGameSetup;

//Game setup plugin for when the game starts 
//Handles:
//  - Car Creation
//  - Terrain Creation
//  - Audio Setup
pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        
        // Create cars
        let mut car_definitions = Vec::new();
        car_definitions.push(build_car([0., 4., 0.], ControlType::WASD, 0));
        car_definitions.push(build_car([0., 0., 0.], ControlType::Arrow, 1)); // COMMENT THIS OUT IF YOU ONLY WANT 1 CAR

        //RESOURCE
        //List of players resource
        let players = CarList {
            cars: car_definitions,
        };


        app
        .insert_resource(players)
        .add_systems(OnEnter(GameState::InGame), (car_startup_system, build_environment))
        .add_systems(Update, (update_engine_speed, update_engine_audio).run_if(in_state(GameState::InGame)));
    }
}