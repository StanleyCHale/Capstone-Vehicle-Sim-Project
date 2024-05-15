use bevy::prelude::*;

// Some of the following code adapted from example code: https://github.com/johanhelsing/matchbox/tree/main/examples/bevy_ggrs

// Use the main menu plugin
use car::{ main_menu::MainMenuPlugin, setup::PhysicsSystemState};
// Use the Game State

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_integrator::{GameState, SimTime, Solver};
use car::{
    build::{build_car, car_startup_system, update_engine_audio, update_engine_speed, CarList},
    control::ControlType,
    environment::build_environment,
    setup::{camera_setup, simulation_setup},
};
use rigid_body::plugin::{CarState, RigidBodyPlugin};

// Main function
fn main() {
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
        .add_plugins(GameSetupPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        //Add game states
        .add_state::<CarState>()
        .add_state::<PhysicsSystemState>()
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

        let car_definitions = Vec::new();

        //RESOURCE
        //List of players resource
        let players = CarList {
            cars: car_definitions,
        };

        app.insert_resource(players);

        app
            .add_systems(
                OnEnter(GameState::InGame),
                (car_building_system, car_startup_system, build_environment).chain(),
            )
            .add_systems(
                Update,
                (update_engine_speed, update_engine_audio).run_if(in_state(GameState::InGame)),
            );
    }
}

//
fn car_building_system(
    mut car_list: ResMut<CarList>,
) {
    // Create cars
    let mut car_definitions = Vec::new();
    car_definitions.push(build_car([0., 4., 0.], ControlType::WASD, 0));
    car_definitions.push(build_car([0., 0., 0.], ControlType::Arrow, 1)); // COMMENT THIS OUT IF YOU ONLY WANT 1 CAR

    for car in car_definitions {
        car_list.cars.push(car);
    }
}
//
