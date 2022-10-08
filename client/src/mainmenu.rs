use bevy::{app::AppExit, prelude::*};

use crate::{ui::utils::basic_text, GameState};
pub struct MainMenuPlugin;

#[derive(Component, Default, Clone)]
pub struct OnlyInMainMenu;

#[derive(Component, Default, Clone)]
pub struct OnlyInCredits;

const DARKCOLOR: Color = Color::rgb(115. / 255., 23. / 255., 45. / 255.);
const LIGHTCOLOR: Color = Color::rgb(180. / 255., 32. / 255., 42. / 255.);
/*
* Structs to get the different text components
*/
#[derive(Component)]
struct AnimateTextFields();

/*
* Used to decide whether or not to display credits
*/
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MainMenuState {
    SplashScreen,
    Credits,
}

impl Default for MainMenuState {
    fn default() -> MainMenuState {
        MainMenuState::SplashScreen
    }
}

/*
* Used to decide whether or not to display credits
*/
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum HoverState {
    PlayGame,
    Credits,
    Quit,
}

impl Default for HoverState {
    fn default() -> HoverState {
        HoverState::PlayGame
    }
}
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HoverState::default())
            .insert_resource(MainMenuState::default())
            .add_state(HoverState::default())
            .add_state(MainMenuState::default())
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(spawn_ui))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(ui_controls))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(animate_text))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(cleanup));
    }
}

