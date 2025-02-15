use bevy::{pbr::ExtendedMaterial, prelude::*};

// Use the main menu and preferences plugin
use car::preferences::{CarPreferences, PreferencesPlugin};

use bevy_integrator::{GameState, SimTime, Solver};

use car::{
    build::{build_car, car_startup_system, update_engine_audio, update_engine_speed, CarList},
    control::ControlType,
    egui_main_menu::EguiMainMenuPlugin,
    environment::build_environment,
    setup::{camera_setup, simulation_setup},
};
use grid_terrain::{examples::TerrainPreferences, MyExtension};
use rigid_body::plugin::{CarState, RigidBodyPlugin};

// Main function
fn main() {

    // Create App
    App::new()
        .add_state::<GameState>()
        .add_plugins(PreferencesPlugin)
        .insert_resource(
            TerrainPreferences {
                grid_size: 1600.0,
                subdivisions: 1024.0,
                seed: 1234567,
            }
        )
        .add_plugins((
            RigidBodyPlugin {
                time: SimTime::new(0.002, 0.0, None),
                solver: Solver::RK4,
                simulation_setup: vec![simulation_setup],
                environment_setup: vec![camera_setup],
                name: "car_demo".to_string(),
            },
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, MyExtension>>::default(),
        ))
        .add_plugins(EguiMainMenuPlugin)
        .insert_resource(Msaa::Off)
        .add_plugins(GameSetupPlugin)
        //Add game states
        .add_state::<CarState>()
        .run();
}

// Game setup plugin for when the game starts
// Handles:
//  - Car Creation
//  - Terrain Creation
//  - Audio Setup
pub struct GameSetupPlugin;

impl Plugin for GameSetupPlugin {
    fn build(&self, app: &mut App) {
        let car_definitions = Vec::new();

        // RESOURCE
        // List of players resource
        let players = CarList {
            cars: car_definitions,
        };

        app.insert_resource(players);

        app.add_systems(
            OnEnter(GameState::InGame),
            (car_building_system, car_startup_system, build_environment).chain(),
        )
        .add_systems(
            Update,
            (update_engine_speed, update_engine_audio).run_if(in_state(GameState::InGame)),
        );
    }
}

/*
 * Inputs: Queries for the list of players
 * Outputs: None
 * Description: This function defines the cars in the game as players
 */
fn car_building_system(mut car_list: ResMut<CarList>, car_preferences: Res<CarPreferences>) {
    //Access mass from car preferences
    let mass = car_preferences.mass;
    let max_speed = car_preferences.max_speed;
    let max_torque = car_preferences.max_torque;
    let friction_coefficient = car_preferences.friction_coefficient;

    // Create cars
    let mut car_definitions = Vec::new();
    car_definitions.push(build_car(
        [4., 0., 0.],
        ControlType::WASD,
        0,
        max_speed,
        mass,
        max_torque,
        friction_coefficient,
    ));
    car_definitions.push(build_car(
        [0., 0., 0.],
        ControlType::Arrow,
        1,
        max_speed,
        mass,
        max_torque,
        friction_coefficient,
    )); // COMMENT THIS OUT IF YOU ONLY WANT 1 CAR

    for car in car_definitions {
        car_list.cars.push(car);
    }
}
