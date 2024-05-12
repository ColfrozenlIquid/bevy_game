use bevy::{prelude::*, window::PrimaryWindow};

use crate::{CursorWorldCoordinates, PlayerCamera};

pub struct CursorPlugin;
const GAME_CURSOR_SPRITE_PATH: &str = ".\\sprites\\cursor\\cursor_hand_200.png";

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game_cursor);
        app.add_systems(Update, (cursor_system, move_cursor));
    }
}

#[derive(Component)]
pub struct GameCursor;

fn setup_game_cursor(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;

    commands.spawn(
        (ImageBundle {
            image: asset_server.load(GAME_CURSOR_SPRITE_PATH).into(),
            style: Style {
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            z_index: ZIndex::Global(15),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        GameCursor
    ));
    println!("Spawned cursor sprite");
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

fn move_cursor(
    window: Query<&Window>,
    mut cursor: Query<&mut Style, With<GameCursor>>
) {
    let window: &Window = window.single();
    if let Some(position) = window.cursor_position() {
        let mut img_transform= cursor.single_mut();
        img_transform.left = Val::Px(position.x);
        img_transform.top = Val::Px(position.y);
    }
}