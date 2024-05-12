// use bevy::prelude::*;

// pub struct MeleePlugin;

// impl Plugin for MeleePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, setup);
//     }
// }

// fn setup() {

// }

// fn sword_attack_animation(
//     commands: &mut Commands,
//     sword_sprite: &Res<SwordSpriteAtlas>,
//     position_cursor: &Vec2,
//     position_player: &Vec3,
// ) {
//     let animation_indices = AnimationIndices { first: 0, last: 4};

//     commands.spawn((
//         SpriteSheetBundle {
//             texture_atlas: sword_sprite.handle.clone(),
//             sprite: TextureAtlasSprite::new(animation_indices.first),
//             transform: Transform {
//                 translation: Vec3::new(position_player.x, position_player.y, 1.0),
//                 scale: Vec3::new(SCALE/2.0, SCALE/2.0, 1.0),
//                 ..Default::default()
//             },
//             ..Default::default()
//         },
//         Sword{ curent_index: 0 },
//         animation_indices.clone(),
//         AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
//     ));
// }

// fn sword_follow_cursor(
//     mut sword_transform: Query<&mut Transform, With<Equipment>>,
//     player_transform: Query<(&Transform, With<ControllablePlayer>, Without<Equipment>)>,
//     cursor_position: Res<CursorWorldCoordinates>,
// ) {
//     let distance_from_player: f32 = 60.0;
//     for mut sword_translation in sword_transform.iter_mut() {
//         if let Ok(player_translation) = player_transform.get_single() {
//             let direction_vector_normalized = (cursor_position.0.truncate() - player_translation.0.translation.truncate()).normalize();
//             sword_translation.translation.x = player_translation.0.translation.x + (direction_vector_normalized.x * distance_from_player);
//             sword_translation.translation.y = player_translation.0.translation.y + (direction_vector_normalized.y * distance_from_player);
//             let angle = direction_vector_normalized.angle_between(Vec2 { x: 1.0, y: 0.0 });

//             if angle <= 0.0 && angle >= -PI/2.0 {
//                 sword_translation.rotation = Quat::from_rotation_z(-angle);
//             }
//             if angle <= -PI/2.0 && angle >= -PI {
//                 sword_translation.rotation = Quat::from_rotation_z(-angle + PI);
//             }
//             if angle > 0.0 && angle <= PI/2.0 {
//                 sword_translation.rotation = Quat::from_rotation_z(-angle);
//             }
//             if angle > PI/2.0 && angle <= PI {
//                 sword_translation.rotation = Quat::from_rotation_z(-angle + PI);
//             }
//         }
//     }
// }

// fn despawn_sword_animation(
//     mut commands: Commands,
//     entity_query: Query<(Entity, &TextureAtlasSprite), With<Sword>>
// ) {
//     for (entity, sprite) in entity_query.iter() {
//         if sprite.index == 4 {
//             commands.entity(entity).despawn();
//         }
//     }
// }

// fn animate_sprite(
//     time: Res<Time>,
//     mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlasSprite)>,
// ) {
//     for (indices, mut timer, mut sprite) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             sprite.index = if sprite.index == indices.last {
//                 indices.first
//             } else {
//                 sprite.index + 1
//             };
//         }
//     }
// }