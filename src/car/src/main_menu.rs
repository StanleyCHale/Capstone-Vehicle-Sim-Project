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
        //Systems to handle the audio settings menu screen
        .add_systems(OnEnter(MenuState::SettingsAudio), settingsaudio_menu_setup)
        .add_systems(OnExit(MenuState::SettingsAudio), despawn_screen::<OnAudioMenuScreen>)
        //SYstem to handle the vehicle settings menu screen
        .add_systems(OnEnter(MenuState::SettingsVehicle), settingsvehicle_menu_setup)
        .add_systems(OnExit(MenuState::SettingsVehicle), despawn_screen::<OnVehicleMenuScreen>)
        
        .add_systems(Update, handle_menu_buttons);
    }
}

//RESOURCE
//Manages the assets that need to be loaded for the main menu UI
impl Resource for UiAssets {}
struct UiAssets {
    button: Handle<Image>,
    button_pressed: Handle<Image>,
    button_yellow: Handle<Image>,
    button_grey: Handle<Image>,
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;
// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
struct OnSettingsMenuScreen;
//Tag componenet used to tag entities added on the audio settings screen
#[derive(Component)]
struct OnAudioMenuScreen;
//Tag componenet used to tag entities added on the audio settings screen
#[derive(Component)]
struct OnVehicleMenuScreen;

// Menu action tags for signifying what to do when a button is pressed
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsAudio,
    SettingsVehicle,
    BackToMainMenu,
    BackToSettings,
    VolumeSet0,
    VolumeSet2,
    VolumeSet4,
    VolumeSet6,
    VolumeSet8,
    VolumeSet10,
    Mass500,
    Mass1000,
    Mass2000,
    GravityMoon,
    GravityEarth,
    GravityJupiter,
    MaxSpeed25,
    MaxSpeed75,
    MaxSpeed150,
    CarAcceleration6,
    CarAcceleration10,
    CarAcceleration15,
    Quit,
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    #[default]
    Main,
    Settings,
    SettingsAudio,
    SettingsVehicle,
    Disabled,
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

// System to handle the main menu state
// This system will catch each interaction event and uses a component to decide what to do
fn handle_menu_buttons(
    interaction_query: Query<(&Children, &Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut image_query: Query<&mut UiImage>,
    ui_assests: Res<UiAssets>,
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
                MenuButtonAction::SettingsAudio => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Sound Button Clicked");

                    //Change menu state to be in settings
                    menu_state.set(MenuState::SettingsAudio);
                }
                MenuButtonAction::SettingsVehicle => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Vehicle Button Clicked");

                    //Change menu state to be in settings
                    menu_state.set(MenuState::SettingsVehicle);
                }
                MenuButtonAction::BackToMainMenu =>  {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Back to Main Menu Button Clicked");

                    //Change main menu state to be Main Menu
                    menu_state.set(MenuState::Main);
                },
                MenuButtonAction::BackToSettings =>  {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Back to Settings Button Clicked");

                    //Change main menu state to be Settings Menu
                    menu_state.set(MenuState::Settings);
                }

