use crate::{CursorWorldCoordinates, PlayerCamera};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;

const FIRE_BALL_SPRITE_PATH: &str = "./sprites/magic/FireBall_64x64.png";
const ICE_SPIKE_SPRITE_PATH: &str = "./sprites/magic/IcePick_64x64.png";
const FIRE_BURST_SPRITE_PATH: &str = "./sprites/magic/FireBurst_64x64.png";
const ICE_SPIKE_SHATTER_SPRITE_PATH: &str = "./sprites/magic/IceShatter_96x96.png";

const SCALE: f32 = 5.0;

pub struct MagicPlugin;

#[derive(Component, Clone)]
struct AnimationIndices {
    first: usize,
    last: usize
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct CastSpell {
    spell_type: Spells,
    _start_pos: Vec3,
    direction: Vec2,
    velocity: f32,
    collision_offset: f32,
}

#[derive(Resource, Default)]
pub struct SelectedSpell {
    pub spell: Spells,
}

#[derive(Default, PartialEq, Debug)]
pub enum Spells {
    #[default]
    FireBall,
    IceSpike,
}

#[derive(Component)]
struct FireBall {
    _last_index: u32,
}

#[derive(Component)]
struct IceSpike {
    _last_index: u32,
}

#[derive(Component)]
struct SpellFlightTime {
    timer: Timer,
}

#[derive(Resource, Default)]
pub struct SpellCoolDown{
    timer: Timer,
}

#[derive(Resource, Default, Clone)]
pub struct FireBurstSpriteAtlas {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default, Clone)]
struct IceSpikeShatterSpriteAtlas {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default, Clone)]
pub struct FireBallSpriteAtlas {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default, Clone)]
pub struct IceSpikeSpriteAtlas {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Debug, Clone, Event)]
pub struct EnemySpellCollisionEvent{
    spell_entity: Entity,
    enemy_entity: Entity,
}

impl Plugin for MagicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (
            despawn_spells,
            // spell_flight_system,
            receive_enemy_spell_collision_event,
            select_spell_system,
            enable_spell_cooldown,
            cursor_system,
            despawn_fireball_spell_collision,
            despawn_icespike_spell_collision,
            animate_sprite,
            enemy_spell_collision_event
        ));

        app.add_event::<EnemySpellCollisionEvent>();

        app.insert_resource(FireBallSpriteAtlas::default());
        app.insert_resource(FireBurstSpriteAtlas::default());
        app.insert_resource(IceSpikeSpriteAtlas::default());
        app.insert_resource(IceSpikeShatterSpriteAtlas::default());
        app.insert_resource(SelectedSpell::default());
        app.insert_resource(SpellCoolDown::default());
        app.insert_resource(CursorWorldCoordinates::default());
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut fireball_sprite_atlas: ResMut<FireBallSpriteAtlas>,
    mut icespike_sprite_atlas: ResMut<IceSpikeSpriteAtlas>,
    mut fireburst_sprite_atlas: ResMut<FireBurstSpriteAtlas>,
    mut icespikeshatter_sprite_atlas: ResMut<IceSpikeShatterSpriteAtlas>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {

    {
        let texture = asset_server.load(FIRE_BALL_SPRITE_PATH);
        let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), 45, 1, None, None);
        let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
        fireball_sprite_atlas.image = texture;
        fireball_sprite_atlas.layout = texture_atlas_layout_handle;
    }

    {
        let texture = asset_server.load(ICE_SPIKE_SPRITE_PATH);
        let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), 30, 1, None, None);
        let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
        icespike_sprite_atlas.image = texture;
        icespike_sprite_atlas.layout = texture_atlas_layout_handle;
    }

    {
        let texture = asset_server.load(FIRE_BURST_SPRITE_PATH);
        let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), 29, 1, None, None);
        let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
        fireburst_sprite_atlas.image = texture;
        fireburst_sprite_atlas.layout = texture_atlas_layout_handle;
    }

    {
        let texture = asset_server.load(ICE_SPIKE_SHATTER_SPRITE_PATH);
        let layout = TextureAtlasLayout::from_grid(Vec2::new(96.0, 96.0), 49, 1, None, None);
        let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
        icespikeshatter_sprite_atlas.image = texture;
        icespikeshatter_sprite_atlas.layout = texture_atlas_layout_handle;
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

fn cursor_system(
    mut cursor_coords: ResMut<CursorWorldCoordinates>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
){
    let (camera, camera_transform) = query_camera.single();
    let window = query_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin) {
            cursor_coords.0 = world_position;
        }
}

