use bevy::prelude::Resource;
//This is a bevy plugin for the main menu of the game

use bevy::{prelude::*, ui::FocusPolicy};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu_setup)
            .add_systems(Update, handle_start_button);
    }
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

    //Insert our UI resource
    commands.insert_resource(ui_assets);
    
}


//Manages the assets that need to be loaded for the main menu UI
struct UiAssets {
    button: Handle<Image>,
    button_pressed: Handle<Image>,
}

impl Resource for UiAssets {}

//Runs when the start button is pushed
fn handle_start_button(
    mut commands: Commands, 
    interaction_query: Query<(&Children, &Interaction), Changed<Interaction>>,
    mut image_query: Query<&mut UiImage>,
    ui_assests: Res<UiAssets>,
    button_query: Query<Entity, With<Button>>
) {
    for (children, interaction) in interaction_query.iter() {
        let child = children.iter().next().unwrap();
        let mut image = image_query.get_mut(*child).unwrap();

        match interaction {
            Interaction::Pressed => {
                println!("Start Game Button Clicked");

                //Change the image to the pressed image
                image.texture = ui_assests.button_pressed.clone();

                //Need to do all of the setup for starting the game here

                //despawn the menu
                button_query.iter().for_each(|entity| {
                    commands.entity(entity).despawn_recursive();
                });

                //commands.spawn_bundle(StartGameBundle);
            }
            Interaction::Hovered => {
                println!("Start Game Button Hovered");
            }
            _ => {}
        }
    }
}

//Function to despawn the main menu
/* 
fn despawn_main_menu(mut commands: Commands, button_query: Query<Entity, With<Button>>) {
    for entity in button_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
*/