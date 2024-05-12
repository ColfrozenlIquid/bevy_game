use bevy::prelude::*;
use crate::AppState;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub mod menu {
    use bevy::{app::AppExit, prelude::*};
    use super::{despawn_screen, AppState};

    const FONT_PATH: &str = ".\\fonts\\Retro Gaming.ttf";
    // const MENU_MUSIC: &str = ".\\music\\menu_music.ogg";
    const CHARACTER_SPRITE_PATH: &str = ".\\sprites\\animated_characters_600.png";
    const UI_SPRITE_PATH: &str = ".\\sprites\\arrow_sprite_200.png";

    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app
                .init_state::<MenuState>()
                .insert_resource(CharacterSpriteAtlas::default())
                .insert_resource(UISpriteAtlas::default())
                .insert_resource(SelectedCharacter { current: 0, count: 3})

                .add_systems(OnEnter(AppState::MainMenu), menu_setup)
                .add_systems(OnEnter(MenuState::Main), main_menu_setup)
                .add_systems(OnExit(MenuState::Main), despawn_screen::<MainMenuScreen>)

                .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
                .add_systems(OnExit(MenuState::Settings), despawn_screen::<SettingsMenuScreen>)

                .add_systems(OnEnter(MenuState::Continue), continue_menu_setup)
                .add_systems(OnExit(MenuState::Continue), despawn_screen::<ContinueMenuScreen>)

                .add_systems(Update, (menu_action, character_selector_menu, button_system, animate_sprite).run_if(in_state(AppState::MainMenu)));
        }
    }

    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        Continue,
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
    enum CharacterSelectorButton {
        Next,
        Previous,
    }

    #[derive(Component, Clone)]
    struct AnimationIndices {
        first: usize,
        last: usize
    }

    #[derive(Component, Deref, DerefMut)]
    struct AnimationTimer(Timer);

    #[derive(Component)]
    pub struct ButtonText;

    #[derive(Component)]
    struct MainMenuScreen;

    #[derive(Component)]
    struct SettingsMenuScreen;

    #[derive(Component)]
    struct ContinueMenuScreen;

    #[derive(Resource)]
    struct UiFont(Handle<Font>);

    #[derive(Default, Resource)]
    struct CharacterSpriteAtlas {
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>
    }

    #[derive(Default, Resource)]
    struct UISpriteAtlas {
        image: Handle<Image>,
        layout: Handle<TextureAtlasLayout>
    }

    #[derive(Default, Resource)]
    struct SelectedCharacter {
        current: usize,
        count: usize,
    }

    // fn menu_music_setup(
    //     asset_server: Res<AssetServer>,
    //     mut commands: Commands
    // ) {
    //     commands.spawn(
    //         AudioBundle {
    //             source: asset_server.load(MENU_MUSIC),
    //             ..Default::default()
    //         }
    //     );
    // }

    fn menu_setup(
        mut menu_state: ResMut<NextState<MenuState>>,
        mut commands: Commands,
        assets_server: Res<AssetServer>,
        mut character_sprites: ResMut<CharacterSpriteAtlas>,
        mut ui_sprites: ResMut<UISpriteAtlas>,
        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        menu_state.set(MenuState::Main);
        let ui_font_handle: Handle<Font> = assets_server.load(FONT_PATH);
        commands.insert_resource(UiFont(ui_font_handle));

        {
            let texture = assets_server.load(CHARACTER_SPRITE_PATH);
            let layout = TextureAtlasLayout::from_grid( Vec2::new(96.0, 96.0), 4, 3, None, None);
            let texture_atlas_layout_handle = texture_atlases.add(layout);
            character_sprites.image = texture;
            character_sprites.layout = texture_atlas_layout_handle;
        }

        {
            let texture = assets_server.load(UI_SPRITE_PATH);
            let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 1, 1, None, None);
            let texture_atlas_layout_handle = texture_atlases.add(layout);
            ui_sprites.image = texture;
            ui_sprites.layout = texture_atlas_layout_handle;
        }
    }

    fn animate_sprite(
        time: Res<Time>,
        mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
        // selected_character: ResMut<SelectedCharacter>
    ) {
        for (indices, mut timer, mut sprite) in &mut query {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index == indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                };
            }
        }
    }

    fn button_system(
        // mut interaction_query: Query<(&Interaction, &mut Children), (Changed<Interaction>, With<Button>)>,
        // mut text_query: Query<(&mut Text, With<ButtonText>)>
    ) {
        // for (interaction, children) in &mut interaction_query {
        //     for &child in children.iter() {
        //         let button_text = text_query.get_mut(child);
        //         match interaction {
        //             Interaction::Pressed => { println!("Pressed button"); },
        //             Interaction::Hovered => { println!("Hovered button"); button_text.unwrap().0.sections[0].style.color = Color::GREEN; },
        //             Interaction::None => { button_text.unwrap().0.sections[0].style.color = Color::WHITE; },
        //         }
        //     }
        // }
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
            }).with_children(|parent| {
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
                    MenuButtonAction::Continue
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

    fn continue_menu_setup(
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        app_state.set(AppState::LoadingScreen);
        println!("Setting state to loading screen");
    }

    fn settings_menu_setup(
        mut commands: Commands,
        ui_font: Res<UiFont>,
        character_sprites: ResMut<CharacterSpriteAtlas>,
        ui_sprites: ResMut<UISpriteAtlas>,
        current_character: Res<SelectedCharacter>
    ) {
        let animation_indices = AnimationIndices {
            first: current_character.current * 4, 
            last: ((current_character.current + 1) * 4) - 1
        };

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
            }).with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Choose Character", 
                        TextStyle {
                            font: ui_font.0.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                            ..Default::default()
                        }
                    ),
                ));

                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::BLACK.into(),
                    ..Default::default()
                }).with_children(|parent| {
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
                        CharacterSelectorButton::Previous,
                    )).with_children(|parent| {
                        parent.spawn(
                            SpriteSheetBundle {
                                texture: ui_sprites.image.clone(),
                                atlas: TextureAtlas{
                                    layout: ui_sprites.layout.clone(),
                                    index: animation_indices.first,
                                },
                                // texture_atlas: ui_sprites.handle.clone(),
                                // texture_atlas_image: UiTextureAtlasImage {
                                //     index: 0,
                                //     ..Default::default()
                                // },
                                ..Default::default()
                            }
                        );
                    });

                    parent.spawn((
                        SpriteSheetBundle {
                            texture: character_sprites.image.clone(),
                            atlas: TextureAtlas {
                                layout: character_sprites.layout.clone(),
                                index: animation_indices.first,
                            },
                            // texture_atlas: character_sprites.handle.clone(),
                            // texture_atlas_image: UiTextureAtlasImage {
                            //     index: animation_indices.first,
                            //     ..Default::default()
                            // },
                            ..Default::default()
                        },
                        animation_indices.clone(),
                        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    ));

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
                        CharacterSelectorButton::Next,
                    )).with_children(|parent| {
                        parent.spawn(
                            SpriteSheetBundle {
                                texture: ui_sprites.image.clone(),
                                atlas: TextureAtlas {
                                    layout: ui_sprites.layout.clone(),
                                    index: animation_indices.first,
                                },
                                // texture_atlas: ui_sprites.handle.clone(),
                                // texture_atlas_image: UiTextureAtlasImage {
                                //     index: 0,
                                //     flip_x: true,
                                //     ..Default::default()
                                // },
                                ..Default::default()
                            }
                        );
                    });
                });
            });
        });
    }

    fn menu_action(
        interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
        mut app_exit_events: EventWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
    ){
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Continue => {
                        println!("Setting state to Game");
                        menu_state.set(MenuState::Continue);
                    },
                    MenuButtonAction::Settings => {
                        println!("Setting state to Settings");
                        menu_state.set(MenuState::Settings);
                    },
                    MenuButtonAction::Exit => {
                        println!("Setting state to Exit");
                        app_exit_events.send(AppExit);
                    },
                }
            }
        }
    }

    fn character_selector_menu(
        interaction_query: Query<(&Interaction, &CharacterSelectorButton), (Changed<Interaction>, With<Button>)>,
        mut selected_character: ResMut<SelectedCharacter>,
    ) {
        for (interaction, button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                println!("Pressed button");
                match button_action {
                    CharacterSelectorButton::Next =>  {
                        if selected_character.current + 1 != selected_character.count {
                            selected_character.current += 1;
                        }
                        else {
                            selected_character.current = 0;
                        }
                    },
                    CharacterSelectorButton::Previous =>  {
                        if selected_character.current != 0 {
                            selected_character.current -= 1;
                        }
                        else {
                            selected_character.current = selected_character.count - 1;
                        }
                    },
                }
            }
        }
    }
}