fn spawn_ui(asset_server: Res<AssetServer>, mut commands: Commands) {
    asset_server.load_folder("mainmenu").unwrap();
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgb(0., 0., 0.).into(),
            ..Default::default()
        })
        .insert(OnlyInMainMenu)
        .with_children(|root| {
            let font = asset_server.get_handle("mainmenu/alagrad.ttf");

            root.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(ImageBundle {
                    image: UiImage(asset_server.get_handle("mainmenu/mainmenuscreen.png")),
                    style: Style {
                        size: Size::new(Val::Px(480.), Val::Px(384.)),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
            root.spawn_bundle(basic_text(
                "Play game",
                24.,
                font.clone(),
                Some(70.),
                None,
                None,
                LIGHTCOLOR,
            ))
            .insert(AnimateTextFields());
            root.spawn_bundle(basic_text(
                "Credits",
                24.,
                font.clone(),
                Some(20.),
                None,
                None,
                DARKCOLOR,
            ))
            .insert(AnimateTextFields());
            root.spawn_bundle(basic_text(
                "Quit",
                24.,
                font.clone(),
                Some(20.),
                None,
                None,
                DARKCOLOR,
            ))
            .insert(AnimateTextFields());
        });
}

fn ui_controls(
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<State<GameState>>,
    mut main_menu_state: ResMut<State<MainMenuState>>,
    mut hover_state: ResMut<State<HoverState>>,
    mut credits_query: Query<Entity, With<OnlyInCredits>>,
    mut main_menu_query: Query<Entity, With<OnlyInMainMenu>>,
    mut app_exit_events: EventWriter<AppExit>,
    audio: Res<Audio>,
    mut commands: Commands,
) {
    let credit_entity = credits_query.get_single_mut();
    if main_menu_state.current() == &MainMenuState::SplashScreen {
        if keyboard_input.just_released(KeyCode::S) || keyboard_input.just_released(KeyCode::Down) {
            match hover_state.current() {
                HoverState::PlayGame => {
                    hover_state.set(HoverState::Credits).unwrap();
                    audio.play(asset_server.load("mainmenu/ui_button.ogg"));
                }
                HoverState::Credits => {
                    hover_state.set(HoverState::Quit).unwrap();
                    audio.play(asset_server.load("mainmenu/ui_button.ogg"));
                }
                HoverState::Quit => {
                    hover_state.set(HoverState::PlayGame).unwrap();
                    audio.play(asset_server.load("mainmenu/ui_button.ogg"));
                }
            }
        }
        if keyboard_input.just_released(KeyCode::W) || keyboard_input.just_released(KeyCode::Up) {
            match hover_state.current() {
                HoverState::PlayGame => {
                    hover_state.set(HoverState::Quit).unwrap();
                    audio.play(asset_server.load("mainmenu/ui_button.ogg"));
                }
                HoverState::Credits => {
                    hover_state.set(HoverState::PlayGame).unwrap();
                    audio.play(asset_server.load("mainmenu/ui_button.ogg"));
                }
                HoverState::Quit => {
                    hover_state.set(HoverState::Credits).unwrap();
                    audio.play(asset_server.load("mainmenu/ui_button.ogg"));
                }
            }
        }
        if keyboard_input.just_released(KeyCode::Return) {
            match hover_state.current() {
                HoverState::PlayGame => game_state.set(GameState::InGame).unwrap(),
                HoverState::Credits => {
                    main_menu_state.set(MainMenuState::Credits).unwrap();
                    /*
                    Spawn the credits information
                    */
                    commands
                        .entity(main_menu_query.single_mut())
                        .despawn_recursive();
                    commands
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                flex_direction: FlexDirection::ColumnReverse,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::rgb(0., 0., 0.).into(),
                            ..Default::default()
                        })
                        .insert(OnlyInCredits)
                        .with_children(|root| {
                            let font = asset_server.get_handle("mainmenu/alagrad.ttf");
                            root.spawn_bundle(basic_text(
                                "In CREDITS.txt. Return to exit.",
                                10.,
                                font.clone(),
                                None,
                                None,
                                None,
                                Color::rgb(115. / 255., 23. / 255., 45. / 255.),
                            ));
                        });
                }
                HoverState::Quit => app_exit_events.send(AppExit),
            }
        }
    } else {
        if keyboard_input.just_released(KeyCode::Return) {
            // Despawn all of the credits information
            commands.entity(credit_entity.unwrap()).despawn_recursive();
            //Set main menu state
            main_menu_state.set(MainMenuState::SplashScreen).unwrap();
            //Default the hover state
            hover_state.set(HoverState::default()).unwrap();
            /*
             * TODO: Put this into a seperate method so we don't have it in two places.
             */

            commands
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::rgb(0., 0., 0.).into(),
                    ..Default::default()
                })
                .insert(OnlyInMainMenu)
                .with_children(|root| {
                    let font = asset_server.load("mainmenu/alagrad.ttf");

                    root.spawn_bundle(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::FlexEnd,
                            ..Default::default()
                        },
                        color: Color::NONE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(ImageBundle {
                            image: UiImage(asset_server.load("mainmenu/mainmenuscreen.png")),
                            style: Style {
                                size: Size::new(Val::Px(480.), Val::Px(384.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
                    root.spawn_bundle(basic_text(
                        "Play game",
                        24.,
                        font.clone(),
                        Some(70.),
                        None,
                        None,
                        LIGHTCOLOR,
                    ))
                    .insert(AnimateTextFields());
                    root.spawn_bundle(basic_text(
                        "Credits",
                        24.,
                        font.clone(),
                        Some(20.),
                        None,
                        None,
                        DARKCOLOR,
                    ))
                    .insert(AnimateTextFields());
                    root.spawn_bundle(basic_text(
                        "Quit",
                        24.,
                        font.clone(),
                        Some(20.),
                        None,
                        None,
                        DARKCOLOR,
                    ))
                    .insert(AnimateTextFields());
                });
        }
    }
}

fn animate_text(
    mut text_query: Query<&mut Text, With<AnimateTextFields>>,
    hover_state: Res<State<HoverState>>,
) {
    for mut text in text_query.iter_mut() {
        if text.sections[f0].value == String::from("Play game") {
            if hover_state.current() == &HoverState::PlayGame {
                text.sections[0].style.color = LIGHTCOLOR;
            } else {
                text.sections[0].style.color = DARKCOLOR;
            }
        } else if text.sections[0].value == String::from("Credits") {
            if hover_state.current() == &HoverState::Credits {
                text.sections[0].style.color = LIGHTCOLOR;
            } else {
                text.sections[0].style.color = DARKCOLOR;
            }
        } else {
            if hover_state.current() == &HoverState::Quit {
                text.sections[0].style.color = LIGHTCOLOR;
            } else {
                text.sections[0].style.color = DARKCOLOR;
            }
        }
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<OnlyInMainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
