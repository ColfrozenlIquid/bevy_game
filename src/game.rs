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
            move_player,
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

// #[derive(Resource, Default)]
// struct SpriteFolder(Handle<LoadedFolder>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player_sprite: ResMut<PlayerSpriteAtlas>,
    mut sword_sprite: ResMut<SwordSpriteAtlas>,
    mut equipment_sprite: ResMut<EquipmentSpriteAtlas>,
    texture_atlas: Res<TextureAtlases>,
    sprite_collection: Res<SpriteCollection>,
    // client_id : ResMut<CurrentClientId>,
) {
    // println!("Setting up client");
    // {
    //     let texture = asset_server.load(PLAYER_SPRITE_PATH);
    //     let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 1, None, None);
    //     let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
    //     player_sprite.image = texture;
    //     player_sprite.layout = texture_atlas_layout_handle;
    // }

    // commands.spawn((Text2dBundle {
    //         text: Text::from_section(
    //             client_id.0.to_string(), 
    //         TextStyle {
    //             font: asset_server.load(FONT_PATH),
    //             font_size : 20.0,
    //             ..default()
    //         },
    //         ),
    //         ..Default::default()
    //     },
    //         PlayerLabel,
    //     ));
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

fn move_player(mut player_query: Query<(&mut Transform, &ControllablePlayer)>, player_position: Res<PlayerPosition>) {
    for mut player_query in player_query.iter_mut() {
        player_query.0.translation = player_position.transform;
    }
}

// fn spawn_character(
//     mut commands: Commands,
//     texture_atlas: Res<TextureAtlases>,
//     sprite_collection: Res<SpriteCollection>,
// ) {
//     println!("Gets here client");
//     let json_sprite = get_JSONSprite_by_type(ANGEL_IDLE, &sprite_collection);
//     // println!("Found JSONsprite: {:?}", json_sprite);
//     // let handle_vector = &texture_atlas.handles;
//     // for handle in handle_vector.iter() {
//     //     if handle.1.file_name == DWARF_F_IDLE {
//     //         commands.spawn(
//     //     (SpriteSheetBundle {
//     //                 texture_atlas: handle.0.clone(),
//     //                 sprite: TextureAtlasSprite::new(handle.1.),
//     //                 transform: Transform {
//     //                     translation: Vec3 { x: 0.0, y: 0.0, z: 5.0 },
//     //                     rotation: Quat::default(),
//     //                     scale: Vec3 { x: 1.5, y: 1.5, z: 1.0 }
//     //                 },
//     //                 ..Default::default()
//     //             },
//     //             Equipment,
//     //         ));
//     //     }
//     // }
// }