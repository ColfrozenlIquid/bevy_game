use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::control::KinematicCharacterController;

use crate::player::{ControllablePlayer, PlayerAnimationStates, PlayerSpriteAnimationStates};
use crate::{AppState, CursorWorldCoordinates, PlayerCamera, PlayerInput};

use crate::magic::{spawn_icespike_attack, FireBallSpriteAtlas, IceSpikeSpriteAtlas, SelectedSpell, Spells};
use crate::magic::spawn_fireball_attack;

pub const SPEED: f32 = 200.0;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput::default());
        app.add_systems(Update, mouse_button_input_system);
        app.add_systems(Update, (keyboard_input_system).run_if(in_state(AppState::InGame)));
    }
}

fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut animation_state_query: Query<&mut PlayerSpriteAnimationStates, With<ControllablePlayer>>,
    mut kinematiccontroller_query: Query<&mut KinematicCharacterController>,
) {
    let mut player = kinematiccontroller_query.single_mut();
    let mut new_velocity = Vec2::ZERO;

    for mut state in &mut animation_state_query {
        if keyboard_input.pressed(KeyCode::KeyA) 
                || keyboard_input.pressed(KeyCode::KeyD) 
                || keyboard_input.pressed(KeyCode::KeyW)
                || keyboard_input.pressed(KeyCode::KeyS) {
            state.current_state = PlayerAnimationStates::RUNNING;
            state.changed = true;
        } else {
            state.current_state = PlayerAnimationStates::IDLE;
        }
    }

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
    
}

fn mouse_button_input_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    mut commands: Commands,
    cursor_coord: Res<CursorWorldCoordinates>,
    selected_spell: Res<SelectedSpell>,
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