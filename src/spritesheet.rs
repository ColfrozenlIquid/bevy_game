use std::fs::File;
use std::io::Read;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::AppState;

pub struct SpriteSheetPlugin;

#[derive(Resource, Default)]
pub struct TextureAtlases {
    pub handles: Vec<(
        Handle<TextureAtlasLayout>,
        Handle<Image>,
        JSONSpriteAtlas,
    )>,
}

#[derive(Resource, Default)]
pub struct SpriteCollection {
    pub sprites: Vec<JSONSprite>,
}

const SPRITE_ATLAS_PATH: &str = "./assets/spritesheets/sprite_atlas.json";
const SPRITE_PATH: &str = "./assets/spritesheets/sprites.json";

pub const ANGEL_IDLE: &str = "angel_idle_anim";
pub const ANGEL_RUN: &str = "angel_run_anim";
pub const DWARF_F_IDLE: &str = "dwarf_f_idle_anim";
pub const DWARF_F_RUN: &str = "dwarf_f_run_anim";

#[derive(Serialize, Deserialize, Clone)]
pub struct JSONSpriteAtlas {
    pub file_name: String,
    pub x_dimension: u32,
    pub y_dimension: u32,
    pub rows: u32,
    pub columns: u32,
    pub sprite_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JSONSprite {
    pub file_name: String,
    pub name: String,
    pub frame_count: u32,
    pub frame_index: Vec<(String, u32)>,
}

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TextureAtlases::default());
        app.insert_resource(SpriteCollection::default());
        app.init_state::<AppState>();
        app.add_systems(OnEnter(AppState::LoadingScreen), setup);
    }
}

fn setup(
    // mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_atlasses: ResMut<TextureAtlases>,
    mut sprites_collection: ResMut<SpriteCollection>,
    mut menu_state: ResMut<NextState<AppState>>,
) {
    let mut file = File::open(SPRITE_ATLAS_PATH).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file contents");
    let spriteatlas_vector: Vec<JSONSpriteAtlas> = serde_json::from_str(&contents).unwrap();

    for sprite_atlas in spriteatlas_vector.iter() {
        let texture: Handle<Image> = asset_server.load(sprite_atlas.file_name.to_owned());
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(sprite_atlas.x_dimension as f32, sprite_atlas.y_dimension as f32),
            sprite_atlas.columns as usize,
            sprite_atlas.rows as usize,
            None,
            None,
        );
        let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
        texture_atlasses.handles.push((texture_atlas_layout_handle, texture, sprite_atlas.clone()));
    }

    let mut file = File::open(SPRITE_PATH).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file contents");
    let sprite_vector: Vec<JSONSprite> = serde_json::from_str(&contents).unwrap();
    for spritejson in sprite_vector {
        sprites_collection.sprites.push(spritejson);
    }

    println!("Gets here spritesheet");

    // {
    //     for sprites in &sprites_collection.sprites {
    //         println!("{}", sprites.name);
    //     }
    // }
    menu_state.set(AppState::InGame);
}

pub fn get_jsonsprite_by_type(requested_sprite: &str, sprite_collection: &Res<SpriteCollection>) -> JSONSprite {
    println!("Sprite collection size: {}", sprite_collection.sprites.len());
    for sprite in &sprite_collection.sprites {
        println!("sprite.name: {}, requested_sprite: {}", sprite.name, requested_sprite);
        if sprite.name == requested_sprite {
            return sprite.clone();
        }
    }
    return JSONSprite {
        file_name: "error".to_owned(),
        name: "error".to_owned(),
        frame_count: 0,
        frame_index: vec![("".to_string(), 0)]
    };
}