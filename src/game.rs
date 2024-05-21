use bevy::prelude::*;

use crate::{network::CurrentClientId, player::{ControllablePlayer, PlayerSpriteAtlas}, spritesheet::{get_sprite, SpriteCollection, TextureAtlases, ANGEL_IDLE, KNIGHT_M_IDLE}, AppState, PlayerCamera, PlayerPosition, FONT_PATH, PLAYER_SPRITE_PATH, SCALE};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerPosition { transform: Vec3::new(0.0, 0.0, 1.0)})
        .insert_resource(PlayerSpriteAtlas::default())
        .insert_resource(SwordSpriteAtlas::default())
        .insert_resource(EquipmentSpriteAtlas::default());

        app.add_systems(Update, (
            camera_follow_player,
        ).run_if(in_state(AppState::InGame)));

        app.add_systems(Startup, setup_camera);

        app.add_systems(OnEnter(AppState::InGame), (setup, get_window));
    }
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(u32);

#[derive(Component)]
pub struct AnimateTranslation;

#[derive(Default, Resource)]
pub struct SwordSpriteAtlas {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Default, Resource)]
pub struct EquipmentSpriteAtlas {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
pub struct Sword {
    curent_index: usize,
}

#[derive(Component)]
pub struct Equipment;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connected;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


fn setup() {
    //game setup system
}

fn setup_camera(
    mut commands: Commands,
){
    commands.spawn((
        Camera2dBundle::default(), 
        PlayerCamera
    ));
}

fn get_window(window: Query<&Window>) {
    let window = window.single();
    let width = window.width();
    let height = window.height();
    dbg!(width, height);
}

fn camera_follow_player(
    player_query: Query<&Transform, With<ControllablePlayer>>,
    mut camera_query: Query<(&mut Transform, &PlayerCamera), Without<ControllablePlayer>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_tranform) = camera_query.get_single_mut() {
            camera_tranform.0.translation = Vec3::lerp(
                camera_tranform.0.translation, 
                player_transform.translation.extend(camera_tranform.0.translation.z).truncate(), 
                0.3
            );
        }
    }
}