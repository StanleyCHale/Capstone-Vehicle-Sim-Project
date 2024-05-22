#![allow(dead_code)]
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

// Egui Main Menu Plugin
pub struct EguiMainMenuPlugin;

impl Plugin for EguiMainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin) // EguiPlugin is needed for literally all bevy_egui functionality
            .add_systems(Update, egui_main_menu); // "Main" function for this file
    }
}

// The following code adapted from the bevy_egui examples found here https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs and here https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/widget_gallery.rs
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct MainMenu {
    enabled: bool,
    visible: bool,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            enabled: true,
            visible: true,
        }
    }
}

impl MainMenu {
    fn show(&mut self, ctx: &egui::Context, app_exit_events: EventWriter<AppExit>) {
        self.gallery_grid_contents(ctx, app_exit_events);
    }

    /*
     * This function generates the actual UI of the main menu. Any UI element rearranging should be done here.
     */
    fn gallery_grid_contents(
        &mut self,
        ctx: &egui::Context,
        app_exit_events: EventWriter<AppExit>,
    ) {
        let Self {
            enabled: _,
            visible: _,
        } = self;

        if self.enabled {
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
                        self.enabled = false;
                    }

                    ui.add_space(10.0); // Space between buttons
                    ui.end_row();

                    if ui
                        .add_sized([200.0, 50.0], egui::Button::new("Settings"))
                        .clicked()
                    {
                        println!("egui: Settings Clicked");
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
    }
}

pub fn exit_program(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit);
}

pub fn egui_main_menu(
    mut commands: Commands,
    mut contexts: EguiContexts,
    app_exit_events: EventWriter<AppExit>,
) {
    let mut main_menu_struct = MainMenu {
        enabled: true,
        visible: true,
    };

    commands.spawn(NodeBundle {
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
    main_menu_struct.show(ctx, app_exit_events);
}
