use bevy::{prelude::*, render::view::visibility};
use bevy_inspector_egui::egui::style;

use crate::{AppState, CursorWorldCoordinates};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
        app.add_systems(Update, (drag_tile).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
struct InventoryTile;

fn setup(
    mut commands: Commands,
    asset_Server: Res<AssetServer>,
) {
    commands.spawn(
        NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Auto,
                height: Val::Auto,
                justify_content: JustifyContent::Center,
                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                align_items: AlignItems::Center,
                ..Default::default()
            },
            // transform: Transform::from_xyz(100.0, 100.0, 1.0),
            ..Default::default()
        }
    ).with_children(
        |builder| {
            item_rect(builder, Color::ORANGE);
            item_rect(builder, Color::GREEN);
            item_rect(builder, Color::BLUE);
            item_rect(builder, Color::RED);
        }
    );
}

fn item_rect(builder: &mut ChildBuilder, color: Color) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Px(80.0),
                height: Val::Px(80.0),
                // padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                ButtonBundle {
                    background_color: BackgroundColor(color.into()),
                    ..default()
                },
                InventoryTile,
            ));
        });
}

fn drag_tile(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Style, &Interaction, &mut Visibility, &mut Transform), (Changed<Interaction>, With<InventoryTile>)>,
    mut cursor_moved_event: EventReader<CursorMoved>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut cursor_position: Res<CursorWorldCoordinates>
) {
   for (entity, mut style, interaction, mut visibility, mut transform) in &mut query {
    match *interaction {
        Interaction::Pressed => {
            println!("Pressed inventory tile");
            // println!("Cursor position: {:?}", cursor_position.0);
            
    },
        Interaction::Hovered => {
            println!("Hovered over inventory tile");
            transform.translation = cursor_position.0;
            // *visibility = Visibility::Hidden;
            // println!("View visibility: {:?}", visibility);
        },
        Interaction::None => {
            println!("Released");
            // *visibility = Visibility::Visible;
            // println!("View visibility: {:?}", visibility);
        },
    }
   } 
}
