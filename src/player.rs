use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::dynamics::{RigidBodyForces, RigidBodyVelocity}};
use crate::{enemy::Enemy, game::{AnimationTimer, Equipment}, spritesheet::*, AppState, CursorWorldCoordinates, PlayerPosition, SCALE};

pub struct PlayerPlugin;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PlayerAnimationStates {
    IDLE,
    RUNNING,
    HIT,
}

#[derive(Component, Debug)]
pub struct Velocity(pub Vec2);

#[derive(PartialEq)]
pub enum Facing {
    LEFT,
    RIGHT,
}

#[derive(Component)]
pub struct PlayerColliding(pub bool);

#[derive(Component)]
pub struct PlayerMoving(bool);

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
pub struct PlayerSpriteAnimationStates {
    pub last_state: PlayerAnimationStates,
    pub current_state: PlayerAnimationStates,
    pub available_states:  Vec<(PlayerAnimationStates, Vec<usize>)>,
    pub changed: bool,
}

#[derive(Component)]
pub struct ControllablePlayer;

pub struct AnimatedSprite {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
    pub animation_states: Vec<(PlayerAnimationStates, Vec<usize>)>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
        app.add_systems(Update, (animate_sprite, update_sprite_facing, player_sprite_follow_mouse).run_if(in_state(AppState::InGame)));
        app.add_systems(Update, (update_system).run_if(in_state(AppState::InGame)));
    }
}


fn setup(
    mut commands: Commands,
    texture_atlas: Res<TextureAtlases>,
    sprite_collection: Res<SpriteCollection>,
) {
    let requested_idle_sprite = LIZARD_M_IDLE.to_owned();
    let requested_running_sprite = LIZARD_M_RUN.to_owned();
    let requested_hit_sprite = LIZARD_M_HIT.to_owned();

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
        get_player_sprite_animation_states(
            PlayerAnimationStates::IDLE,
            requested_idle_sprite,
            &sprite_collection
        )
    );

    animated_sprite.animation_states.push(
        get_player_sprite_animation_states(
            PlayerAnimationStates::RUNNING,
            requested_running_sprite,
            &sprite_collection
        )
    );

    animated_sprite.animation_states.push(
        get_player_sprite_animation_states(
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

    let sprite_animation_states = PlayerSpriteAnimationStates {
        last_state: PlayerAnimationStates::IDLE,
        current_state: PlayerAnimationStates::IDLE,
        available_states: animated_sprite.animation_states,
        changed: false,
    };

    let player_entity = commands.spawn((
        SpriteSheetBundle {
            sprite: Sprite {
                flip_x: false,
                ..Default::default()
            },
            texture: animated_sprite.texture.clone(),
            atlas: TextureAtlas {
                layout: animated_sprite.atlas_layout.clone(),
                index: animation_indices.first,
            },
            transform: Transform {
                translation: Vec3 { x: 1400.0, y: 1600.0, z: 5.0 },
                rotation: Quat::default(),
                scale: Vec3 { x: SCALE/1.2, y: SCALE/1.2, z: 1.0 },
            },

            ..Default::default()
        },
        SpriteFacing { facing: Facing::RIGHT },
        animation_indices.clone(),
        PlayerPosition::default(),
        PlayerMoving(false),
        Velocity(Vec2::ZERO),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ControllablePlayer,
        sprite_animation_states,
        RigidBody::KinematicPositionBased,
        Name::new("Player"),
        LockedAxes::ROTATION_LOCKED,
        PlayerColliding(false),
    )).id();

    commands.entity(player_entity).with_children(|parent| {
        parent.spawn((
            TransformBundle::from(Transform { translation: Vec3::new(0.0, -9.0, 0.0), ..Default::default()}),
            Collider::cuboid(8.0, 5.0),
            KinematicCharacterController::default(),
            ActiveEvents::COLLISION_EVENTS,
        ));
    });
}

fn register_collision_events(
    mut character_controller_output: Query<&mut KinematicCharacterControllerOutput>,
){
    for mut output in &mut character_controller_output {
        for collisions in &output.collisions {
            println!("Collision: {:?}", collisions);
        }
    }
}

fn debug_player_velocity(velocity_query: Query<&Velocity, With<ControllablePlayer>>) {
    for velocity in &velocity_query {
        println!("Velocity: {:?}", velocity);
    }
}

fn update_system(mut controllers: Query<&mut KinematicCharacterController>) {
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(Vec2::new(1.0, -0.5));
    }
}

fn show_player_animation_state(
    mut animation_state_query: Query<&PlayerSpriteAnimationStates, With<ControllablePlayer>>,
) {
    for state in &mut animation_state_query {
        println!("Current animation state: {:?}", state.current_state);
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut TextureAtlas), With<ControllablePlayer>>,
    mut animation_state_query: Query<&mut PlayerSpriteAnimationStates, With<ControllablePlayer>>,
) {
    let mut animation_state = animation_state_query.single_mut();
    
    if animation_state.current_state != animation_state.last_state {
        let mut index_first: usize = 0;
        let mut index_last: usize = 0;
        for state in &animation_state.available_states {
            if state.0 == animation_state.current_state {
                index_first = *state.1.first().unwrap();
                index_last = *state.1.last().unwrap();
            }
        }
        for (mut indices, timer, mut sprite) in &mut query {
            indices.first = index_first;
            indices.last = index_last;
            sprite.index = index_first;
        }
        animation_state.last_state = animation_state.current_state;
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

fn player_sprite_follow_mouse(
    mut player_moving_query: Query<(&PlayerMoving, &mut Sprite, &Transform), With<ControllablePlayer>>,
    cursor_coordinate: Res<CursorWorldCoordinates>,
) {
    for (player_moving, mut sprite, player_position) in &mut player_moving_query {
        if !player_moving.0 {
            if cursor_coordinate.0.x >= player_position.translation.x {
                sprite.flip_x = false;
            }
            if cursor_coordinate.0.x < player_position.translation.x {
                sprite.flip_x = true;
            }
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