use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_game_client::{connection_config, ClientChannel, NetworkedEntities, Player, PlayerInput, PlayerPosition, ServerChannel, ServerMessages, Velocity, PROTOCOL_ID};
use bevy_renet::{renet::{transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig}, ClientId, RenetServer, ServerEvent}, transport::NetcodeServerPlugin, RenetServerPlugin};

const PLAYER_SPEED: f32 = 300.0;

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<ClientId, Entity>
}

#[derive(Debug, Component)]
struct Bot;

#[derive(Default, Resource)]
struct PlayerSpriteAtlas {
    handle: Handle<TextureAtlas>,
}

#[derive(Debug, Resource)]

struct BotId(u64);

#[derive(Component, Clone)]
struct AnimationIndices {
    first: usize,
    last: usize
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);


fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RenetServerPlugin);

    app.insert_resource(ServerLobby::default());
    app.insert_resource(BotId(0));
    app.insert_resource(PlayerSpriteAtlas::default());

    initialise_renet_transport_server(&mut app);

    // app.add_systems(Startup, setup);
    // app.add_systems(Update, animate_sprite);

    app.add_systems(Update, (server_update_system, server_network_sync, move_players_system, spawn_bot, apply_position_system));

    app.add_systems(FixedUpdate, apply_velocity_system);

    app.run();
}

fn initialise_renet_transport_server(app: &mut App) {
    let server = RenetServer::new(connection_config());
    let public_address = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_address).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_address],
        authentication: ServerAuthentication::Unsecure
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

    app.add_plugins(NetcodeServerPlugin);

    app.insert_resource(server)
        .insert_resource(transport);

    println!("Successfully initialised Renet Server.");
}

#[allow(clippy::too_many_arguments)]
fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    players: Query<(Entity, &Player, &Transform)>,
    player_sprite: ResMut<PlayerSpriteAtlas>
) {
    let animation_indices = AnimationIndices {first: 0, last: 3};

    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);
                for (entity, player, transform) in players.iter() {
                    let translation: [f32;3] = transform.translation.into();
                    let message = bincode::serialize(&ServerMessages::PlayerCreate { 
                        entity: entity, 
                        id: player.id, 
                        translation: translation 
                    })
                    .unwrap();
                    server.send_message(*client_id, ServerChannel::ServerMessages, message);
                }

                let transform = Transform::from_xyz((fastrand::f32() - 0.5) * 40.0, (fastrand::f32() - 0.5) * 40.0, 0.0); 
                let player_entity = commands.spawn(SpriteSheetBundle {
                    texture_atlas: player_sprite.handle.clone(),
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform: Transform {
                        translation: Vec3::ZERO,
                        rotation: Quat::default(),
                        scale: Vec3 { x: 6.0, y: 6.0, z: 6.0 }
                    },
                    ..Default::default()
                },)
                    .insert(PlayerInput::default())
                    //.insert(Velocity::default())
                    .insert(PlayerPosition::default())
                    .insert(Player {id: *client_id})
                    .id();
                println!("generated player entity");

                lobby.players.insert(*client_id, player_entity);
                
                let translation: [f32; 3] = transform.translation.into();
                let message = bincode::serialize(&ServerMessages::PlayerCreate { 
                    entity: player_entity, 
                    id: *client_id, 
                    translation: translation, 
                }).unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
                },
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected. Reason: {}", client_id, reason);
                if let Some(player_entity) = lobby.players.remove(client_id) {
                    commands.entity(player_entity).despawn();
                }
                let message = bincode::serialize(&ServerMessages::PlayerRemove { id: *client_id }).unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
        }
    }
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Position) {
            let player_position: PlayerPosition = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                commands.entity(*player_entity).insert(player_position);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn server_network_sync(mut server: ResMut<RenetServer>, query: Query<(Entity, &Transform), With<Player>>) {
    let mut networked_entities = NetworkedEntities::default();
    for (entity, transform) in query.iter() {
        networked_entities.entities.push(entity);
        networked_entities.translation.push(transform.translation.into());
    }
    let sync_message = bincode::serialize(&networked_entities).unwrap();
    server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
}

fn move_players_system(mut query: Query<(&mut Velocity, &PlayerInput)>) {
    for (mut velocity, input) in query.iter_mut() {
        let x = (input.right as i8 - input.left as i8) as f32;
        let y = (input.up as i8 - input.down as i8) as f32;
        let direction = Vec2::new(x, y).normalize_or_zero();
        velocity.0.x = direction.x * PLAYER_SPEED;
        velocity.0.y = direction.y * PLAYER_SPEED;
    }
}

fn apply_velocity_system(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn apply_position_system(mut query: Query<(&PlayerPosition, &mut Transform)>) {
    for (player_position, mut player_transform) in query.iter_mut() {
        player_transform.translation = player_position.transform;
    }
}

fn spawn_bot(
    keyboard_input: Res<Input<KeyCode>>,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    mut bot_id: ResMut<BotId>,
    mut commands: Commands,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let client_id = ClientId::from_raw(bot_id.0);
        bot_id.0 += 1;
        // Spawn new player
        let transform = Transform::from_xyz((fastrand::f32() - 0.5) * 40., 0.51, (fastrand::f32() - 0.5) * 40.);
        let player_entity = commands
            .spawn(Player { id: client_id })
            .id();

        lobby.players.insert(client_id, player_entity);

        let translation: [f32; 3] = transform.translation.into();
        let message = bincode::serialize(&ServerMessages::PlayerCreate {
            id: client_id,
            entity: player_entity,
            translation,
        })
        .unwrap();
        server.broadcast_message(ServerChannel::ServerMessages, message);
    }
}