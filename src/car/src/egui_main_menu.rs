#![allow(dead_code)]
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_integrator::GameState;

use crate::preferences::CarPreferences;

// Egui Main Menu Plugin
pub struct EguiMainMenuPlugin;

impl Plugin for EguiMainMenuPlugin {
    fn build(&self, app: &mut App) {
        let main_menu_struct = MainMenu::default();

        app
            .add_plugins(EguiPlugin) // EguiPlugin is needed for literally all bevy_egui functionality
            .insert_resource(main_menu_struct)
            .add_systems(Update, egui_main_menu); // "Main" function for this file
    }
}

// ENUM
// Enum used for the current menu screen state
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    #[default]
    Main,
    Settings,
    SettingsAudio,
    SettingsVehicle,
    SettingsTerrain,
    Disabled,
}

// The following code adapted from the bevy_egui examples found here https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs and here https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/widget_gallery.rs
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Resource)]
pub struct MainMenu {
    menu: MenuState,
    visible: bool,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            menu: MenuState::Main,
            visible: true,
        }
    }
}

impl MainMenu {
    fn show(&mut self, ctx: &egui::Context, app_exit_events: EventWriter<AppExit>, game_state: ResMut<NextState<GameState>>, car_preferences: ResMut<CarPreferences>,) {
        match self.menu {
            MenuState::Main => {
                self.gallery_main_contents(ctx, app_exit_events, game_state);
            }
            MenuState::Settings => {
                self.gallery_settings_contents(ctx);
            }
            MenuState::SettingsAudio => {
                self.gallery_audio_settings_contents(ctx, car_preferences);
            }
            MenuState::SettingsVehicle => {
                self.gallery_vehicle_settings_contents(ctx, car_preferences);
            }
            MenuState::SettingsTerrain => {
                self.gallery_terrain_settings_contents(ctx);
            }
            MenuState::Disabled => {}
        }
    }

