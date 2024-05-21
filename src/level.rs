use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use crate::SCALE;

const LEVEL_0_PATH: &str = ".\\level\\level.png";

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    println!("Setting up level");
    {
        let texture = asset_server.load(LEVEL_0_PATH);
        commands.spawn(
            SpriteBundle {
                texture: texture,
                ..Default::default()
            }
        );
    }
}