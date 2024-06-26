// use std::io::{BufRead, BufReader};
// use std::fs::File;

// use bevy::prelude::*;

// const MAP_FILE_PATH: &str = "assets/map/map1.txt";
// const MAP_SPRITE_PATH: &str = "./sprites/character and tileset/Dungeon_Tileset.png";
// const TILE_SIZE: f32 = 16.0;
// const OFFSET_X: f32 = -200.0;
// const OFFSET_Y: f32 = -200.0;

// #[derive(Resource, Default)]
// struct MapData {
//     data: Vec<char>,
//     width: usize,
//     height: usize
// }

// #[derive(Component)]
// pub struct TileCollider;

// #[derive(Default, Resource)]
// struct MapSpriteAtlas {
//     handle: Handle<TextureAtlasLayout>,
// }

// pub struct TileMapPlugin;

// impl Plugin for TileMapPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, (setup, generate_tile_textureatlas, generate_map).chain());
            
//         app.insert_resource(MapData::default())
//             .insert_resource(MapSpriteAtlas::default());
//     }
// }

// fn setup(
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
//     mut map_sprites: ResMut<MapSpriteAtlas>,
// ) {
//     let texture_handle = asset_server.load(MAP_SPRITE_PATH);
//     let texture_atlas = TextureAtlasLayout::from_grid(texture_handle, Vec2::new(16.0, 16.0), 10, 10, None);
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);
//     map_sprites.handle = texture_atlas_handle.clone();
// }

// fn generate_tile_textureatlas(
//     mut map_data: ResMut<MapData>
// ) {
//     let mut filereader = BufReader::new(File::open(MAP_FILE_PATH).expect(format!("Error opening file: {}", MAP_FILE_PATH).as_str()));
//     let mut buffer = Vec::<u8>::new();
    
//     let mut columns: usize = 0;
//     let mut rows: usize = 0;
//     while filereader.read_until(b'\n', &mut buffer).expect("method read_until failed") != 0 {
//         let s = String::from_utf8(buffer).expect("method from_utf8 failed");
//         for c in s.chars() {
//             map_data.data.push(c);
//             columns += 1;
//         }
//         buffer = s.into_bytes();
//         buffer.clear();
//         rows += 1;
//     }
//     map_data.width = columns / rows;
//     map_data.height = rows;
// }   

// fn generate_map(
//     mut commands: Commands, 
//     map_data: Res<MapData>,
//     map_sprites: Res<MapSpriteAtlas>
// ) {
//     println!("Map width: {:?}", map_data.width);
//     println!("Map height: {:?}", map_data.height);
//     let mut column: usize = 0;
//     let mut row: usize = 0;
//     for c in &map_data.data {
//         if column == map_data.width {
//             column = 0;
//             row += 1;
//         }
//         if *c == '#' {
//             commands.spawn(SpriteSheetBundle {
//                 atlas: map_sprites.handle.clone(),
//                 sprite: TextureAtlasSprite::new(1),
//                 transform: Transform {
//                     translation: Vec3 { x: column as f32 * TILE_SIZE * 6.0 - OFFSET_X, y: row as f32 * TILE_SIZE * 6.0 - OFFSET_Y, z: -0.5 },
//                     rotation: Quat::default(),
//                     scale: Vec3 { x: 6.0, y: 6.0, z: 6.0 }
//                 },
//                 ..Default::default()
//             })
//             .insert(GlobalTransform::default())
//             .insert(TileCollider);
//         }
//         else if *c == '.' {
//             commands.spawn(SpriteSheetBundle {
//                 atlas: map_sprites.handle.clone(),
//                 sprite: TextureAtlasSprite::new(6),
//                 transform: Transform {
//                     translation: Vec3 { x: column as f32 * TILE_SIZE * 6.0 - OFFSET_X, y: row as f32 * TILE_SIZE * 6.0 - OFFSET_Y, z: -0.5 },
//                     rotation: Quat::default(),
//                     scale: Vec3 { x: 6.0, y: 6.0, z: 6.0 }
//                 },
//                 ..Default::default()
//             })
//             .insert(GlobalTransform::default());
//             // .insert(TileCollider);
//         }
//         column += 1;
//     }
// }