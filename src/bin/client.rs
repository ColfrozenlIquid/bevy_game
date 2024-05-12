use bevy_game_client::{cursor::CursorPlugin, input::InputPlugin, mainmenu::menu::MenuPlugin, splashscreen::splash::SplashPlugin, CursorWorldCoordinates, PlayerCamera};
use std::{borrow::BorrowMut, f32::consts::PI, net::UdpSocket, time::{Duration, SystemTime, UNIX_EPOCH}};
use bevy::{asset::LoadedFolder, prelude::*, transform::commands, utils::HashMap};
use bevy_egui::egui::epaint::text::cursor;
use bevy_game_client::{connection_config, debug, magic::MagicPlugin, mainmenu, splashscreen, tilemap, ClientChannel, AppState, NetworkedEntities, Player, PlayerInput, PlayerPosition, ServerChannel, ServerMessages, PROTOCOL_ID};
// use bevy_asset_loader::prelude::*;

use bevy_renet::{client_connected, renet::{transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError}, ClientId, RenetClient}, transport::NetcodeClientPlugin, RenetClientPlugin};

use debug::DebugPlugin;

const SWORD_SPRITE_PATH: &str = ".\\sprites\\sword_anim.png";
const PLAYER_SPRITE_PATH: &str = ".\\sprites\\vampire_v1_1_animated.png";
const FONT_PATH: &str = ".\\fonts\\Retro Gaming.ttf";
const PLAYER_SPEED: f32 = 500.0;
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const SWORD_EQUIPED_SPRITE_PATH: &str = ".\\sprites\\sword.png";

const SCALE: f32 = 5.0;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(u32);

#[derive(Component)]
pub struct ControllablePlayer;

#[derive(Component)]
pub struct AnimateTranslation;

#[derive(Component)]
pub struct PlayerLabel;

#[derive(Default, Resource)]
pub struct PlayerSpriteAtlas {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

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

#[derive(Debug)]
pub struct PlayerInfo {
    client_entity: Entity,
    server_entity: Entity,
}

#[derive(Default, Resource)]
pub struct NetworkMapping(HashMap<Entity, Entity>);

#[derive(Debug, Default, Resource)]
pub struct ClientLobby {
    players: HashMap<ClientId, PlayerInfo>
}

#[derive(Debug, Resource)]
pub struct CurrentClientId(u64);

#[derive(Component)]
pub struct Sword {
    curent_index: usize,
}

#[derive(Component)]
pub struct Equipment;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connected;

#[derive(Component, Clone)]
pub struct AnimationIndices {
    first: usize,
    last: usize
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Resource, Default)]
struct SpriteFolder(Handle<LoadedFolder>);

#[derive(Resource, Debug, Default)]
struct CurrentState(AppState);

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RenetClientPlugin)
        //.add_plugins(TileMapPlugin)
        // .add_plugins(SpriteSheetPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(MagicPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(SplashPlugin)
        // .add_plugins(LdtkPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(InputPlugin)
        .init_state::<AppState>();

    // app.init_loading_state(
    //     LoadingState::new(AppState::AssetLoading)
    // );
    // LoadingState::new(AppState::AssetLoading);

    initialise_renet_transport_client(&mut app);

    app.insert_resource(ClientLobby::default())
        .insert_resource(PlayerInput::default())
        .insert_resource(PlayerPosition { transform: Vec3::new(0.0, 0.0, 1.0)})
        .insert_resource(NetworkMapping::default())
        .insert_resource(PlayerSpriteAtlas::default())
        .insert_resource(SwordSpriteAtlas::default())
        .insert_resource(EquipmentSpriteAtlas::default())
        .insert_resource(CurrentState::default());
        // .insert_resource(CursorWorldCoordinates::default())
        // .insert_resource(LevelSelection::index(0));

    app.add_systems(Startup, setup_camera);

    app.add_systems(OnEnter(AppState::InGame), (setup, get_window));
    app.add_systems(Update, (
        (client_send_input, client_sync_players, client_send_position).in_set(Connected),
        animate_sprite,
        move_player,
        debug_current_state,
        label_movement,
        camera_follow_player,
    ).run_if(in_state(AppState::InGame)));

    app.run();
}

