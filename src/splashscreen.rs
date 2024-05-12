use bevy::prelude::*;
use crate::AppState;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub mod splash {
    use bevy::prelude::*;
    use super::{despawn_screen, AppState};

    const SPLASH_SCREEN_PATH: &str = "splashscreen/splash_screen.png";

    pub struct SplashPlugin;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_systems(OnEnter(AppState::SplashScreen), splash_setup)
                .add_systems(Update, countdown.run_if(in_state(AppState::SplashScreen)))
                .add_systems(OnExit(AppState::SplashScreen), despawn_screen::<OnSplashScreen>);
        }
    }

    #[derive(Component)]
    struct OnSplashScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load(SPLASH_SCREEN_PATH);
        commands.spawn((
            ImageBundle {
                style: Style {
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            },
            OnSplashScreen,
        ));
        commands.insert_resource(SplashTimer(Timer::from_seconds(0.5, TimerMode::Once)));
    }

    fn countdown(
        mut game_state: ResMut<NextState<AppState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(AppState::MainMenu);
        }
    }
}