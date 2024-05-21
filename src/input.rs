use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::player::{ControllablePlayer, Facing, PlayerAnimationStates, SpriteAnimationStates, SpriteFacing, Velocity};
use crate::{AppState, CursorWorldCoordinates, PlayerCamera, PlayerInput, PlayerPosition};

use crate::magic::{icespike_attack_animation, FireBallSpriteAtlas, IceSpikeSpriteAtlas, SelectedSpell, Spells};
use crate::magic::fireball_attack_animation;

pub const SPEED: f32 = 0.5;
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput::default());
        app.add_systems(Update, (mouse_button_input_system));
        app.add_systems(Update, (keyboard_input_system).run_if(in_state(AppState::InGame)));
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
    mut player_velocity_query: Query<&mut Velocity, With<ControllablePlayer>>,
    mut animation_state_query: Query<(&mut SpriteAnimationStates, &mut SpriteFacing), With<ControllablePlayer>>,
    // wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>
) {
    for (mut state, mut facing) in &mut animation_state_query {
        if keyboard_input.just_released(KeyCode::KeyW) {
            state.current_state = PlayerAnimationStates::IDLE;
            state.changed = true;
        }

        if keyboard_input.just_released(KeyCode::KeyA) {
            state.current_state = PlayerAnimationStates::IDLE;
            state.changed = true;
        }

        if keyboard_input.just_released(KeyCode::KeyS) {
            state.current_state = PlayerAnimationStates::IDLE;
            state.changed = true;
        }

        if keyboard_input.just_released(KeyCode::KeyD) {
            state.current_state = PlayerAnimationStates::IDLE;
            state.changed = true;
        }

        if keyboard_input.just_pressed(KeyCode::KeyA) {
            state.current_state = PlayerAnimationStates::RUNNING;
            state.changed = true;
            facing.facing = Facing::LEFT;
        }
    
        if keyboard_input.just_pressed(KeyCode::KeyD) {
            state.current_state = PlayerAnimationStates::RUNNING;
            state.changed = true;
            facing.facing = Facing::RIGHT;
        }
    
        if keyboard_input.just_pressed(KeyCode::KeyW) {
            state.current_state = PlayerAnimationStates::RUNNING;
            state.changed = true;
        }
    
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            state.current_state = PlayerAnimationStates::RUNNING;
            state.changed = true;
        }
    }

    for mut velocity in &mut player_velocity_query {
        let mut velocity = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            player_velocity.0 = Vec2::new(-1.0 * SPEED, 0.0);
            velocity.x = -SPEED;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
                player_velocity.0 = Vec2::new(1.0 * SPEED, 0.0);
                velocity.x
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
                player_velocity.0 = Vec2::new(0.0, 1.0 * SPEED);
                velocity = Vec3::new(0.0, 1.0 * SPEED, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
                player_velocity.0 = Vec2::new(0.0, -1.0 * SPEED);
                velocity = Vec3::new(0.0, -1.0 * SPEED, 0.0);
        }
    }

    // if keyboard_input.just_released(KeyCode::KeyW) {
    //     for (mut state, facing) in &mut animation_state_query {
    //         state.current_state = PlayerAnimationStates::IDLE;
    //         state.changed = true;
    //     }
    //     player_velocity.0 = Vec2::ZERO;
    // }

    // if keyboard_input.just_released(KeyCode::KeyA) {
    //     for (mut state, facing) in &mut animation_state_query {
    //         state.current_state = PlayerAnimationStates::IDLE;
    //         state.changed = true;
    //     }
    //     player_velocity.0 = Vec2::ZERO;
    // }

    // if keyboard_input.just_released(KeyCode::KeyS) {
    //     for (mut state, facing) in &mut animation_state_query {
    //         state.current_state = PlayerAnimationStates::IDLE;
    //         state.changed = true;
    //     }
    //     player_velocity.0 = Vec2::ZERO;
    // }

    // if keyboard_input.just_released(KeyCode::KeyD) {
    //     for (mut state, facing) in &mut animation_state_query {
    //         state.current_state = PlayerAnimationStates::IDLE;
    //         state.changed = true;
    //     }
    //     player_velocity.0 = Vec2::ZERO;
    // }

    // if keyboard_input.just_pressed(KeyCode::KeyA) {
    //     for (mut state, mut facing) in &mut animation_state_query {
    //         state.current_state = PlayerAnimationStates::RUNNING;
    //         state.changed = true;
    //         facing.facing = Facing::LEFT;
    //     }
    // }

    // if keyboard_input.just_pressed(KeyCode::KeyD) {
    //     for (mut state, mut facing) in &mut animation_state_query {
    //         state.current_state = PlayerAnimationStates::RUNNING;
    //         state.changed = true;
    //         facing.facing = Facing::RIGHT;
    //     }
    // }

    // if keyboard_input.just_pressed(KeyCode::KeyW) {
    //     for (mut state, facing) in &mut animation_state_query {
    //         state.current_state = PlayerAnimationStates::RUNNING;
    //         state.changed = true;
    //     }
    // }

    // if keyboard_input.just_pressed(KeyCode::KeyS) {
    //     for (mut state, facing) in &mut animation_state_query {
    //         state.current_state = PlayerAnimationStates::RUNNING;
    //         state.changed = true;
    //     }
    // }

    // if keyboard_input.pressed(KeyCode::KeyA) {
    //         player_velocity.0 = Vec2::new(-1.0 * SPEED, 0.0);
    //         velocity = Vec3::new(-1.0 * SPEED, 0.0, 0.0);
    // }

    // if keyboard_input.pressed(KeyCode::KeyD) {
    //         player_velocity.0 = Vec2::new(1.0 * SPEED, 0.0);
    //         velocity = Vec3::new(1.0 * SPEED, 0.0, 0.0);
    // }

    // if keyboard_input.pressed(KeyCode::KeyW) {
    //         player_velocity.0 = Vec2::new(0.0, 1.0 * SPEED);
    //         velocity = Vec3::new(0.0, 1.0 * SPEED, 0.0);
    // }

    // if keyboard_input.pressed(KeyCode::KeyS) {
    //         player_velocity.0 = Vec2::new(0.0, -1.0 * SPEED);
    //         velocity = Vec3::new(0.0, -1.0 * SPEED, 0.0);
    // }

    player_input.left = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    player_input.right = keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
    player_input.up = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    player_input.down = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    
    player_position.transform += velocity * 5.0;
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

    if mouse_input.pressed(MouseButton::Right) {
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