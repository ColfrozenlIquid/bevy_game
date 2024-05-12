use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{CursorWorldCoordinates, PlayerCamera, PlayerInput, PlayerPosition};

use crate::magic::{icespike_attack_animation, FireBallSpriteAtlas, IceSpikeSpriteAtlas, SelectedSpell, Spells};
use crate::magic::fireball_attack_animation;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (keyboard_input_system, mouse_button_input_system));
    }
}

// fn wall_collision(
//     player_pos: Vec3,
//     wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>
// ) -> bool {
//     for wall_transform in wall_query.iter() {
//         let collision = collide(
//             player_pos, 
//             Vec2::splat(16.0 * 6.0 * 0.5), 
//             wall_transform.translation, 
//             Vec2::splat(16.0 * 6.0)
//         );
//         if collision.is_some() {
//             return false;
//         }
//     }
//     return true;
// }

fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut player_input: ResMut<PlayerInput>,
    mut player_position: ResMut<PlayerPosition>,
    // wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>
) {
    let mut direction = Vec3::ZERO;
    // let current_position = player_position.transform;

    if keyboard_input.pressed(KeyCode::KeyA) {
        // if wall_collision(current_position + Vec3::new(-2.0, 0.0, 0.0), &wall_query) {
            direction.x -= 1.0;
        // }
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        // if wall_collision(current_position + Vec3::new(2.0, 0.0, 0.0), &wall_query) {
            direction.x += 1.0;
        // }
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        // if wall_collision(current_position + Vec3::new(0.0, 2.0, 0.0), &wall_query) {
            direction.y += 1.0;
        // }
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        // if wall_collision(current_position + Vec3::new(0.0, -2.0, 0.0), &wall_query) {
            direction.y -= 1.0;
        // }
    }

    player_input.left = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    player_input.right = keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
    player_input.up = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    player_input.down = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    
    player_position.transform += direction * 5.0;
}

fn mouse_button_input_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    mut commands: Commands,
    // sword_sprite: Res<SwordSpriteAtlas>,
    cursor_coord: Res<CursorWorldCoordinates>,
    selected_spell: Res<SelectedSpell>,
    // mut spell_cooldown: ResMut<SpellCoolDown>,
    fireball_sprite: Res<FireBallSpriteAtlas>,
    icespike_sprite: Res<IceSpikeSpriteAtlas>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if mouse_input.just_pressed(MouseButton::Right) {
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate()) {
                println!("Pressed left mouse button");
                println!("Cursor position is: {},{}", world_position.x, world_position.y);
                //sword_attack_animation(&mut commands, &sword_sprite, &world_position, &camera_transform.translation());
                // if spell_cooldown.timer.finished() {
                    // spell_cooldown.timer.set_duration(Duration::from_secs(2));
                    if selected_spell.spell == Spells::FireBall {
                        fireball_attack_animation(&mut commands, &fireball_sprite, &cursor_coord, &camera_transform.translation());
                    }
                    if selected_spell.spell == Spells::IceSpike {
                        icespike_attack_animation(&mut commands, &icespike_sprite, &cursor_coord, &camera_transform.translation())
                    }
                // }
                // else {
                //     println!("Spells are on cooldown");
                // }
        }
    }    
}