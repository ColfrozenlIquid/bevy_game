use bevy::prelude::*;
use crate::GameState;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub mod menu {
    use bevy::prelude::*;
    use super::{despawn_screen, GameState};

    const FONT_PATH: &str = ".\\fonts\\Retro Gaming.ttf";

    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_state::<MenuState>()
                .add_systems(OnEnter(GameState::Menu), menu_setup)
                .add_systems(OnEnter(MenuState::Main), main_menu_setup)
                .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
                .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
                .add_systems(OnExit(MenuState::Settings), despawn_screen::<OnSettingsMenuScreen>);
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
    struct OnMainMenuScreen;

    #[derive(Component)]
    struct OnSettingsMenuScreen;

    #[derive(Resource)]
    struct UiFont(Handle<Font>);

    fn menu_setup(
        mut menu_state: ResMut<NextState<MenuState>>,
        mut commands: Commands,
        assets_server: Res<AssetServer>
    ) {
        menu_state.set(MenuState::Main);
        let ui_font_handle: Handle<Font> = assets_server.load(FONT_PATH);
        commands.insert_resource(UiFont(ui_font_handle));
    }

    fn main_menu_setup(
        mut commands: Commands, 
        asset_server: Res<AssetServer>,
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
            OnMainMenuScreen,
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
                parent.spawn(
                    TextBundle::from_section(
                        "Continue", 
                        TextStyle {
                            font: ui_font.0.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..Default::default()
                        }
                    )
                );
                parent.spawn(
                    TextBundle::from_section(
                        "Settings", 
                        TextStyle {
                            font: ui_font.0.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..Default::default()
                        }
                    )
                );
                parent.spawn(
                    TextBundle::from_section(
                        "Exit", 
                        TextStyle {
                            font: ui_font.0.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..Default::default()
                        }
                    )
                );
            });
        });
    }

    fn settings_menu_setup() {

    }
}