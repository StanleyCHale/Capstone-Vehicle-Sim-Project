//This is a bevy plugin for the main menu of the game

use bevy::{prelude::*, ui::FocusPolicy};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu_setup);
    }
}

//Manages the assets that need to be loaded for the main menu UI
struct UiAssets {
    button: Handle<Image>,
    button_pressed: Handle<Image>,
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>,) {
    let ui_assets= UiAssets {
        button: asset_server.load("textures/ui/buttons/button.png"),
        button_pressed: asset_server.load("textures/ui/buttons/button_pressed.png"),
    };

    //Print statement
    println!("Main Menu Setup");

    //Spawn a camera for our 2d bundle
    //commands.spawn(UiCameraConfig::default());

    //Spawn a button bundle for the Start Game button
    commands.spawn(ButtonBundle {
        style: Style {
            align_self: AlignSelf::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Auto),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(ImageBundle {
            style: Style {
                max_width: Val::Percent(20.0),
                max_height: Val::Percent(10.0),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            image: ui_assets.button.clone().into(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Start Game", 
                    TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            });
        });
    
    });


}