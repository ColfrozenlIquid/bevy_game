use bevy::prelude::*;
use crate::{game::AnimationTimer, spritesheet::*, AppState, PlayerPosition, SCALE};

pub struct PlayerPlugin;

#[derive(Debug)]
pub enum PlayerAnimationStates {
    IDLE,
    RUNNING,
    HIT,
}

#[derive(Component)]
pub struct AnimationState {
    pub state: PlayerAnimationStates,
    pub sprite_index: Vec<(PlayerAnimationStates, Vec<usize>)>
}

#[derive(Component, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize
}

#[derive(Component)]
pub struct PlayerLabel;

#[derive(Default, Resource)]
pub struct PlayerSpriteAtlas {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
pub struct ControllablePlayer;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
        app.add_systems(Update, (animate_sprite, show_player_animation_state));
    }
}

fn setup(
    mut commands: Commands,
    texture_atlas: Res<TextureAtlases>,
    sprite_collection: Res<SpriteCollection>,
) {
    let animation_indices = AnimationIndices {first: 0, last: 3};
    let idle_sprite = get_sprite(KNIGHT_M_IDLE.to_owned(), &texture_atlas, &sprite_collection);
    let running_sprite = get_sprite(KNIGHT_M_RUN.to_owned(), &texture_atlas, &sprite_collection);
    let hit_sprite = get_sprite(KNIGHT_M_HIT.to_owned(), &texture_atlas, &sprite_collection);
    println!("Function worked");

    // let mut sprite_indices = Vec::<(PlayerAnimationStates, Vec<usize>)>::new();
    
    // {
    //     let mut indices = Vec::<usize>::new();
    //     for index in idle_sprite.1.frame_index {
    //         indices.push(index.1 as usize);
    //     }
    //     sprite_indices.push((PlayerAnimationStates::IDLE, indices));
    // }

    // {
    //     let mut indices = Vec::<usize>::new();
    //     for index in running_sprite.1.frame_index {
    //         indices.push(index.1 as usize);
    //     }
    //     sprite_indices.push((PlayerAnimationStates::RUNNING, indices));
    // }

    // {
    //     let mut indices = Vec::<usize>::new();
    //     for index in hit_sprite.1.frame_index {
    //         indices.push(index.1 as usize);
    //     }
    //     sprite_indices.push((PlayerAnimationStates::HIT, indices));
    // }

    // let animation_state = AnimationState { 
    //     state: PlayerAnimationStates::IDLE ,
    //     sprite_index: sprite_indices 
    // };

    {
        let animation_indices = AnimationIndices {
            first: idle_sprite.1.frame_index[0].1 as usize, 
            last: ((idle_sprite.1.frame_index[0].1) + (idle_sprite.1.frame_count - 1)) as usize};

        let mut player_entity = commands.spawn((
            SpriteSheetBundle {
                texture: idle_sprite.0.1.clone(),
                atlas: TextureAtlas {
                    layout: idle_sprite.0.0.clone(),
                    index: idle_sprite.1.frame_index[0].1 as usize,
                },
                transform: Transform {
                    translation: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
                    rotation: Quat::default(),
                    scale: Vec3 { x: SCALE/1.5, y: SCALE/1.5, z: SCALE/1.5 }
                },
                ..Default::default()
            },
            animation_indices.clone(),
            PlayerPosition::default(),
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
        player_entity.insert(ControllablePlayer);
        // player_entity.insert(animation_state);
    }
    // let mut client_entity = commands.spawn((
    //     SpriteSheetBundle {
    //         texture: player_sprite.image.clone(),
    //         atlas: TextureAtlas {
    //             layout: player_sprite.layout.clone(),
    //             index: animation_indices.first,
    //         },
    //         transform: Transform {
    //             translation: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
    //             rotation: Quat::default(),
    //             scale: Vec3 { x: SCALE, y: SCALE, z: SCALE }
    //         },
    //         ..Default::default()
    //     },
    //     // PlayerLabel,
    //     animation_indices.clone(),
    //     PlayerPosition::default(),
    //     PlayerAnimationState::default(),
    //     AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    // ));
    // client_entity.insert(ControllablePlayer);
}

fn show_player_animation_state(
    mut animation_state_query: Query<&AnimationState, With<ControllablePlayer>>,
) {
    for state in &mut animation_state_query {
        println!("Current animation state: {:?}", state.state)
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