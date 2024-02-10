use std::{net::UdpSocket, time::SystemTime};

use bevy::prelude::*;
use bevy_renet::{renet::{transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig}, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent}, transport::NetcodeServerPlugin, RenetServerPlugin};

const SPRITE_PATH: &str = ".\\sprites\\vampire_v1_1_animated.png";


struct Character {
    x_pos: f32,
    y_pos: f32
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn main() {
    let mut app = App::new();
    initialise_renet_server(&mut app);
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        //.add_systems(Startup, setup)
        //.add_systems(Update, animate_sprite)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Startup, send_message_system)
        .add_systems(Startup, receive_message_system)
        .add_systems(Startup, handle_events_system)
        .run();
}

fn initialise_renet_server(app: &mut App) {
    let server = RenetServer::new(ConnectionConfig::default());
    let server_address = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_address).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_address],
        authentication: ServerAuthentication::Unsecure
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    app.add_plugins(RenetServerPlugin)
        .insert_resource(server)
        .add_plugins(NetcodeServerPlugin)
        .insert_resource(transport);
    println!("Successfully initialised Renet Server.");
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(SPRITE_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices {first: 0, last: 3};
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform {
                translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                rotation: Quat::default(),
                scale: Vec3 { x: 6.0, y: 6.0, z: 6.0 }
            },
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::D) {
        println!("Pressed D key");
    }
    if keyboard_input.pressed(KeyCode::A) {
        println!("Pressed A key");
    }
    if keyboard_input.pressed(KeyCode::W) {
        println!("Pressed W key");
    }
    if keyboard_input.pressed(KeyCode::S) {
        println!("Pressed S key");
    }
}

fn send_message_system(mut server: ResMut<RenetServer>) {
    let channel_id = 0;
    server.broadcast_message(DefaultChannel::ReliableOrdered, "server message");
}

fn receive_message_system(mut server: ResMut<RenetServer>) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
            println!("{:?}", message);
        }
    }
}

fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {} connected.", client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {} disconnected. Reason: {}.", client_id, reason);
            }
        }
    }
}