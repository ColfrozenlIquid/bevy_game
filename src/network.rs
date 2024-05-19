use std::{
    net::UdpSocket,
    time::{SystemTime, UNIX_EPOCH}
};
use bevy::{
    prelude::*, utils::HashMap
};
use bevy_renet::{
    client_connected, 
    renet::{transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError}, ClientId, RenetClient},
    transport::NetcodeClientPlugin, RenetClientPlugin
};
use crate::{
    connection_config, game::{AnimationTimer, Connected}, player::{AnimationIndices, ControllablePlayer, PlayerSpriteAtlas}, AppState, ClientChannel, NetworkedEntities, PlayerInput, PlayerPosition, ServerChannel, ServerMessages, PROTOCOL_ID, SCALE
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, mut app: &mut App) {
        app.insert_resource(NetworkMapping::default());
        app.insert_resource(ClientLobby::default());
        
        app.add_plugins(RenetClientPlugin);

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

        println!("Successfully initialised Renet client.");

        // app.add_systems(OnEnter(AppState::LoadingScreen), initialise_renet_transport_client);
        app.add_systems(
            Update,
            (panic_on_error_system.run_if(in_state(AppState::InGame)), 
            (client_send_input, client_sync_players, client_send_position).in_set(Connected))
        );
    }
}

#[derive(Debug, Resource)]
pub struct CurrentClientId(pub u64);

#[derive(Default, Resource)]
pub struct NetworkMapping(HashMap<Entity, Entity>);

#[derive(Debug, Default, Resource)]
pub struct ClientLobby {
    players: HashMap<ClientId, PlayerInfo>
}

#[derive(Debug)]
pub struct PlayerInfo {
    client_entity: Entity,
    server_entity: Entity,
}

// fn initialise_renet_transport_client(app: &mut App) {
//     let client = RenetClient::new(connection_config());
//     let server_address = "127.0.0.1:5000".parse().unwrap();
//     let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
//     let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
//     let client_id = current_time.as_millis() as u64;

//     let authentication = ClientAuthentication::Unsecure { 
//         protocol_id: PROTOCOL_ID, 
//         client_id: client_id, 
//         server_addr: server_address, 
//         user_data: None, 
//     };

//     let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    
//     app.add_plugins(NetcodeClientPlugin);
//     app.configure_sets(Update, Connected.run_if(client_connected));

//     app.insert_resource(client)
//         .insert_resource(transport)
//         .insert_resource(CurrentClientId(client_id));

//     #[allow(clippy::never_loop)]
//     fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
//         for e in renet_error.read() {
//             panic!("{}", e);
//         }
//     }

//     app.add_systems(Update, panic_on_error_system);

//     println!("Successfully initialised Renet client.")
// }

fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
    let input_message = bincode::serialize(&*player_input).unwrap();
    client.send_message(ClientChannel::Input, input_message)
}

fn client_send_position(player_position: Res<PlayerPosition>, mut client: ResMut<RenetClient>) {
    let position_message = bincode::serialize(&*player_position).unwrap();
    client.send_message(ClientChannel::Position, position_message);
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