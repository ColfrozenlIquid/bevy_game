use bevy::prelude::*;

use crate::{AppState, FONT_PATH};

pub struct HealthBarPlugin;

const HEART_FULL: &str = "./user_interface/heart_500.png";
const MANA_FULL: &str = "./user_interface/mana_full_500.png";

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup);
        app.add_systems(Update, (update_health_bar).run_if(in_state(AppState::InGame)));
        app.insert_resource(PlayerHealth(5));
    }
}

#[derive(Resource, Default)]
struct PlayerHealth(u32);

#[derive(Default)]
enum HeartHealth {
    #[default]
    FULL,
    HALF_FULL,
    EMPTY,
}

#[derive(Component, Default)]
struct Heart {
    id: u32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {

    let texture_handle: Handle<Image> = asset_server.load(HEART_FULL);
    let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(11.0, 10.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(
        TextBundle::from_section(
            "Sample Text",
            TextStyle {
                font: asset_server.load(FONT_PATH),
                font_size: 10.0,
                ..Default::default()
            },
        ).with_style(
            Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            }
        )
    );

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(25.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect {
                    top: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            AtlasImageBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    width: Val::Px(55.0),
                    height: Val::Px(50.0),
                    flex_direction: FlexDirection::Row,
                    margin: UiRect {
                        right: Val::Px(5.0),
                        left: Val::Px(5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle.clone().into(),
                image: UiImage::new(texture_handle.clone()),
                ..Default::default()
            },
            Heart { id: 1, ..Default::default()},
        ));
        parent.spawn((
            AtlasImageBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    width: Val::Px(55.0),
                    height: Val::Px(50.0),
                    flex_direction: FlexDirection::Row,
                    margin: UiRect {
                        right: Val::Px(5.0),
                        left: Val::Px(5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle.clone().into(),
                image: UiImage::new(texture_handle.clone()),
                ..Default::default()
            },
            Heart { id: 2, ..Default::default()},
        ));
        parent.spawn((
            AtlasImageBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    width: Val::Px(55.0),
                    height: Val::Px(50.0),
                    flex_direction: FlexDirection::Row,
                    margin: UiRect {
                        right: Val::Px(5.0),
                        left: Val::Px(5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle.clone().into(),
                image: UiImage::new(texture_handle.clone()),
                ..Default::default()
            },
            Heart { id: 3, ..Default::default()},
        ));
    });
}

fn update_health_bar(
    mut atlas_image: Query<(&mut TextureAtlas, &Heart), With<Heart>>,
    player_health: Res<PlayerHealth>,
) {
    for (mut atlas_image, heart) in &mut atlas_image {
        let fill_level = player_health.0 as f32 / heart.id as f32;
        if fill_level >= 2.0 {
            atlas_image.index = 0;
        } else if fill_level < 1.5 {
            atlas_image.index = 2;
        } else {
            atlas_image.index = 1;
        }
    }   
}