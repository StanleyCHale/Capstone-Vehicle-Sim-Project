use bevy::app::AppExit;
use bevy::prelude::Resource;
//This is a bevy plugin for the main menu of the game

use bevy::{prelude::*, ui::FocusPolicy};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
        // Systems to handle the main menu screen
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        // Systems to handle the settings menu screen
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(OnExit(MenuState::Settings), despawn_screen::<OnSettingsMenuScreen>)
        
        .add_systems(Update, handle_menu_buttons);
    }
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsSound,
    SettingsDisplay,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    #[default]
    Main,
    Settings,
    SettingsSound,
    Disabled,
}

//Function for setting up the main menu UI of the game
fn main_menu_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    //Set the menu state to be the main menu
    menu_state.set(MenuState::Main);

    let ui_assets= UiAssets {
        button: asset_server.load("textures/ui/buttons/button.png"),
        button_pressed: asset_server.load("textures/ui/buttons/button_pressed.png"),
    };

    //Print statement
    println!("Main Menu Setup");

    //Spawn a camera for our 2d bundle
    //commands.spawn(UiCameraConfig::default());

    //This is a node bundle that will be the parent of all of our UI elements
    commands.spawn((NodeBundle {
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
        },
        //Tag this node as being the main menu screen
        OnMainMenuScreen,   
    )).with_children(|parent| {

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

//Function for setting up the settings menu UI of the game
fn settings_menu_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {

}


//Manages the assets that need to be loaded for the main menu UI
struct UiAssets {
    button: Handle<Image>,
    button_pressed: Handle<Image>,
}

impl Resource for UiAssets {}

fn handle_menu_buttons(
    mut commands: Commands, 
    interaction_query: Query<(&Children, &Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<MenuState>>,
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
        if interaction == &Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Quit Button Clicked");
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Play => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Play Game Button Clicked");
    
                    //Change main menu state to be disabled
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Settings Button Clicked");
    
                    //Change menu state to be in settings
                    menu_state.set(MenuState::Settings);
                }
                MenuButtonAction::SettingsSound => println!("Sound Button Clicked"),
                MenuButtonAction::BackToMainMenu => println!("Back to Main Menu Button Clicked"),
                MenuButtonAction::BackToSettings => println!("Back to Settings Button Clicked"),
                MenuButtonAction::SettingsDisplay => println!("Display Button Clicked"),
            }
            //
        }    
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}