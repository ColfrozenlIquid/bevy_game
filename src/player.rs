use std::default;

use bevy::{prelude::*, sprite};
use crate::{game::AnimationTimer, spritesheet::*, AppState, PlayerPosition, SCALE};

pub struct PlayerPlugin;

#[derive(Debug, PartialEq)]
pub enum PlayerAnimationStates {
    IDLE,
    RUNNING,
    HIT,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(PartialEq)]
pub enum Facing {
    LEFT,
    RIGHT,
}

#[derive(Component, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize
}

#[derive(Component)]
pub struct SpriteFacing {
    pub facing: Facing
}

#[derive(Component)]
pub struct PlayerLabel;

#[derive(Default, Resource)]
pub struct PlayerSpriteAtlas {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
pub struct SpriteAnimationStates {
    pub current_state: PlayerAnimationStates,
    pub available_states:  Vec<(PlayerAnimationStates, Vec<usize>)>,
    pub changed: bool,
}

#[derive(Component)]
pub struct ControllablePlayer;

pub struct AnimatedSprite {
    texture: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
    animation_states: Vec<(PlayerAnimationStates, Vec<usize>)>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
        app.add_systems(Update, (animate_sprite, update_sprite_facing, debug_player_velocity, move_sprite).run_if(in_state(AppState::InGame)));
    }
}

fn setup(
    mut commands: Commands,
    texture_atlas: Res<TextureAtlases>,
    sprite_collection: Res<SpriteCollection>,
) {
    let requested_idle_sprite = KNIGHT_M_IDLE.to_owned();
    let requested_running_sprite = KNIGHT_M_RUN.to_owned();
    let requested_hit_sprite = KNIGHT_M_HIT.to_owned();

    let animated_sprite_texture = get_sprite_texture_handle(
        requested_idle_sprite.clone(), 
        &texture_atlas, 
        &sprite_collection
    ).expect("Could not find sprite texture handle");

    let animated_sprite_atlas_layout = get_sprite_atlas_layout(
        requested_idle_sprite.clone(), 
        &texture_atlas, 
        &sprite_collection
    ).expect("Could not find sprite texture atlas layout");


    let mut animated_sprite = AnimatedSprite {
        texture: animated_sprite_texture,
        atlas_layout: animated_sprite_atlas_layout,
        animation_states: vec![],
    };


    animated_sprite.animation_states.push(
        get_sprite_animation_states(
            PlayerAnimationStates::IDLE,
            requested_idle_sprite,
            &sprite_collection
        )
    );

    animated_sprite.animation_states.push(
        get_sprite_animation_states(
            PlayerAnimationStates::RUNNING,
            requested_running_sprite,
            &sprite_collection
        )
    );

    animated_sprite.animation_states.push(
        get_sprite_animation_states(
            PlayerAnimationStates::HIT,
            requested_hit_sprite,
            &sprite_collection
        )
    );

    let mut default_sprite_index_first: usize = 0;
    let mut default_sprite_index_last: usize = 0;
    for (animation_state, indices) in &animated_sprite.animation_states {
        if *animation_state == PlayerAnimationStates::IDLE {
            default_sprite_index_first = indices[0];
            default_sprite_index_last = *indices.last().unwrap();
        }
    }

    let animation_indices = AnimationIndices {
        first: default_sprite_index_first,
        last: default_sprite_index_last
    };

    let sprite_animation_states = SpriteAnimationStates {
        current_state: PlayerAnimationStates::IDLE,
        available_states: animated_sprite.animation_states,
        changed: false,
    };

    {
        let mut player_entity = commands.spawn((
            SpriteSheetBundle {
                sprite: Sprite {
                    flip_x: false,
                    ..Default::default()
                },
                texture: animated_sprite.texture,
                atlas: TextureAtlas {
                    layout: animated_sprite.atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform {
                    translation: Vec3 { x: 0.0, y: 0.0, z: 1.0 },
                    rotation: Quat::default(),
                    scale: Vec3 { x: SCALE/1.5, y: SCALE/1.5, z: SCALE/1.5 },
                },

                ..Default::default()
            },
            SpriteFacing { facing: Facing::RIGHT },
            animation_indices.clone(),
            PlayerPosition::default(),
            Velocity(Vec2::ZERO),
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
        player_entity.insert(ControllablePlayer);
        player_entity.insert(sprite_animation_states);
        // player_entity.insert(animation_state);
    }

}

fn show_player_animation_state(
    mut animation_state_query: Query<&SpriteAnimationStates, With<ControllablePlayer>>,
) {
    for state in &mut animation_state_query {
        println!("Current animation state: {:?}", state.current_state);
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
    mut animation_state_query: Query<&mut SpriteAnimationStates, With<ControllablePlayer>>,
) {
    let mut current_animation_state = animation_state_query.single_mut();

    if (current_animation_state.changed) {
        let mut index_first: usize = 0;
        let mut index_last: usize = 0;
        for state in &current_animation_state.available_states {
            if state.0 == current_animation_state.current_state {
                index_first = *state.1.first().unwrap();
                index_last = *state.1.last().unwrap();
                // println!("Assigning sprite indices to {} and {}", index_first, index_last);
            }
        }
        for (mut indices, timer, mut sprite) in &mut query {
            indices.first = index_first;
            indices.last = index_last;
            sprite.index = index_first;
            // println!("Sprite indices are: {} and {}", indices.first, indices.last);
        }
        current_animation_state.changed = false;
    }

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

// fn animate_sprite(
//     time: Res<Time>,
//     mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
// ) {
//     for (indices, mut timer, mut sprite) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             sprite.index = if sprite.index == indices.last {
//                 indices.first
//             } else {
//                 sprite.index + 1
//             };
//         }
//     }
// }

fn update_sprite_facing(
    mut sprite_query: Query<(&mut Sprite, &SpriteFacing), With<ControllablePlayer>>,
) {
    let (mut sprite, facing) = sprite_query.get_single_mut().unwrap();
    if facing.facing == Facing::LEFT {
        sprite.flip_x = true;
    }
    if facing.facing == Facing::RIGHT {
        sprite.flip_x = false;
    }
}

fn move_sprite(
    time: Res<Time>,
    mut sprite_query: Query<(&Velocity, &mut Transform, &PlayerPosition), With<ControllablePlayer>>,
) {
    let (velocity, mut transform, player_position) = sprite_query.single_mut();
    // println!("Current player transform is: {:?}", transform.translation);

            transform.translation.x = player_position.transform.x * time.delta_seconds();
            transform.translation.y = player_position.transform.y * time.delta_seconds();
            // println!("Transform after translation is: {:?}", transform.translation);
}

fn debug_player_velocity(
    player_velocity_query: Query<&Velocity, With<ControllablePlayer>>
) {
    let player_velocity = player_velocity_query.get_single().unwrap();
    // println!("Player velocity is: {:?}", player_velocity.0);
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