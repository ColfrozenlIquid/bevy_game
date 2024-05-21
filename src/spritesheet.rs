use std::fs::File;
use std::io::Read;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{player::{PlayerAnimationStates, PlayerSpriteAtlas}, AppState, PLAYER_SPRITE_PATH};

const SPRITE_ATLAS_PATH: &str = "./assets/spritesheets/sprite_atlas.json";
const SPRITE_PATH: &str = "./assets/spritesheets/sprites.json";
pub const ANGEL_IDLE: &str = "angel_idle_anim";
pub const ANGEL_RUN: &str = "angel_run_anim";
pub const DWARF_F_IDLE: &str = "dwarf_f_idle_anim";
pub const DWARF_F_RUN: &str = "dwarf_f_run_anim";

pub const KNIGHT_M_IDLE: &str = "knight_m_idle_anim";
pub const KNIGHT_M_RUN: &str = "knight_m_run_anim";
pub const KNIGHT_M_HIT: &str = "knight_m_hit_anim";

pub const LIZARD_M_IDLE: &str = "lizard_m_idle_anim";
pub const LIZARD_M_RUN: &str = "lizard_m_run_anim";
pub const LIZARD_M_HIT: &str = "lizard_m_hit_anim";

pub const SLUG_ANIM: &str = "slug_anim";

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
    mut player_sprite: ResMut<PlayerSpriteAtlas>,
) {
    println!("Loading spritesheet plugin");
    let mut file = File::open(SPRITE_ATLAS_PATH).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file contents");
    let spriteatlas_vector: Vec<JSONSpriteAtlas> = serde_json::from_str(&contents).unwrap();

    {
        let texture = asset_server.load(PLAYER_SPRITE_PATH);
        let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 1, None, None);
        let texture_atlas_layout_handle = texture_atlas_layouts.add(layout);
        player_sprite.image = texture;
        player_sprite.layout = texture_atlas_layout_handle;
    }

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

    {
        for sprites in &sprites_collection.sprites {
            println!("{}", sprites.name);
        }
    }
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

pub fn get_sprite(
    requested_sprite: String, 
    atlases: &TextureAtlases,
    sprite_collection: &SpriteCollection,
) -> ((Handle<TextureAtlasLayout>, Handle<Image>, JSONSpriteAtlas), JSONSprite)
{
    let mut requested_atlas_name = String::new();
    let mut json_sprite = JSONSprite { file_name: "Error".to_owned(), name: "Error".to_owned(), frame_count: 0, frame_index: Vec::new() };
    for sprite in &sprite_collection.sprites {
        if sprite.name == requested_sprite {
            requested_atlas_name = sprite.file_name.clone();
            json_sprite = sprite.clone();
        }
    }
    for atlas in &atlases.handles {
        if atlas.2.file_name == requested_atlas_name {
            println!("Found sprite atlas handles");
            return (atlas.clone(), json_sprite);
        }
    }
    return (atlases.handles[0].clone(), JSONSprite { file_name: "Error".to_owned(), name: "Error".to_owned(), frame_count: 0, frame_index: Vec::new() });
}

pub fn get_sprite_texture_handle(
    requested_sprite: String, 
    atlases: &TextureAtlases,
    sprite_collection: &SpriteCollection,
) -> Option<Handle<Image>> {
    let mut requested_atlas_name = String::new();

    for sprite in &sprite_collection.sprites {
        if sprite.name == requested_sprite {
            requested_atlas_name = sprite.file_name.clone();
        }
    }

    for atlas in &atlases.handles {
        if atlas.2.file_name == requested_atlas_name {
            return Some(atlas.1.clone());
        }
    }
    None
}

pub fn get_sprite_atlas_layout(
    requested_sprite: String, 
    atlases: &TextureAtlases,
    sprite_collection: &SpriteCollection,
) -> Option<Handle<TextureAtlasLayout>> {
    let mut requested_atlas_name = String::new();

    for sprite in &sprite_collection.sprites {
        if sprite.name == requested_sprite {
            requested_atlas_name = sprite.file_name.clone();
        }
    }

    for atlas in &atlases.handles {
        if atlas.2.file_name == requested_atlas_name {
            return Some(atlas.0.clone());
        }
    }
    None
}

pub fn get_sprite_animation_states(
    requested_state: PlayerAnimationStates,
    requested_sprite: String, 
    sprite_collection: &SpriteCollection,
) -> (PlayerAnimationStates, Vec<usize>) {
    let mut indices = Vec::<usize>::new();

    for sprite in &sprite_collection.sprites {
        if sprite.name == requested_sprite {
            for (_name, index) in &sprite.frame_index {
                indices.push(*index as usize);
            }
        }
    }

    return (requested_state, indices);
}