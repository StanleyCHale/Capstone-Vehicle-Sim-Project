//This is a bevy plugin for the user preferences of the game
use bevy::prelude::*;

pub struct PreferencesPlugin;


//RESOURCE
//Manages the user's preferences for the vehicle
#[derive(Resource)]
pub struct CarPreferences {
    pub mass: f64,
    pub gravity: f64,
    pub max_speed: f64,
    pub max_torque: f64,
}

impl Default for CarPreferences {
        fn default() -> Self {
            CarPreferences {
                mass: 1000.0,
                gravity: 9.81,
                max_speed: 75.0,
                max_torque: 1000.0,
            }
        }
    }

impl Plugin for PreferencesPlugin {

    fn build(&self, app: &mut App) {
        //Set default preferences
        let car_preferences = CarPreferences::default();

        //Setup systems and insert resources
        app
        .init_resource::<CarPreferences>()
        .insert_resource(car_preferences);
    }
}