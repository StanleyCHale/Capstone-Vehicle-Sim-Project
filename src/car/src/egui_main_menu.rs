#![allow(dead_code)]
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

// The following code adapted from the bevy_egui examples found here https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs and here https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/widget_gallery.rs
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct MainMenu {
    enabled: bool,
    visible: bool,
    opacity: f32
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            enabled: true,
            visible: true,
            opacity: 1.0,
        }
    }
}

impl MainMenu {
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        let Self {
            enabled: _,
            visible: _,
            opacity: _
        } = self;

        ui.label("Main Menu");
        ui.end_row();

        if ui.button("Start Game").clicked() {
            println!("egui: Start Game Clicked");
        }
        ui.end_row();
        if ui.button("Settings").clicked() {
            println!("egui: Settings Clicked");
        }
        ui.end_row();
        if ui.button("Quit Game").clicked() {
            println!("egui: Quit Game Clicked");
        }
        ui.end_row();
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Driver's Altitude")
                .resizable(false)
                .collapsible(false)
                .movable(false)
                .show(ctx, |ui| {
                    self.ui(ui, ctx);
                });
    }

    fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.add_enabled_ui(self.enabled, |ui| {
            ui.set_visible(self.visible);

            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    self.gallery_grid_contents(ui);
                });
        });

        ui.separator();

        let screen_width = ctx.input(|i| i.screen_rect().width());
        let screen_height = ctx.input(|i| i.screen_rect().height());

        ui.set_min_width(screen_width);
        ui.set_min_height(screen_height);
        ui.style_mut().wrap = Some(false);
    }
}

pub fn egui_main_menu(mut commands: Commands, mut contexts: EguiContexts) {
    
    let mut main_menu_struct = MainMenu {
        enabled: true,
        visible: true,
        opacity: 1.0
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        });

    let ctx = contexts.ctx_mut();

    // Show the main menu
    main_menu_struct.show(ctx);
}
