use bevy::prelude::*;
use crate::GameState;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub mod menu {
    use bevy::{app::AppExit, prelude::*};
    use super::{despawn_screen, GameState};

    const FONT_PATH: &str = ".\\fonts\\Retro Gaming.ttf";
    const MENU_MUSIC: &str = ".\\music\\menu_music.ogg";

    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_state::<MenuState>()
                .add_systems(OnEnter(GameState::Menu), (menu_setup))
                .add_systems(OnEnter(MenuState::Main), main_menu_setup)
                .add_systems(OnExit(MenuState::Main), despawn_screen::<MainMenuScreen>)
                .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
                .add_systems(OnExit(MenuState::Settings), despawn_screen::<SettingsMenuScreen>)
                .add_systems(Update, (menu_action, button_system).run_if(in_state(GameState::Menu)));
        }
    }

    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        Settings,
        #[default]
        Disabled,
    }

    #[derive(Component)]
    enum MenuButtonAction {
        Continue,
        Settings,
        Exit,
    }

    #[derive(Component)]
    pub struct ButtonText;

    #[derive(Component)]
    struct MainMenuScreen;

    #[derive(Component)]
    struct SettingsMenuScreen;

    #[derive(Resource)]
    struct UiFont(Handle<Font>);

    fn menu_music_setup(
        asset_server: Res<AssetServer>,
        mut commands: Commands
    ) {
        commands.spawn(
            AudioBundle {
                source: asset_server.load(MENU_MUSIC),
                ..Default::default()
            }
        );
    }

    fn menu_setup(
        mut menu_state: ResMut<NextState<MenuState>>,
        mut commands: Commands,
        assets_server: Res<AssetServer>
    ) {
        menu_state.set(MenuState::Main);
        let ui_font_handle: Handle<Font> = assets_server.load(FONT_PATH);
        commands.insert_resource(UiFont(ui_font_handle));
    }

    fn button_system(
        mut interaction_query: Query<(&Interaction, &mut Children), (Changed<Interaction>, With<Button>)>,
        mut text_query: Query<(&mut Text, With<ButtonText>)>
    ) {
        for (interaction, children) in &mut interaction_query {
            for &child in children.iter() {
                let button_text = text_query.get_mut(child);
                match interaction {
                    Interaction::Pressed => { println!("Pressed button"); },
                    Interaction::Hovered => { println!("Hovered button"); button_text.unwrap().0.sections[0].style.color = Color::GREEN; },
                    Interaction::None => { button_text.unwrap().0.sections[0].style.color = Color::WHITE; },
                }
            }
        }
    }

    fn main_menu_setup(
        mut commands: Commands,
        ui_font: Res<UiFont>
    ) {
        commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            },
            MainMenuScreen,
        )).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            })
            .with_children(|parent| {

                parent.spawn(
                    TextBundle::from_section(
                        "MAIN MENU", 
                        TextStyle {
                            font: ui_font.0.clone(),
                            font_size: 80.0,
                            color: Color::WHITE,
                            ..Default::default()
                        }
                    )
                );

                parent.spawn((
                    ButtonBundle {
                        background_color: Color::NONE.into(),
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    MenuButtonAction::Continue
                ))
                .with_children(|parent| {
                    parent.spawn(
                        (TextBundle::from_section(
                            "Continue", 
                            TextStyle {
                                font: ui_font.0.clone(),
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..Default::default()
                            }
                        ),
                        ButtonText
                    ));
                });

                parent.spawn((
                    ButtonBundle {
                        background_color: Color::NONE.into(),
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    MenuButtonAction::Settings
                ))
                .with_children(|parent| {
                    parent.spawn(
                        (TextBundle::from_section(
                            "Settings", 
                            TextStyle {
                                font: ui_font.0.clone(),
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..Default::default()
                            }
                        ),
                        ButtonText
                    ));
                });

                parent.spawn((
                    ButtonBundle {
                        background_color: Color::NONE.into(),
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    MenuButtonAction::Exit
                ))
                .with_children(|parent| {
                    parent.spawn(
                        (TextBundle::from_section(
                            "Exit", 
                            TextStyle {
                                font: ui_font.0.clone(),
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..Default::default()
                            }
                        ),
                        ButtonText
                    ));
                });
            });
        });
    }

    fn settings_menu_setup() {

    }

    fn menu_action(
        interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
        mut app_exit_events: EventWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameState>>
    ){
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Continue => {
                        println!("Setting state to Game");
                        game_state.set(GameState::Game);
                        menu_state.set(MenuState::Disabled);
                    },
                    MenuButtonAction::Settings => {
                        println!("Setting state to Settings");
                        menu_state.set(MenuState::Settings)
                    },
                    MenuButtonAction::Exit => {
                        println!("Setting state to Exit");
                        app_exit_events.send(AppExit)
                    },
                }
            }
        }
    }
}