    /*
     * This function generates the actual UI of the main menu. Any UI element rearranging should be done here.
     */
    fn gallery_main_contents(
        &mut self,
        ctx: &egui::Context,
        app_exit_events: EventWriter<AppExit>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        let Self {
            menu: _,
            visible: _,
        } = self;

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {     // Center the UI

                // Modify text size
                let mut custom_style = egui::Style::default();

                custom_style.text_styles = [
                    (egui::TextStyle::Heading, egui::FontId::proportional(64.0)),
                    (egui::TextStyle::Body, egui::FontId::proportional(32.0)),
                    (egui::TextStyle::Button, egui::FontId::proportional(18.0)),
                    (egui::TextStyle::Small, egui::FontId::proportional(14.0)),
                ]
                .into();

                ctx.set_style(custom_style);

                // Start of UI elements
                ui.heading("Driver's Altitude");

                ui.add_space(60.0); // Space between header and label

                ui.add(egui::Label::new("Main Menu"));

                ui.add_space(20.0); // Space between label and buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Start Game"))
                    .clicked()
                {
                    //Transition to the "In Game" state
                    game_state.set(GameState::InGame);
                    
                    self.menu = MenuState::Disabled;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Settings"))
                    .clicked()
                {
                    self.menu = MenuState::Settings;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Quit"))
                    .clicked()
                {
                    exit_program(app_exit_events);
                }

                ui.end_row();
            });
        });
    }

    /*
     * This function generates the actual UI of the main menu. Any UI element rearranging should be done here.
     */
    fn gallery_settings_contents(
        &mut self,
        ctx: &egui::Context,
    ) {
        let Self {
            menu: _,
            visible: _,
        } = self;

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {     // Center the UI

                // Modify text size
                let mut custom_style = egui::Style::default();

                custom_style.text_styles = [
                    (egui::TextStyle::Heading, egui::FontId::proportional(64.0)),
                    (egui::TextStyle::Body, egui::FontId::proportional(32.0)),
                    (egui::TextStyle::Button, egui::FontId::proportional(18.0)),
                    (egui::TextStyle::Small, egui::FontId::proportional(14.0)),
                ]
                .into();

                ctx.set_style(custom_style);

                // Start of UI elements
                ui.heading("Driver's Altitude");

                ui.add_space(60.0); // Space between header and label

                ui.add(egui::Label::new("Settings Menu"));

                ui.add_space(20.0); // Space between label and buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Audio Settings"))
                    .clicked()
                {
                    self.menu = MenuState::SettingsAudio;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Vehicle Settings"))
                    .clicked()
                {
                    self.menu = MenuState::SettingsVehicle;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Terrain Settings"))
                    .clicked()
                {
                    self.menu = MenuState::SettingsTerrain;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Back"))
                    .clicked()
                {
                    self.menu = MenuState::Main;
                }

                ui.end_row();
            });
        });
    }

    /*
     * This function generates the actual UI of the main menu. Any UI element rearranging should be done here.
     */
    fn gallery_audio_settings_contents(
        &mut self,
        ctx: &egui::Context,
        mut car_preferences: ResMut<CarPreferences>,
    ) {
        let Self {
            menu: _,
            visible: _,
        } = self;

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {     // Center the UI

                // Modify text size
                let mut custom_style = egui::Style::default();

                custom_style.text_styles = [
                    (egui::TextStyle::Heading, egui::FontId::proportional(64.0)),
                    (egui::TextStyle::Body, egui::FontId::proportional(32.0)),
                    (egui::TextStyle::Button, egui::FontId::proportional(18.0)),
                    (egui::TextStyle::Small, egui::FontId::proportional(14.0)),
                ]
                .into();

                ctx.set_style(custom_style);

                // Start of UI elements
                ui.heading("Driver's Altitude");

                ui.add_space(60.0); // Space between header and label

                ui.add(egui::Label::new("Audio Settings Menu"));

                ui.add_space(20.0); // Space between label and buttons
                ui.end_row();

                //Grab the current value of the volume
                let mut my_f64 = car_preferences.volume;
                if ui
                    .add(egui::Slider::new(&mut my_f64, 0.0..=1.0).text("Audio Volume"))
                    .changed() 
                {
                    //Update the volume
                    car_preferences.volume = my_f64;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Back"))
                    .clicked()
                {
                    self.menu = MenuState::Main;
                }

                ui.end_row();
            });
        });
    }

    /*
     * This function generates the actual UI of the main menu. Any UI element rearranging should be done here.
     */
    fn gallery_vehicle_settings_contents(
        &mut self,
        ctx: &egui::Context,
        mut car_preferences: ResMut<CarPreferences>,
    ) {
        
        let Self {
            menu: _,
            visible: _,
        } = self;

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {     // Center the UI

                // Modify text size
                let mut custom_style = egui::Style::default();

                custom_style.text_styles = [
                    (egui::TextStyle::Heading, egui::FontId::proportional(64.0)),
                    (egui::TextStyle::Body, egui::FontId::proportional(32.0)),
                    (egui::TextStyle::Button, egui::FontId::proportional(18.0)),
                    (egui::TextStyle::Small, egui::FontId::proportional(14.0)),
                ]
                .into();

                ctx.set_style(custom_style);

                // Start of UI elements
                ui.heading("Driver's Altitude");

                ui.add_space(60.0); // Space between header and label

                ui.add(egui::Label::new("Vehicle Settings Menu"));

                ui.add_space(20.0); // Space between label and buttons
                ui.end_row();

                //Grab the current value of the chassis mass
                let mut my_f64 = car_preferences.mass;
                if ui
                    .add(egui::Slider::new(&mut my_f64, 1.0..=15000.0).text("Chassis Mass (kg)"))
                    .changed() 
                {
                    //Update the chassis mass
                    car_preferences.mass = my_f64;
                }

                //Grab the current value of gravity
                my_f64 = car_preferences.gravity;
                if ui
                    .add(egui::Slider::new(&mut my_f64, 1.0..=200.0).text("Gravity (m/s)"))
                    .changed() 
                {
                    //Update the gravity
                    car_preferences.gravity= my_f64;
                }

                //Grab the current value of the max speed
                my_f64 = car_preferences.max_speed;
                if ui
                    .add(egui::Slider::new(&mut my_f64, 1.0..=200.0).text("Max Speed (m/s)"))
                    .changed() 
                {
                    //Update the max speed
                    car_preferences.max_speed= my_f64;
                }

                //Grab the current value of the max torque
                my_f64 = car_preferences.max_torque;
                if ui
                    .add(egui::Slider::new(&mut my_f64, 1.0..=10000.0).text("Car Torque (Nm)"))
                    .changed() 
                {
                    //Update the max torque
                    car_preferences.max_torque= my_f64;
                }

                //Grab the current value of the friction coefficient
                my_f64 = car_preferences.friction_coefficient;
                if ui
                    .add(egui::Slider::new(&mut my_f64, 0.0..=5.0).text("Friction Coefficient"))
                    .changed() 
                {
                    //Update the friction coefficient
                    car_preferences.friction_coefficient= my_f64;
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Back"))
                    .clicked()
                {
                    self.menu = MenuState::Settings;
                }

                ui.end_row();
            });
        });
    }
 
    /*
     * This function generates the actual UI of the main menu. Any UI element rearranging should be done here.
     */
    fn gallery_terrain_settings_contents(
        &mut self,
        ctx: &egui::Context,
    ) {
        let Self {
            menu: _,
            visible: _,
        } = self;

        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {     // Center the UI

                // Modify text size
                let mut custom_style = egui::Style::default();

                custom_style.text_styles = [
                    (egui::TextStyle::Heading, egui::FontId::proportional(64.0)),
                    (egui::TextStyle::Body, egui::FontId::proportional(32.0)),
                    (egui::TextStyle::Button, egui::FontId::proportional(18.0)),
                    (egui::TextStyle::Small, egui::FontId::proportional(14.0)),
                ]
                .into();

                ctx.set_style(custom_style);

                // Start of UI elements
                ui.heading("Driver's Altitude");

                ui.add_space(60.0); // Space between header and label

                ui.add(egui::Label::new("Terrain Settings Menu"));

                ui.add_space(20.0); // Space between label and buttons
                ui.end_row();

                
                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Generate New Terrain Seed"))
                    .clicked()
                {
                    //Generate a new terrain seed
                    todo!("Generate a new terrain seed");
                }

                ui.end_row();

                //Grab the current value of the volume
                let mut my_string = String::from("0");
                let response = ui.add(egui::TextEdit::singleline(&mut my_string));
                if response.changed() {
                    // … Considering just using lost_focus() instead
                    todo!("Update the terrain seed on change??");
                }
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    // …
                    todo!("Update the terrain seed on losing focus");
                }

                ui.add_space(10.0); // Space between buttons
                ui.end_row();

                if ui
                    .add_sized([200.0, 50.0], egui::Button::new("Back"))
                    .clicked()
                {
                    self.menu = MenuState::Main;
                }

                ui.end_row();
            });
        });
    }
}

pub fn egui_main_menu(
    mut contexts: EguiContexts,
    mut main_menu_struct: ResMut<MainMenu>,
    app_exit_events: EventWriter<AppExit>,
    game_state: ResMut<NextState<GameState>>,
    car_preferences: ResMut<CarPreferences>,
) {
    let ctx = contexts.ctx_mut();

    // Show the main menu
    main_menu_struct.show(ctx, app_exit_events, game_state, car_preferences);
}

/*
 * Exits the program when called
 */
pub fn exit_program(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit);
}