fn debug_current_state(
    state: Res<State<AppState>>,
    mut current_state: ResMut<CurrentState>,
    keyboard_input: Res<ButtonInput<KeyCode>>, 
){
    // if keyboard_input.pressed(KeyCode::KeyI) {
    //     println!("Pressed I");
    //     eprintln!("Current State: {:?}", state.get());
    // }
    // if *state.get() != current_state.0 {
        eprintln!("Current State: {:?}", state.get());
        // current_state.0 = *state.get();
    // }
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut player_sprite: ResMut<PlayerSpriteAtlas>,
    mut sword_sprite: ResMut<SwordSpriteAtlas>,
    mut equipment_sprite: ResMut<EquipmentSpriteAtlas>,
    client_id : ResMut<CurrentClientId>,
) {
    let animation_indices = AnimationIndices {first: 0, last: 3};

    println!("Setting up client");

    {
        let texture = asset_server.load(PLAYER_SPRITE_PATH);
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 1, None, None);
        let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
        player_sprite.image = texture;
        player_sprite.layout = texture_atlas_layout_handle;
    }

    // commands.spawn(
    //     (SpriteSheetBundle {
    //         texture: player_sprite.image.clone(),
    //         atlas: TextureAtlas {
    //             layout: player_sprite.layout.clone(),
    //             index: animation_indices.first,
    //         },
    //         transform: Transform {
    //             translation: Vec3 { x: 0.0, y: 0.0, z: 12.0 },
    //             rotation: Quat::default(),
    //             scale: Vec3 { x: 1.5, y: 1.5, z: 1.0 }
    //         },
    //         ..Default::default()
    //     },
    //     // Equipment,
    // ));

    // commands.spawn(LdtkWorldBundle {
    //     ldtk_handle: asset_server.load(LEVEL_0_PATH),
    //     transform: Transform {
    //         translation: Vec3::new(-1000.0, -1200.0, -1.0),
    //         scale: Vec3::new(SCALE, SCALE, 1.0),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });

    // {
    //     let texture_handle = asset_server.load(SWORD_SPRITE_PATH);
    //     let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 1, None, None);
    //     let texture_atlas_handle = texture_atlases.add(texture_atlas);
    //     sword_sprite.handle = texture_atlas_handle.clone();
    // }

    // {
    //     let texture_handle = asset_server.load(SWORD_EQUIPED_SPRITE_PATH);
    //     let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    //     let texture_atlas_handle = texture_atlases.add(texture_atlas);
    //     equipment_sprite.handle = texture_atlas_handle.clone();
    // }

    // commands.spawn(
    //     (SpriteSheetBundle {
    //         texture_atlas: equipment_sprite.handle.clone(),
    //         sprite: TextureAtlasSprite::new(0),
    //         transform: Transform {
    //             translation: Vec3 { x: 0.0, y: 0.0, z: 5.0 },
    //             rotation: Quat::default(),
    //             scale: Vec3 { x: 1.5, y: 1.5, z: 1.0 }
    //         },
    //         ..Default::default()
    //     },
    //     Equipment,
    // ));

    commands.spawn((Text2dBundle {
            text: Text::from_section(
                client_id.0.to_string(), 
            TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size : 20.0,
                ..default()
            },
            ),
            ..Default::default()
        },
            PlayerLabel,
        ));
}

fn setup_camera(
    mut commands: Commands,
){
    commands.spawn((
        Camera2dBundle::default(), 
        PlayerCamera
    ));
}

