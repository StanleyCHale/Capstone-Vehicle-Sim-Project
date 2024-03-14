use bevy::app::AppExit;
use bevy::prelude::Resource;
//This is a bevy plugin for the main menu of the game

use bevy::{prelude::*, ui::FocusPolicy};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu_setup)
            .add_systems(Update, handle_menu_buttons);
    }
}

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
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

    //This is a node bundle that will be the parent of all of our UI elements
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            // Place children in a column
            flex_direction: FlexDirection::Column,
            // Center children
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::ALICE_BLUE),
        ..default()
    }).with_children(|parent| {

        //Spawn a button bundle for the Start Game button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::all(Val::Px(20.0)),
                    min_width: Val::Vw(20.0),
                    min_height: Val::Vh(6.0),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..Default::default()
            },
            MenuButtonAction::Play,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    max_width: Val::Percent(100.0),
                    max_height: Val::Percent(100.0),
                    margin: UiRect::all(Val::Percent(0.0)),
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

        //Spawn a button bundle for the Settings button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::all(Val::Px(20.0)),
                    min_width: Val::Vw(20.0),
                    min_height: Val::Vh(6.0),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..Default::default()
            },
            MenuButtonAction::Settings,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    max_width: Val::Percent(100.0),
                    max_height: Val::Percent(100.0),
                    margin: UiRect::all(Val::Percent(0.0)),
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
                        "Settings", 
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

        //Spawn a button bundle for the Exit button
        parent.spawn((
            ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Px(20.0)),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
            },
            MenuButtonAction::Quit,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    max_width: Val::Percent(100.0),
                    max_height: Val::Percent(100.0),
                    margin: UiRect::all(Val::Percent(0.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                image: ui_assets.button.clone().into(),
                ..Default::default()
            })
            .insert(FocusPolicy::Pass)
            //
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Quit Game", 
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
            //
        });
        //
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
//Currently handles all of the buttons atm//
fn handle_menu_buttons(
    mut commands: Commands, 
    interaction_query: Query<(&Children, &Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut background_query: Query<&mut BackgroundColor>,
    mut image_query: Query<&mut UiImage>,
    ui_assests: Res<UiAssets>,
    button_query: Query<Entity, With<Button>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    //For every button interaction found, we will run this code
    for (children, interaction, menu_button_action) in &interaction_query {
        let child = children.iter().next().unwrap();
        let mut image = image_query.get_mut(*child).unwrap();

        //
        if(interaction == &Interaction::Pressed) {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Quit Button Clicked");
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Play => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Play Game Button Clicked");
    
                    //set background colour to none
                    background_query.iter_mut().for_each(|mut background| {
                        background.0 = Color::NONE;
                    });
    
                    //despawn the menu
                    button_query.iter().for_each(|entity| {
                        commands.entity(entity).despawn_recursive();
                    });
                }
                MenuButtonAction::Settings => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Settings Button Clicked");
    
                    //Change visibility of quit button to invisible

                    //Change visibility of settings buttons to visible
                }
                MenuButtonAction::SettingsSound => println!("Sound Button Clicked"),
                MenuButtonAction::BackToMainMenu => println!("Back to Main Menu Button Clicked"),
                MenuButtonAction::BackToSettings => println!("Back to Settings Button Clicked"),
            }
            //
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