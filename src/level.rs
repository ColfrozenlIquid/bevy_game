// use std::collections::{HashMap, HashSet};

// use bevy::prelude::*;
// use bevy_ecs_ldtk::prelude::*;

// const LEVEL_0_PATH: &str = ".\\level\\level_0.ldtk";

// pub struct LevelPlugin;

// impl Plugin for LevelPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, setup);
//         // app.add_plugins(LdtkPlugin);
//         // app.insert_resource(LevelSelection::index(1))
//         // .insert_resource(LdtkSettings {
//         //     level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation { 
//         //         load_level_neighbors: true 
//         //     },
//         //     ..Default::default()
//         // })
//         // .add_systems(Startup, spawn_wall_collision)
//         // .register_ldtk_int_cell::<WallBundle>(1);
//     }
// }

// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ){
//     let ldtk_handle: Handle<_> = asset_server.load(LEVEL_0_PATH);
//     commands.spawn(
//         LdtkWorldBundle {
//             ldtk_handle: ldtk_handle,
//             ..Default::default()
//         });
// }

// #[derive(Clone, Copy, Eq, PartialEq, Debug, Default, Component)]
// struct Wall;

// #[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
// struct WallBundle {
//     wall: Wall
// }

// fn spawn_wall_collision(
//     mut commands: Commands,
    
// ) {

//     #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
//     struct Plate {
//         left: i32,
//         right: i32,
//     }

//     #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
//     struct Rect {
//         left: i32,
//         right: i32,
//         top: i32,
//         bottom: i32,
//     }

//     let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();    
// }