fn initialise_renet_transport_client(app: &mut App) {
    let client = RenetClient::new(connection_config());
    let server_address = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;

    let authentication = ClientAuthentication::Unsecure { 
        protocol_id: PROTOCOL_ID, 
        client_id: client_id, 
        server_addr: server_address, 
        user_data: None, 
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    
    app.add_plugins(NetcodeClientPlugin);
    app.configure_sets(Update, Connected.run_if(client_connected));

    app.insert_resource(client)
        .insert_resource(transport)
        .insert_resource(CurrentClientId(client_id));

    #[allow(clippy::never_loop)]
    fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
        for e in renet_error.read() {
            panic!("{}", e);
        }
    }

    app.add_systems(Update, panic_on_error_system);

    println!("Successfully initialised Renet client.")
}

fn get_window(window: Query<&Window>) {
    let window = window.single();
    let width = window.width();
    let height = window.height();
    dbg!(width, height);
}

fn label_movement(
    mut set: ParamSet<(
        Query<(&Transform, &ControllablePlayer), Without<PlayerLabel>>,
        Query<&mut Transform, With<PlayerLabel>>
    )>,
) {
    let mut transform = Vec3::default();
    for player_transform in set.p0().iter() {
        transform = player_transform.0.translation;
    }
    if let Ok(mut label_transform) = set.p1().get_single_mut() {
        label_transform.translation = transform + Vec3::new(0.0, 80.0, 0.0);
    }
}

fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
    let input_message = bincode::serialize(&*player_input).unwrap();
    client.send_message(ClientChannel::Input, input_message)
}

fn client_send_position(player_position: Res<PlayerPosition>, mut client: ResMut<RenetClient>) {
    let position_message = bincode::serialize(&*player_position).unwrap();
    client.send_message(ClientChannel::Position, position_message);
}

fn move_player(mut player_query: Query<(&mut Transform, &ControllablePlayer)>, player_position: Res<PlayerPosition>) {
    for mut player_query in player_query.iter_mut() {
        player_query.0.translation = player_position.transform;
    }
}

fn client_sync_players(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    client_id : ResMut<CurrentClientId>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkMapping>,
    player_sprite: ResMut<PlayerSpriteAtlas>
) {
    let animation_indices = AnimationIndices {first: 0, last: 3};
    let client_id = client_id.0;
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerCreate { entity, id, translation } => {
                println!("Player {} connected.", id); 
                let mut client_entity = commands.spawn((
                    SpriteSheetBundle {
                        texture: player_sprite.image.clone(),
                        atlas: TextureAtlas {
                            layout: player_sprite.layout.clone(),
                            index: animation_indices.first,
                        },
                        transform: Transform {
                            translation: Vec3 { x: translation[0], y: translation[1], z: 1.0 },
                            rotation: Quat::default(),
                            scale: Vec3 { x: SCALE, y: SCALE, z: SCALE }
                        },
                        ..Default::default()
                    },
                    // PlayerLabel,
                    animation_indices.clone(),
                    PlayerPosition::default(),
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                ));

                if client_id == id.raw() {
                    client_entity.insert(ControllablePlayer);
                }

                let player_info = PlayerInfo {
                    server_entity: entity,
                    client_entity: client_entity.id(),
                };
                lobby.players.insert(id, player_info);
                network_mapping.0.insert(entity, client_entity.id());
            }
            ServerMessages::PlayerRemove { id } => {
                println!("Player {} disconnected.", id);
                if let Some( PlayerInfo { 
                    client_entity, 
                    server_entity 
                }) = lobby.players.remove(&id) {
                    commands.entity(client_entity).despawn();
                    network_mapping.0.remove(&server_entity);
                }
            }
        }
    }

    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();
        for i in 0..networked_entities.entities.len() {
            if let Some(entity) = network_mapping.0.get(&networked_entities.entities[i]) {
                let translation = networked_entities.translation[i].into();
                let transform = Transform {
                    translation: translation,
                    scale: Vec3 { x: SCALE, y: SCALE, z: 1.0 },
                    ..Default::default()
                };
                commands.entity(*entity).insert(transform);
            }
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
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