                //Volume settings
                MenuButtonAction::VolumeSet0 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Volume Set to 0.0");
                }
                MenuButtonAction::VolumeSet2 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Volume Set to 0.2");
                }
                MenuButtonAction::VolumeSet4 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Volume Set to 0.4");
                }
                MenuButtonAction::VolumeSet6 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Volume Set to 0.6");
                }
                MenuButtonAction::VolumeSet8 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Volume Set to 0.8");
                }
                MenuButtonAction::VolumeSet10 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Volume Set to 1.0");
                }

                //Mass settings
                MenuButtonAction::Mass500 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Mass Set to 500");
                }
                MenuButtonAction::Mass1000 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Mass Set to 1000");
                }
                MenuButtonAction::Mass2000 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Mass Set to 2000");
                }

                //Gravity settings
                MenuButtonAction::GravityMoon => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Gravity Set to Moon");
                }
                MenuButtonAction::GravityEarth => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Gravity Set to Earth");
                }
                MenuButtonAction::GravityJupiter => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Gravity Set to Jupiter");
                }   

                //Max Speed settings
                MenuButtonAction::MaxSpeed25 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Max Speed Set to 25");
                }   
                MenuButtonAction::MaxSpeed75 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Max Speed Set to 75");
                }
                MenuButtonAction::MaxSpeed150 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Max Speed Set to 150");
                }

                //Car Acceleration settings
                MenuButtonAction::CarAcceleration6 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Car Acceleration Set to 6");
                }
                MenuButtonAction::CarAcceleration10 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Car Acceleration Set to 10");
                }
                MenuButtonAction::CarAcceleration15 => {
                    image.texture = ui_assests.button_pressed.clone();
                    println!("Car Acceleration Set to 15");
                }

            }
        }    
    }
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
        button_yellow: asset_server.load("textures/ui/buttons/button_yellow.png"),
        button_grey: asset_server.load("textures/ui/buttons/button_grey.png"),
    };

    //Print statement
    println!("Main Menu Setup");

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

        //Main title of the game
        parent.spawn(ImageBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(20.0)),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: ui_assets.button_yellow.clone().into(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Driver's Altitude", 
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
    ui_assets: Res<UiAssets>,
) {

    //Print statement
    println!("Settings Menu Setup");

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
        OnSettingsMenuScreen,   
    )).with_children(|parent| {


        //Settings menu title
        parent.spawn(ImageBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(20.0)),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: ui_assets.button_yellow.clone().into(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Settings Menu", 
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

        
        //Spawn a button bundle for the Audio settings button
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
            MenuButtonAction::SettingsAudio,
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
                        "Audio Settings", 
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
            MenuButtonAction::SettingsVehicle,
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
                        "Vehicle Settings", 
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
            MenuButtonAction::BackToMainMenu,
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
                        "Back to Main Menu", 
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
}

//Function for setting up the audio settings menu UI of the game
fn settingsaudio_menu_setup(
    mut commands: Commands, 
    ui_assets: Res<UiAssets>,
) {

    //Print statement
    println!("Audio Settings Menu Setup");

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
    OnAudioMenuScreen,   
    )).with_children(|parent| {

        //Spawn text for the title of the audio settings
        parent.spawn(ImageBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(20.0)),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: ui_assets.button_yellow.clone().into(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Audio Settings", 
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


        //Spawn text for the title of the Mass of the car
        parent.spawn(ImageBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(20.0)),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: ui_assets.button_grey.clone().into(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Audio Volume", 
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..Default::default()
                    },
                ),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            });
        });

        //Node for the volume options
        parent.spawn(NodeBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                margin: UiRect::all(Val::Px(20.0)),
                // Place children in a row
                flex_direction: FlexDirection::Row,
                // Center children
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::ALICE_BLUE),
            ..default()
        }
        ).with_children(|parent| {
            //Spawn a button for 0 volume
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(20.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::VolumeSet0,
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
                            " 0 ", 
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

            //Spawn a button for 2 volume
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(20.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::VolumeSet2,
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
                            " 2 ", 
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

            //Spawn a button for 4 volume
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(20.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::VolumeSet4,
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
                            " 4 ", 
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

            //Spawn a button for 6 volume
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(20.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::VolumeSet6,
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
                            " 6 ", 
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

            //Spawn a button for 8 volume
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(20.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::VolumeSet8,
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
                            " 8 ", 
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

            //Spawn a button for 10 volume (Max)
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(20.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::VolumeSet10,
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
                            " 10 ", 
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
        
        });
        

        //Spawn a button bundle for getting back to the settings menu
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
            MenuButtonAction::BackToSettings,
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
                        "Back to Settings", 
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

}

fn settingsvehicle_menu_setup(
    mut commands: Commands, 
    ui_assets: Res<UiAssets>,
) {


    //Print statement
    println!("Settings Vehicle Setup");

    //This is a node bundle that will be the parent of all of our UI elements
    commands.spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Place children in a row
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
        OnVehicleMenuScreen,   
    )).with_children(|parent| {

        //Spawn text for the title of the audio settings
        parent.spawn(ImageBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(20.0)),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: ui_assets.button_yellow.clone().into(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Vehicle Settings", 
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


        //Spawn text for the title of the Mass of the car
        parent.spawn(ImageBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(10.0)),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: ui_assets.button_grey.clone().into(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Mass (kg)", 
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..Default::default()
                    },
                ),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            });
        });

        //Node for the Mass options
        parent.spawn(NodeBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                margin: UiRect::all(Val::Px(0.0)),
                // Place children in a column
                flex_direction: FlexDirection::Row,
                // Center children
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::ALICE_BLUE),
            ..default()
        }
        ).with_children(|parent| {
            //Spawn a button for 500 Mass
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::Mass500,
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
                            " 500 ", 
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

            //Spawn a button for 1000 Mass
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::Mass1000,
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
                            " 1000 ", 
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

            //Spawn a button for 2000 Mass
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::Mass2000,
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
                            " 2000 ", 
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

        });

        //Spawn text for the title of the Gravity
        parent.spawn(ImageBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(10.0)),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: ui_assets.button_grey.clone().into(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Gravity (m/s)", 
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..Default::default()
                    },
                ),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            });
        });

        //Node for the Gravity options
        parent.spawn(NodeBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                margin: UiRect::all(Val::Px(0.0)),
                // Place children in a column
                flex_direction: FlexDirection::Row,
                // Center children
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::ALICE_BLUE),
            ..default()
        }
        ).with_children(|parent| {
            //Spawn a button for 1.62 (Moon Gravity)
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::GravityMoon,
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
                            " Moon 1.62 ", 
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

            //Spawn a button for 9.81 (Earth Gravity)
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::GravityEarth,
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
                            " Earth 9.81 ", 
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

            //Spawn a button for 24.79 (Jupiter Gravity)
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::GravityJupiter,
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
                            " Jupiter 24.79 ", 
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

        });


        //Spawn text for the title of the Max speed of the car
        parent.spawn(ImageBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(10.0)),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: ui_assets.button_grey.clone().into(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Max Speed (m/s)", 
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..Default::default()
                    },
                ),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            });
        });

        //Node for the Mass options
        parent.spawn(NodeBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                margin: UiRect::all(Val::Px(0.0)),
                // Place children in a column
                flex_direction: FlexDirection::Row,
                // Center children
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::ALICE_BLUE),
            ..default()
        }
        ).with_children(|parent| {
            //Spawn a button for 25 max speed
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::MaxSpeed25,
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
                            " 25 ", 
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

            //Spawn a button for 75 max speed
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::MaxSpeed75,
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
                            " 75 ", 
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

            //Spawn a button for 150 max speed
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::MaxSpeed150,
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
                            " 150 ", 
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

        });


        //Spawn text for the title of the acceleration of the car
        parent.spawn(ImageBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(10.0)),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: ui_assets.button_grey.clone().into(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Car Acceleration (m/s^2)", 
                    TextStyle {
                        font_size: 40.0,
                        color: Color::BLACK,
                        ..Default::default()
                    },
                ),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            });
        });

        //Node for the Mass options
        parent.spawn(NodeBundle {
            style: Style {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                margin: UiRect::all(Val::Px(0.0)),
                // Place children in a column
                flex_direction: FlexDirection::Row,
                // Center children
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::ALICE_BLUE),
            ..default()
        }
        ).with_children(|parent| {
            //Spawn a button for 6 acc
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::CarAcceleration6,
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
                            " 6 ", 
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

            //Spawn a button for 10 acc
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::CarAcceleration10,
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
                            " 10 ", 
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

            //Spawn a button for 15 acc
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        min_width: Val::Vw(5.0),
                        min_height: Val::Vh(5.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
                MenuButtonAction::CarAcceleration15,
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
                            " 15 ", 
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

        });
        

        //Spawn a button bundle for the Exit button
        parent.spawn((
            ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Px(10.0)),
                min_width: Val::Vw(20.0),
                min_height: Val::Vh(6.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
            },
            MenuButtonAction::BackToSettings,
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
                        "Back", 
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

}
