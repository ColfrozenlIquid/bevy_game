use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection};
use bevy_game_client::cursor::CursorPlugin;
use bevy_game_client::game::GamePlugin;
use bevy_game_client::input::InputPlugin;
use bevy_game_client::level::LevelPlugin;
use bevy_game_client::magic::MagicPlugin;
use bevy_game_client::mainmenu::menu::MenuPlugin;
use bevy_game_client::player::PlayerPlugin;
use bevy_game_client::splashscreen::splash::SplashPlugin;
use bevy_game_client::spritesheet::SpriteSheetPlugin;
use bevy_game_client::AppState;
use bevy_game_client::debug::DebugPlugin;
use bevy_rapier2d::plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

#[derive(Resource, Debug, Default)]
pub struct CurrentState(AppState);

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(DebugPlugin)
        .add_plugins(MagicPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(SplashPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(LdtkPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(PlayerPlugin)
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(NetworkPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(SpriteSheetPlugin)
        .init_state::<AppState>();

        let mut rapier_config = RapierConfiguration::new(100.0);
        rapier_config.gravity = Vec2::new(0.0, 0.0);
        app.insert_resource(rapier_config);

        app.insert_resource(CurrentState::default());

        app.add_systems(Update, debug_current_state);

    app.run();
}

fn debug_current_state(
    state: Res<State<AppState>>,
    mut current_state: ResMut<CurrentState>,
){
    if *state.get() != current_state.0 {
        eprintln!("Current State: {:?}", state.get());
        current_state.0 = *state.get();
    }
}