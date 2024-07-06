use std::f32::consts::PI;
use rand::prelude::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{game::AnimationTimer, player::{AnimationIndices, ControllablePlayer, Facing, SpriteFacing}, spritesheet::{get_enemy_sprite_animation_states, get_sprite_atlas_layout, get_sprite_texture_handle, SpriteCollection, TextureAtlases, CHORT_IDLE, CHORT_RUN, LIZARD_M_HIT}, AppState, FONT_PATH, SCALE,};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
        app.add_systems(Update, (animate_sprite, enemy_movement, test_damage_number, update_damage_numbers).run_if(in_state(AppState::InGame)));
    }
}

pub struct AnimatedSprite {
    pub texture: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
    pub animation_states: Vec<(EnemyAnimationStates, Vec<usize>)>,
}

#[derive(Debug, PartialEq)]
pub enum EnemyAnimationStates {
    IDLE,
    RUNNING,
    HIT,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemySpriteAnimationStates {
    pub current_state: EnemyAnimationStates,
    pub available_states:  Vec<(EnemyAnimationStates, Vec<usize>)>,
    pub changed: bool,
}

#[derive(Component)]
struct DamageNumbers {
    value: i32,
    direction: Vec3,
    timer: Timer,
}

impl DamageNumbers {
    fn new(value: i32) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let direction = Vec3::new(angle.cos(), angle.sin(), 0.0);

        return Self {
            value,
            direction,
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

fn spawn_damage_number(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    value: i32,
    position: Vec3,
) {
    println!("Spawned damage number");
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                value.to_string(),
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 30.0,
                    color: Color::WHITE.into(),
                },
            ),
            transform: Transform::from_translation(position),
            ..Default::default()
        }, 
        DamageNumbers::new(value),
    ));
}

fn update_damage_numbers(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Text, &mut DamageNumbers)>,
){
    for (entity, mut transform, mut text, mut damagenumber) in &mut query {
        damagenumber.timer.tick(time.delta());

        if damagenumber.timer.finished() {
            commands.entity(entity).despawn();
        } else {
            transform.translation += damagenumber.direction * time.delta_seconds() * 65.0;
            let alpha = 1.0 - damagenumber.timer.fraction();
            text.sections[0].style.color.set_a(alpha);
        }
    }
}

fn test_damage_number(
    commands: Commands,
    asset_server: ResMut<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyI) {
        let value = 10;
        let position = Vec3::new(1500.0, 1500.0, 7.0);
    
        spawn_damage_number(commands, asset_server, value, position)
    }
}

fn setup(
    mut commands: Commands,
    texture_atlas: Res<TextureAtlases>,
    sprite_collection: Res<SpriteCollection>,
) {
    let requested_idle_sprite = CHORT_IDLE.to_owned();
    let requested_running_sprite = CHORT_RUN.to_owned();
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
        get_enemy_sprite_animation_states(
            EnemyAnimationStates::IDLE,
            requested_idle_sprite,
            &sprite_collection
        )
    );

    animated_sprite.animation_states.push(
        get_enemy_sprite_animation_states(
            EnemyAnimationStates::RUNNING,
            requested_running_sprite,
            &sprite_collection
        )
    );

    animated_sprite.animation_states.push(
        get_enemy_sprite_animation_states(
            EnemyAnimationStates::HIT,
            requested_hit_sprite,
            &sprite_collection
        )
    );

    let mut default_sprite_index_first: usize = 0;
    let mut default_sprite_index_last: usize = 0;
    for (animation_state, indices) in &animated_sprite.animation_states {
        if *animation_state == EnemyAnimationStates::IDLE {
            default_sprite_index_first = indices[0];
            default_sprite_index_last = *indices.last().unwrap();
        }
    }

    let animation_indices = AnimationIndices {
        first: default_sprite_index_first,
        last: default_sprite_index_last
    };

    let sprite_animation_states = EnemySpriteAnimationStates {
        current_state: EnemyAnimationStates::IDLE,
        available_states: animated_sprite.animation_states,
        changed: false,
    };

    let bot_entity = commands.spawn((
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
                translation: Vec3 { x: 1450.0, y: 1450.0, z: 5.0 },
                rotation: Quat::default(),
                scale: Vec3 { x: SCALE/1.2, y: SCALE/1.2, z: 1.0 },
            },
            ..Default::default()
        },
        SpriteFacing { facing: Facing::RIGHT },
        sprite_animation_states,
        animation_indices.clone(),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Name::new("Bot"),
        Enemy{},
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        CollisionGroups::new(Group::from_bits(0b01).unwrap(), Group::from_bits(0b01).unwrap()),
        ColliderMassProperties::Density(2.0),
        Damping {
            linear_damping: 2.0,
            ..Default::default()
        },
    )).id();

    commands.entity(bot_entity).with_children(|parent| {
        parent.spawn((
            TransformBundle::from(Transform { translation: Vec3::new(0.0, -4.0, 0.0), ..Default::default()}),
            Collider::cuboid(10.0, 10.0),
            ActiveEvents::COLLISION_EVENTS,
        ));
    });

    
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut TextureAtlas), With<Enemy>>,
    mut animation_state_query: Query<&mut EnemySpriteAnimationStates, With<Enemy>>,
) {
    let mut current_animation_state = animation_state_query.single_mut();

    if current_animation_state.changed {
        let mut index_first: usize = 0;
        let mut index_last: usize = 0;
        for state in &current_animation_state.available_states {
            if state.0 == current_animation_state.current_state {
                index_first = *state.1.first().unwrap();
                index_last = *state.1.last().unwrap();
            }
        }
        for (mut indices, _timer, mut sprite) in &mut query {
            indices.first = index_first;
            indices.last = index_last;
            sprite.index = index_first;
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

fn enemy_movement(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &mut EnemySpriteAnimationStates, &mut Sprite), With<Enemy>>,
    player_query: Query<&Transform, (With<ControllablePlayer>, Without<Enemy>)>,
) {
    let player_transform = player_query.get_single().unwrap();
    let player_position = player_transform.translation;

    for (mut transform, mut state, mut sprite) in &mut enemy_query {
        let vector = player_position - transform.translation;
        let angle = vector.y.atan2(vector.x);
        let direction = player_position - transform.translation;
        let distance = direction.length();

        let radius = 200.0;
        if radius > distance {
            if state.current_state == EnemyAnimationStates::IDLE {
                state.changed = true;
            }

            if state.changed == true {
                state.current_state = EnemyAnimationStates::RUNNING;
            }

            if angle <= 0.0 && angle >= -PI/2.0 {
                sprite.flip_x = false;
            }
            if angle <= -PI/2.0 && angle >= -PI {
                sprite.flip_x = true;
            }
            if angle > 0.0 && angle <= PI/2.0 {
                sprite.flip_x = false;
            }
            if angle > PI/2.0 && angle <= PI {
                sprite.flip_x = true;
            }
            let movement = direction.normalize() * time.delta_seconds() * 80.0;
            transform.translation += movement;
        } 
        else if state.current_state != EnemyAnimationStates::IDLE {
            state.current_state = EnemyAnimationStates::IDLE;
            state.changed = true;
        }
    }
}