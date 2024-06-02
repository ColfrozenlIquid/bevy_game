use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::control::KinematicCharacterController;
use bevy_rapier2d::rapier::dynamics::RigidBody;

use crate::player::{ControllablePlayer, Facing, PlayerAnimationStates, PlayerColliding, SpriteAnimationStates, SpriteFacing, Velocity};
use crate::{AppState, CursorWorldCoordinates, PlayerCamera, PlayerInput, PlayerPosition};

use crate::magic::{spawn_icespike_attack, FireBallSpriteAtlas, IceSpikeSpriteAtlas, SelectedSpell, Spells};
use crate::magic::spawn_fireball_attack;

pub const SPEED: f32 = 200.0;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput::default());
        app.add_systems(Update, (mouse_button_input_system));
        app.add_systems(Update, (keyboard_input_system).run_if(in_state(AppState::InGame)));
    }
}

fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut player_input: ResMut<PlayerInput>,
    mut player_position: ResMut<PlayerPosition>,
    mut player_velocity_query: Query<&mut Velocity, With<ControllablePlayer>>,
    mut animation_state_query: Query<(&mut SpriteAnimationStates, &mut SpriteFacing), With<ControllablePlayer>>,
    player_collision_query: Query<&PlayerColliding, With<ControllablePlayer>>,
    mut kinematiccontroller_query: Query<&mut KinematicCharacterController>,
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

    let mut player = kinematiccontroller_query.single_mut();

    // let player_colliding = player_collision_query.single();
    // if !player_colliding.0 {
        // for mut velocity in &mut player_velocity_query {
            let mut new_velocity = Vec2::ZERO;
    
            if keyboard_input.pressed(KeyCode::KeyA) {
                new_velocity.x = -SPEED * time.delta_seconds();
            }
    
            if keyboard_input.pressed(KeyCode::KeyD) {
                new_velocity.x = SPEED * time.delta_seconds();
            }
    
            if keyboard_input.pressed(KeyCode::KeyW) {
                new_velocity.y = SPEED * time.delta_seconds();
            }
    
            if keyboard_input.pressed(KeyCode::KeyS) {
                new_velocity.y = -SPEED * time.delta_seconds();
            }
            player.translation = Some(new_velocity);
        // }
    // }
    
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
                if selected_spell.spell == Spells::FireBall {
                    spawn_fireball_attack(&mut commands, &fireball_sprite, &cursor_coord, &camera_transform.translation());
                }
                if selected_spell.spell == Spells::IceSpike {
                    spawn_icespike_attack(&mut commands, &icespike_sprite, &cursor_coord, &camera_transform.translation())
                }
        }
    }    
}