use bevy::prelude::*;
use bevy_egui::{egui::*, EguiContexts, EguiPlugin, EguiSettings};

//This is a bevy plugin for the main menu of the game
pub struct EguiMainMenuPlugin;

impl Plugin for EguiMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, egui_example_system);
    }
}

// Some of the following code adapted from the bevy_egui examples found here: https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs
fn egui_example_system(mut contexts: EguiContexts) {
    bevy_egui::egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
