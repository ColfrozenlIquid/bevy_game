use std::{net::UdpSocket, time::{SystemTime, UNIX_EPOCH}};
use bevy::{prelude::*, sprite::collide_aabb::collide, utils::HashMap};
use bevy_game_client::{connection_config, debug, mainmenu, splashscreen, tilemap, ClientChannel, GameState, NetworkedEntities, Player, PlayerInput, PlayerPosition, ServerChannel, ServerMessages, PROTOCOL_ID};
use bevy_renet::{client_connected, renet::{transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError}, ClientId, RenetClient}, transport::NetcodeClientPlugin, RenetClientPlugin};

use debug::DebugPlugin;
use tilemap::{TileMapPlugin, TileCollider};

const PLAYER_SPRITE_PATH: &str = ".\\sprites\\vampire_v1_1_animated.png";
const FONT_PATH: &str = ".\\fonts\\Retro Gaming.ttf";
const PLAYER_SPEED: f32 = 300.0;
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const GAME_CURSOR_SPRITE_PATH: &str = ".\\sprites\\cursor_200.png";

#[derive(Component, Default)]
struct PlayerCamera;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

#[derive(Component)]
struct ControllablePlayer;

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct PlayerLabel;

#[derive(Default, Resource)]
struct PlayerSpriteAtlas {
    handle: Handle<TextureAtlas>,
}

#[derive(Default, Resource)]
struct GameCursorAtlas {
    handle: Handle<TextureAtlas>
}

#[derive(Debug)]
struct PlayerInfo {
    client_entity: Entity,
    server_entity: Entity,
}

#[derive(Default, Resource)]
struct NetworkMapping(HashMap<Entity, Entity>);

#[derive(Debug, Default, Resource)]
struct ClientLobby {
    players: HashMap<ClientId, PlayerInfo>
}

#[derive(Debug, Resource)]
struct CurrentClientId(u64);

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Connected;

#[derive(Component, Clone)]
struct AnimationIndices {
    first: usize,
    last: usize
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct GameCursor;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RenetClientPlugin)
        .add_plugins(TileMapPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(mainmenu::menu::MenuPlugin)
        .add_plugins(splashscreen::splash::SplashPlugin)
        .add_state::<GameState>();

    initialise_renet_transport_client(&mut app);

    app.insert_resource(ClientLobby::default());
    app.insert_resource(PlayerInput::default());
    app.insert_resource(PlayerPosition::default());
    app.insert_resource(NetworkMapping::default());
    app.insert_resource(PlayerSpriteAtlas::default());
    app.insert_resource(GameCursorAtlas::default());

    app.add_systems(Startup, (setup, setup_game_cursor));
    app.add_systems(Startup, get_window);
    app.add_systems(Update, (
        move_cursor,
        keyboard_input_system,
        (client_send_input, client_sync_players, client_send_position).in_set(Connected),
        animate_sprite,
        move_player,
        label_movement,
        camera_follow_player
    ));

    app.run();
}

fn setup_game_cursor(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;

    commands.spawn(
        (ImageBundle {
            image: asset_server.load(GAME_CURSOR_SPRITE_PATH).into(),
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            z_index: ZIndex::Global(15),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        GameCursor
    ));
    println!("Spawned cursor sprite");
}

fn move_cursor(
    window: Query<&Window>,
    mut cursor: Query<&mut Style, With<GameCursor>>
) {
    let window: &Window = window.single();
    if let Some(position) = window.cursor_position() {
        let mut img_transform= cursor.single_mut();
        img_transform.left = Val::Px(position.x);
        img_transform.top = Val::Px(position.y);
    }
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
    app.configure_sets(Update, Connected.run_if(client_connected()));

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

fn wall_collision(
    player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            player_pos, 
            Vec2::splat(16.0 * 6.0 * 0.9), 
            wall_transform.translation, 
            Vec2::splat(16.0 * 6.0)
        );
        if collision.is_some() {
            return false;
        }
    }
    return true;
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>, 
    mut player_input: ResMut<PlayerInput>,
    mut player_position: ResMut<PlayerPosition>,
    mut sprite_query: Query<&mut TextureAtlasSprite>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>
) {
    let mut direction = Vec3::ZERO;
    let current_position = player_position.transform;


    if keyboard_input.pressed(KeyCode::A) {
        if wall_collision(current_position + Vec3::new(-2.0, 0.0, 0.0), &wall_query) {
            direction.x -= 1.0;
        }
    }

    if keyboard_input.pressed(KeyCode::D) {
        if wall_collision(current_position + Vec3::new(2.0, 0.0, 0.0), &wall_query) {
            direction.x += 1.0;
        }
    }

    if keyboard_input.pressed(KeyCode::W) {
        if wall_collision(current_position + Vec3::new(0.0, 2.0, 0.0), &wall_query) {
            direction.y += 1.0;
        }
    }

    if keyboard_input.pressed(KeyCode::S) {
        if wall_collision(current_position + Vec3::new(0.0, -2.0, 0.0), &wall_query) {
            direction.y -= 1.0;
        }
    }

    
    player_input.left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
    player_input.right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
    player_input.up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
    player_input.down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);

    for mut sprite in &mut sprite_query {
        if player_input.left {
            sprite.flip_x = true;
        } else if player_input.right {
            sprite.flip_x = false;
        }
    }
    
    player_position.transform += direction * 5.0;
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
                        texture_atlas: player_sprite.handle.clone(),
                        sprite: TextureAtlasSprite::new(animation_indices.first),
                        transform: Transform {
                            translation: Vec3 { x: translation[0], y: translation[1], z: translation[2] },
                            rotation: Quat::default(),
                            scale: Vec3 { x: 3.0, y: 3.0, z: 3.0 }
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
                    scale: Vec3 { x: 6.0, y: 6.0, z: 6.0 },
                    ..Default::default()
                };
                commands.entity(*entity).insert(transform);
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut player_sprite: ResMut<PlayerSpriteAtlas>,
    mut cursor_sprite: ResMut<GameCursorAtlas>,
    client_id : ResMut<CurrentClientId>,
) {
    let texture_handle = asset_server.load(PLAYER_SPRITE_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    player_sprite.handle = texture_atlas_handle.clone();
    
    commands.spawn((
        Camera2dBundle::default(), 
        PlayerCamera
    ));

    commands.spawn((Text2dBundle {
            text: Text::from_section(
                client_id.0.to_string(), 
            TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size : 20.0,
                ..default()
            },
            ).with_alignment(TextAlignment::Center),
            ..Default::default()
        },
            PlayerLabel,
        ));
}

fn animate_sprite(
    time: Res<Time>,
        mut query: Query<(
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite
        )>,
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
    player_query: Query<(&Transform, With<ControllablePlayer>)>,
    mut camera_query: Query<(&mut Transform, &PlayerCamera), Without<ControllablePlayer>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_tranform) = camera_query.get_single_mut() {
            camera_tranform.0.translation = Vec3::lerp(
                camera_tranform.0.translation, 
                player_transform.0.translation.extend(camera_tranform.0.translation.z).truncate(), 
                0.3
            );
        }
    }
}