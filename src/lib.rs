pub mod debug;
pub mod tilemap;
pub mod mainmenu;
pub mod splashscreen;
pub mod level;
pub mod magic;
pub mod input;
pub mod cursor;
pub mod melee;
pub mod spritesheet;
pub mod game;
pub mod network;
pub mod player;
pub mod healthbar;
pub mod enemy;
pub mod chest;
pub mod inventory;

use std::time::Duration;

use bevy::{
    ecs::{component::Component, entity::Entity, event::Event, schedule::States, system::Resource},
    math::Vec3, render::color::Color
};
use bevy_renet::renet::{
    ChannelConfig, ClientId, ConnectionConfig, SendType
};
use serde::{Deserialize, Serialize};

pub const PROTOCOL_ID: u64 = 7;

const SWORD_SPRITE_PATH: &str = ".\\sprites\\sword_anim.png";
const PLAYER_SPRITE_PATH: &str = ".\\sprites\\vampire_v1_1_animated.png";
const FONT_PATH: &str = ".\\fonts\\Retro Gaming.ttf";
const PLAYER_SPEED: f32 = 500.0;
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const SWORD_EQUIPED_SPRITE_PATH: &str = ".\\sprites\\sword.png";
const SCALE: f32 = 5.0;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    SplashScreen,
    MainMenu,
    LoadingScreen,
    InGame,
    PauseScreen,
    AssetLoading,
}

#[derive(Resource, Default, Debug)]
pub struct CursorWorldCoordinates(pub Vec3);

#[derive(Component, Default)]
pub struct PlayerCamera;

#[derive(Debug, Component)]
pub struct Player {
    pub id: ClientId
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerPosition {
    pub transform: Vec3,
}

#[derive(Debug, Default, Component)]
pub struct Velocity(pub Vec3);

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NetworkedEntities {
    pub entities: Vec<Entity>,
    pub translation: Vec<[f32; 3]>,
}

#[derive(Debug, Serialize, Deserialize, Component, Event)]
pub enum ClientChannel {
    Input,
    Command,
    Position,
}
pub enum ServerChannel {
    ServerMessages,
    NetworkedEntities,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerCreate {
        entity: Entity,
        id: ClientId,
        translation: [f32; 3],
    },
    PlayerRemove {
        id: ClientId
    },
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Command => 0,
            ClientChannel::Input => 1,
            ClientChannel::Position => 3,
        }
    }
}

impl ClientChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: Self::Input.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                }
            },
            ChannelConfig {
                channel_id: Self::Command.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered { 
                    resend_time: Duration::ZERO 
                }
            },
            ChannelConfig {
                channel_id: Self::Position.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered { 
                    resend_time: Duration::ZERO 
                }
            }
        ]
    }
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::NetworkedEntities => 0,
            ServerChannel::ServerMessages => 1,
        }
    }
}

impl ServerChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: Self::NetworkedEntities.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::Unreliable,
            },
            ChannelConfig {
                channel_id: Self::ServerMessages.into(),
                max_memory_usage_bytes: 10 * 1024 *1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(200)
                }
            }
        ]
    }
}

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: ClientChannel::channels_config(),
        server_channels_config: ServerChannel::channels_config()
    }
}
