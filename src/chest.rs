use bevy::prelude::*;

use crate::{player::ControllablePlayer, AppState, SCALE};

pub struct ChestPlugin;

const KEYBOARD_SPRITES: &str = "./sprites/keyboard/Keyboard.png";
const CHEST_SPRITES: &str = "./sprites/inventory/chest/chests.png";

impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::LoadingScreen), load_sprite_atlases);
        app.add_systems(OnEnter(AppState::InGame), setup);
        app.add_systems(Update, (interaction_system).run_if(in_state(AppState::InGame)));
        app.insert_resource(KeyboardSpriteAtlas::default());
        app.insert_resource(ChestSpriteAtlas::default());
        app.insert_resource(SpawnedEntity::default());
        app.add_event::<ChestInteractionEvent>();
    }
}

#[derive(Resource, Default, Clone)]
struct KeyboardSpriteAtlas {
    layout: Handle<TextureAtlasLayout>,
    image: Handle<Image>,
}

#[derive(Resource, Default, Clone)]
struct ChestSpriteAtlas {
    layout: Handle<TextureAtlasLayout>,
    image: Handle<Image>,
}

#[derive(Component, Default)]
struct Chest {
    state: ChestState,
}

#[derive(Default, PartialEq)]
enum ChestState {
    #[default]
    CLOSED,
    CLOSED_INSPECTED,
    OPENED,
    OPENED_INSPECTED,
}

#[derive(Component, Clone)]
struct SpriteIndex(usize);

#[derive(Event)]
struct ChestInteractionEvent {
    pub triggered: bool,
}

impl ChestInteractionEvent {
    fn new(triggered: bool) -> Self {
        return ChestInteractionEvent {
            triggered,
        };
    }
}

#[derive(Resource, Default)]
struct SpawnedEntity {
    entity: Option<Entity>,
}

fn load_sprite_atlases(
    mut keyboard_sprite_atlas: ResMut<KeyboardSpriteAtlas>,
    mut chest_sprite_atlas: ResMut<ChestSpriteAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    //Load Keyboard sprites
    {
        let image_handle: Handle<Image> = asset_server.load(KEYBOARD_SPRITES);
        let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 8, 14, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        keyboard_sprite_atlas.image = image_handle;
        keyboard_sprite_atlas.layout = texture_atlas_handle;
    }

    //Load Chest Sprite
    {
        let image_handle: Handle<Image> = asset_server.load(CHEST_SPRITES);
        let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 2, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        chest_sprite_atlas.image = image_handle;
        chest_sprite_atlas.layout = texture_atlas_handle;
    }
}

fn setup(
    mut commands: Commands,
    chest_sprite_atlas: Res<ChestSpriteAtlas>,
) {

    let mut sprite_index = SpriteIndex(0);

    let chest_entity = commands.spawn((
        SpriteSheetBundle {
            sprite: Sprite {
                flip_x: false,
                ..Default::default()
            },
            texture: chest_sprite_atlas.image.clone(),
            atlas: TextureAtlas {
                layout: chest_sprite_atlas.layout.clone(),
                index: sprite_index.0,
            },
            transform: Transform {
                translation: Vec3 { x: 1400.0, y: 1400.0, z: 5.0 },
                rotation: Quat::default(),
                scale: Vec3 { x: SCALE/1.2, y: SCALE/1.2, z: 1.0 },
            },
            ..Default::default()
        },
        Chest::default(),
        sprite_index.clone(),
    )).id();
}

fn interaction_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<ControllablePlayer>>,
    mut chest_query: Query<(&Transform, &mut SpriteIndex, &mut TextureAtlas, &mut Chest), With<Chest>>,
    mut interaction_event: EventWriter<ChestInteractionEvent>,
    keyboard_sprites: Res<KeyboardSpriteAtlas>,
    mut spawned_entity: ResMut<SpawnedEntity>,
) {
    for player_transform in &player_query {
        for (chest_transform, mut index, mut sprite, mut chest) in &mut chest_query {
            let distance = player_transform.translation.distance(chest_transform.translation);
            if distance < 100.0 {
                if chest.state == ChestState::CLOSED {
                    println!("Setting state to closed inspected");
                    chest.state = ChestState::CLOSED_INSPECTED;
                    index.0 = 4;
                    sprite.index = index.0;
                    spawn_interaction_key(&mut commands, &keyboard_sprites, chest_transform, &mut spawned_entity);
                }
                if chest.state == ChestState::OPENED {
                    chest.state = ChestState::OPENED_INSPECTED;
                }
                if keyboard_input.just_pressed(KeyCode::KeyE) {
                    interaction_event.send(ChestInteractionEvent::new(true));
                    println!("Sent a chest interaction event");
                }
            } 
            else {
                if chest.state == ChestState::CLOSED_INSPECTED {
                    println!("Setting state to closed");
                    chest.state = ChestState::CLOSED;
                    index.0 = 0;
                    sprite.index = index.0;
                    despawn_interaction_key(&mut commands, &mut spawned_entity);
                }
            } 
        }
    }
}

fn spawn_interaction_key(
    commands: &mut Commands,
    keyboard_sprites: &Res<KeyboardSpriteAtlas>,
    transform: &Transform,
    spawned_entity: &mut ResMut<SpawnedEntity>,
) {
    let position = transform.translation + Vec3::new(40.0, 40.0, 0.0);

    let key_entity = commands.spawn((
        SpriteSheetBundle {
            sprite: Sprite {
                flip_x: false,
                ..Default::default()
            },
            texture: keyboard_sprites.image.clone(),
            atlas: TextureAtlas {
                layout: keyboard_sprites.layout.clone(),
                index: 20,
            },
            transform: Transform {
                translation: position,
                rotation: Quat::default(),
                scale: Vec3 { x: SCALE/2.0, y: SCALE/2.0, z: 1.0 },
            },
            ..Default::default()
        },
    )).id();

    spawned_entity.entity = Some(key_entity);
    println!("Spawned key entity");
}

fn despawn_interaction_key(
    commands: &mut Commands,
    spawned_entity: &mut ResMut<SpawnedEntity>,
) {
    if let Some(entity) = spawned_entity.entity {
        commands.entity(entity).despawn();
        spawned_entity.entity = None;
        println!("Despawned key entity");
    }
}