fn select_spell_system(
    mut selected_spell: ResMut<SelectedSpell>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Digit1) && selected_spell.spell != Spells::FireBall {
        println!("Switched to Fire Ball");
        selected_spell.spell = Spells::FireBall;
    }
    if keyboard_input.pressed(KeyCode::Digit2) && selected_spell.spell != Spells::IceSpike {
        println!("Switched to Ice Spike");
        selected_spell.spell = Spells::IceSpike;
    }
}

fn despawn_spells(
    mut commands: Commands,
    mut timer_query: Query<(Entity, &Transform, &mut SpellFlightTime, &CastSpell)>,
    fireburst_sprite: Res<FireBurstSpriteAtlas>,
    icespikeshatter_sprite: Res<IceSpikeShatterSpriteAtlas>,
    time: Res<Time>,
) {
    for (entity, &transform, mut spell_timer, cast_spell) in timer_query.iter_mut() {
        spell_timer.timer.tick(time.delta());

        if spell_timer.timer.finished() {
            let impact_position = transform.translation + (cast_spell.direction * SCALE/2.0 * cast_spell.collision_offset).extend(1.0);
            println!("Cast spell type attribute: {:?}", cast_spell.spell_type);
            if cast_spell.spell_type == Spells::FireBall {
                println!("despawn spell fireball");
                spawn_fireball_spell_collision(&mut commands, &fireburst_sprite, &transform, &impact_position);
                commands.entity(entity).despawn_recursive();
            }
            else if cast_spell.spell_type == Spells::IceSpike {
                println!("despawn spell icespike");
                spawn_icespike_spell_collision(&mut commands, &icespikeshatter_sprite, &transform, &impact_position);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn spawn_fireball_spell_collision(
    commands: &mut Commands,
    fireburst_sprite: &Res<FireBurstSpriteAtlas>,
    spell_transform: &Transform,
    spell_impact_position: &Vec3,
) {
    let animation_indices = AnimationIndices { first: 0, last: 28};
    let rotation_quat = Quat::from_rotation_z(PI/2.0);
    let sprite_quat = spell_transform.rotation * rotation_quat;

    commands.spawn((
        SpriteSheetBundle {
            texture: fireburst_sprite.image.clone(),
            atlas: TextureAtlas {
                layout: fireburst_sprite.layout.clone(),
                index: animation_indices.first,
            },
            transform: Transform {
                translation: Vec3::new(spell_impact_position.x, spell_impact_position.y, 5.0),
                scale: Vec3::new(-SCALE/2.0, SCALE/2.0, 1.0),
                rotation: sprite_quat,
                ..Default::default()
            },
            ..Default::default()
        },
        FireBall { _last_index: 28 },
        animation_indices.clone(),
        AnimationTimer(Timer::from_seconds(0.02, TimerMode::Repeating)),
    ));
}

fn spawn_icespike_spell_collision(
    commands: &mut Commands,
    fireburst_sprite: &Res<IceSpikeShatterSpriteAtlas>,
    spell_transform: &Transform,
    spell_impact_position: &Vec3,
) {
    let animation_indices = AnimationIndices { first: 0, last: 48};
    let rotation_quat = Quat::from_rotation_z(PI/2.0);
    let sprite_quat = spell_transform.rotation * rotation_quat;

    commands.spawn((
        SpriteSheetBundle {
            texture: fireburst_sprite.image.clone(),
            atlas: TextureAtlas {
                layout: fireburst_sprite.layout.clone(),
                index: animation_indices.first,
            },
            transform: Transform {
                translation: Vec3::new(spell_impact_position.x, spell_impact_position.y, 5.0),
                scale: Vec3::new(-SCALE/2.0, SCALE/2.0, 1.0),
                rotation: sprite_quat,
                ..Default::default()
            },
            ..Default::default()
        },
        IceSpike { _last_index: 48 },
        animation_indices.clone(),
        AnimationTimer(Timer::from_seconds(0.02, TimerMode::Repeating)),
    ));
}

fn despawn_fireball_spell_collision(
    mut commands: Commands,
    entity_query: Query<(Entity, &TextureAtlas), With<FireBall>>
) {
    for (entity, sprite) in entity_query.iter() {
        if sprite.index == 28 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_icespike_spell_collision(
    mut commands: Commands,
    entity_query: Query<(Entity, &TextureAtlas), With<IceSpike>>
) {
    for (entity, sprite) in entity_query.iter() {
        if sprite.index == 48 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_icespike_attack(
    commands: &mut Commands,
    magic_sprite: &Res<IceSpikeSpriteAtlas>,
    cursor_coord: &Res<CursorWorldCoordinates>,
    position_player: &Vec3,
) {
    let animation_indices = AnimationIndices { first: 0, last: 44};
    let sprite_head_offset = 29.0;

    let direction_vector_normalized = (cursor_coord.0.truncate() - position_player.truncate()).normalize();
    let sprite_spawn_position = position_player.truncate() + (direction_vector_normalized * 65.0);
    let angle = direction_vector_normalized.angle_between(Vec2 { x: 1.0, y: 0.0 });

    let spell_entity = commands.spawn((
        SpriteSheetBundle {
            texture: magic_sprite.image.clone(),
            atlas: TextureAtlas {
                layout: magic_sprite.layout.clone(),
                index: animation_indices.first,
            },
            transform: Transform {
                translation: Vec3::new(sprite_spawn_position.x, sprite_spawn_position.y, 1.0),
                scale: Vec3::new(-SCALE/2.0, SCALE/2.0, 1.0),
                rotation: Quat::from_rotation_z(-angle + PI),
                ..Default::default()
            },
            ..Default::default()
        },
        CastSpell {
            spell_type: Spells::IceSpike,
            _start_pos: sprite_spawn_position.extend(1.0),
            direction: direction_vector_normalized,
            velocity: 600.0,
            collision_offset: sprite_head_offset
        },
        SpellFlightTime {timer: Timer::new(Duration::from_secs(1), TimerMode::Once)},
        animation_indices.clone(),
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        ActiveEvents::COLLISION_EVENTS,
        Name::new("Ice Spike"),
    )).id();

    commands.entity(spell_entity).with_children(|parent| {
        parent.spawn((
            TransformBundle::from(Transform { translation: Vec3::new(26.0, 0.0, 0.0), ..Default::default()}),
            RigidBody::Dynamic,
            Collider::cuboid(5.0, 5.0),
        )).insert(Velocity {
            linvel: direction_vector_normalized,
            angvel: 0.4
        });
    });

}

pub fn spawn_fireball_attack(
    commands: &mut Commands,
    magic_sprite: &Res<FireBallSpriteAtlas>,
    cursor_coord: &Res<CursorWorldCoordinates>,
    position_player: &Vec3,
) {
    let animation_indices = AnimationIndices { first: 0, last: 44};
    let sprite_head_offset = 18.0 * 2.0;

    let direction_vector_normalized = (cursor_coord.0.truncate() - position_player.truncate()).normalize();
    let sprite_spawn_position = position_player.truncate() + (direction_vector_normalized * 65.0);
    let angle = direction_vector_normalized.angle_between(Vec2 { x: 1.0, y: 0.0 });

    let mut spell_entity = commands.spawn((
        SpriteSheetBundle {
            texture: magic_sprite.image.clone(),
            atlas: TextureAtlas {
                layout: magic_sprite.layout.clone(),
                index: animation_indices.first,
            },
            transform: Transform {
                translation: Vec3::new(sprite_spawn_position.x, sprite_spawn_position.y, 5.0),
                scale: Vec3::new(-SCALE/2.0, SCALE/2.0, 1.0),
                rotation: Quat::from_rotation_z(-angle + PI),
                ..Default::default()
            },
            ..Default::default()
        },
        CastSpell { 
            spell_type: Spells::FireBall,
            _start_pos: sprite_spawn_position.extend(1.0),
            direction: direction_vector_normalized,
            velocity: 300.0,
            collision_offset: sprite_head_offset 
        },
        SpellFlightTime {timer: Timer::new(Duration::from_secs(1), TimerMode::Once)},
        animation_indices.clone(),
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        ActiveEvents::COLLISION_EVENTS,
        LockedAxes::ROTATION_LOCKED,
        Name::new("FireBall"),
        (
            RigidBody::Dynamic,
            Collider::cuboid(5.0, 5.0),
            // TransformBundle::from(Transform { translation: Vec3::new(26.0, 0.0, 0.0), ..Default::default()}),
        ),
        Velocity {
            linvel: direction_vector_normalized * 300.0,
            angvel: 0.0
        },
    ));

    spell_entity.insert(CollisionGroups::new(Group::from_bits(0b01).unwrap(), Group::from_bits(0b01).unwrap()));
}

// fn spell_flight_system(
//     time: Res<Time>,
//     mut cast_spell_query: Query<(&mut Transform, &CastSpell)>
// ) {
//     for (mut transform, cast_spell) in cast_spell_query.iter_mut() {
//         let delta_seconds = time.delta_seconds();
//         transform.translation.x += cast_spell.direction.x * delta_seconds * cast_spell.velocity;
//         transform.translation.y += cast_spell.direction.y * delta_seconds * cast_spell.velocity;
//     }
// }

fn enable_spell_cooldown(
    mut spell_cooldown: ResMut<SpellCoolDown>,
    time: Res<Time>,
) {
    spell_cooldown.timer.tick(time.delta());
}

fn enemy_spell_collision_event(
    mut collision_events: EventReader<CollisionEvent>,
    query_name: Query<&Name, With<Collider>>,
    mut events: EventWriter<EnemySpellCollisionEvent>
) {
    for event in collision_events.read() {
        println!("Collision event detected: {:?}", event);
        match event {
            CollisionEvent::Started(entity_1, entity_2, _) => {
                let unknown = &Name::new("Unknown");
                let entity_1_name = query_name.get(*entity_1).unwrap_or(unknown);
                let entity_2_name = query_name.get(*entity_2).unwrap_or(unknown);
                println!("Entity 1 name: {:?}, Entity 2 name: {:?}", entity_1_name, entity_2_name);
                if entity_1_name == &Name::new("FireBall") && entity_2_name == &Name::new("Enemy") {
                    events.send(EnemySpellCollisionEvent { 
                        spell_entity: *entity_1, 
                        enemy_entity: *entity_2
                    });
                }
                else if entity_1_name == &Name::new("Enemy") && entity_2_name == &Name::new("FireBall") {
                    events.send(EnemySpellCollisionEvent { 
                        spell_entity: *entity_2, 
                        enemy_entity: *entity_1
                    });
                }
            },
            _ => {},
        }
    }
}

fn receive_enemy_spell_collision_event(
    mut events: EventReader<EnemySpellCollisionEvent>,
    mut commands: Commands,
) {
    for event in events.read().enumerate() {
        commands.entity(event.1.enemy_entity).despawn_recursive();
        commands.entity(event.1.spell_entity).despawn_recursive();
    }
}