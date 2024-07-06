use std::f32::consts::PI;
use bevy_rapier2d::prelude::*;

use bevy::{prelude::*, tasks::futures_lite::io::Repeat};

use crate::{game::Equipment, player::ControllablePlayer, spritesheet::{get_sprite_atlas_layout, get_sprite_texture_handle, SpriteCollection, TextureAtlases, WEAPON_SWORD}, AppState, CursorWorldCoordinates, SCALE};

pub struct MeleePlugin;

impl Plugin for MeleePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
        app.add_systems(Update, (sword_follow_cursor, sword_swing, update_swing, sword_stab_animation).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
enum SwordState {
    IDLE,
    ATTACK
}

#[derive(Component)]
struct SwordSwing {
    timer: Timer,
    swinging: bool,
    start_rotation: f32,
    end_rotation: f32,
}

impl SwordSwing {
    fn new(duration: f32, start_rotation: f32,  end_rotation: f32) -> Self {
        SwordSwing { 
            timer: Timer::from_seconds(duration, TimerMode::Once), 
            swinging: false, 
            start_rotation, 
            end_rotation 
        }
    }
}

fn update_swing(
    time: Res<Time>,
    mut query: Query<(&mut SwordSwing, &mut Transform)>,
) {
    for (mut swing, mut transform) in query.iter_mut() {
        if swing.swinging {
            swing.timer.tick(time.delta());
            let progress = swing.timer.fraction();
            let current_rotation = transform.rotation;
            let target_rotation = Quat::from_rotation_z(swing.start_rotation + progress * (swing.end_rotation - swing.start_rotation));
            transform.rotation = current_rotation.slerp(target_rotation, 0.5); // Adjust the slerp factor as needed

            if swing.timer.finished() {
                swing.swinging = false;
            }
        }
    }
}

fn sword_swing(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut SwordSwing, &mut Transform)>,
) {
    for (mut swing, mut transform) in &mut query {
        if keyboard_input.just_pressed(KeyCode::Space) && !swing.swinging {
            println!("Started swing");
            swing.swinging = true;
            swing.timer.reset();
            swing.start_rotation = transform.rotation.z;
            transform.rotation = Quat::from_rotation_z(swing.start_rotation);
        }
    }
}

fn setup(
    mut commands: Commands,
    texture_atlas: Res<TextureAtlases>,
    sprite_collection: Res<SpriteCollection>
) {
    let requested_sprite = WEAPON_SWORD.to_owned();

    let sword_sprite_texture = get_sprite_texture_handle(
        requested_sprite.clone(), 
        &texture_atlas, 
        &sprite_collection
    ).expect("Could not load sword texture handle");

    let sword_sprite_atlas_layout = get_sprite_atlas_layout(
        requested_sprite.clone(),
        &texture_atlas, 
        &sprite_collection
    ).expect("Could not load sword texture atlas layout");

    let _sword_entity = commands.spawn((
        SpriteSheetBundle {
            sprite: Sprite {
                flip_x: false,
                ..Default::default()
            },
            texture: sword_sprite_texture,
            atlas: TextureAtlas {
                layout: sword_sprite_atlas_layout.clone(),
                index: 0,
            },
            transform: Transform {
                translation: Vec3 { x: 1400.0, y: 1600.0, z: 6.0 },
                rotation: Quat::default(),
                scale: Vec3 { x: SCALE/1.6, y: SCALE/1.6, z: 1.0 },
            },
            ..Default::default()
        },
        Name::new("Sword"),
        Equipment,
        Collider::cuboid(5.0, 8.0),
        SwordState::IDLE,
        SwordSwing::new(0.2, 0.0, PI/2.0),
        Sensor,
    ));
}

fn sword_stab_animation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut SwordSwing, &mut Transform)>,
) {

}

fn sword_follow_cursor(
    mut sword_transform: Query<(&mut Transform, &mut SwordSwing), With<Equipment>>,
    player_transform: Query<&Transform, (With<ControllablePlayer>, Without<Equipment>)>,
    cursor_position: Res<CursorWorldCoordinates>,
) {
    let distance_from_player: f32 = 60.0;
    for (mut sword_translation, swing) in sword_transform.iter_mut() {
        if !swing.swinging {
            if let Ok(player_translation) = player_transform.get_single() {
                let direction_vector_normalized = (cursor_position.0.truncate() - player_translation.translation.truncate()).normalize();
                sword_translation.translation.x = player_translation.translation.x + (direction_vector_normalized.x * distance_from_player);
                sword_translation.translation.y = player_translation.translation.y + (direction_vector_normalized.y * distance_from_player);
                let angle = direction_vector_normalized.angle_between(Vec2 { x: 1.0, y: 0.0 });
    
                if angle <= 0.0 && angle >= -PI/2.0 {
                    sword_translation.rotation = Quat::from_rotation_z(-angle - PI/4.0);
                }
                if angle <= -PI/2.0 && angle >= -PI {
                    sword_translation.rotation = Quat::from_rotation_z(-angle + PI + PI/4.0);
                }
                if angle > 0.0 && angle <= PI/2.0 {
                    sword_translation.rotation = Quat::from_rotation_z(-angle - PI/4.0);
                }
                if angle > PI/2.0 && angle <= PI {
                    sword_translation.rotation = Quat::from_rotation_z(-angle + PI + PI/4.0);
                }
            }
        }
    }
}