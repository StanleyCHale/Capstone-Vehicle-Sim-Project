use bevy::prelude::*;
use bevy_egui::{egui::{self, Ui}, EguiContexts};

// The following code adapted from the bevy_egui examples found here https://github.com/mvlabat/bevy_egui/blob/main/examples/ui.rs and here: https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/widget_gallery.rs
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

        ui.add(doc_link_label("Label", "label"));
        ui.label("Welcome to the widget gallery!");
        ui.end_row();

        ui.add(doc_link_label("Button", "button"));
        if ui.button("Click me!").clicked() {
            // *boolean = !*boolean;
        }
        ui.end_row();
    }
    
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Main Menu")
                .open(open)
                .resizable(false)
                .default_width(280.0)
                .show(ctx, |ui| {
                    self.ui(ui);
                });
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
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
    }
}

fn doc_link_label<'a>(title: &'a str, search_term: &'a str) -> impl egui::Widget + 'a {
    doc_link_label_with_crate("egui", title, search_term)
}

fn doc_link_label_with_crate<'a>(
    crate_name: &'a str,
    title: &'a str,
    search_term: &'a str,
) -> impl egui::Widget + 'a {
    let label = format!("{title}:");
    let url = format!("https://docs.rs/{crate_name}?search={search_term}");
    move |ui: &mut egui::Ui| {
        ui.hyperlink_to(label, url).on_hover_ui(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Search egui docs for");
                ui.code(search_term);
            });
        })
    }
}

pub fn egui_main_menu(mut contexts: EguiContexts) {
    let main_menu_struct = MainMenu {
        enabled: true,
        visible: true,
        opacity: 1.0
    };
    
    main_menu_struct.gallery_grid_contents(ui);
}
