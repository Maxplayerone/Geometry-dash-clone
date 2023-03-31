use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{GameAssets, GameState, GameStateVariant, GroundMarker};

#[derive(Component)]
struct PlayerMarker;
#[derive(Component)]
struct LevelCameraMarker;
#[derive(Component)]
struct AttemptsTextMarker;
#[derive(Component)]
struct SpikeMarker;
#[derive(Component)]
struct ClippedBlockMarker;

#[derive(Component)]
enum BlockType {
    Spike(SpikeMarker),
    Ground(GroundMarker),
    Clipped(ClippedBlockMarker),
}

const PLAYER_JUMP_VALUE: f32 = 900.0;
const PLAYER_SPEED: f32 = 450.0;
const PLAYER_ROTATION_SPEED: f32 = 5.0;

pub const STARTING_PLAYER_POSTION: Vec3 = Vec3::new(-600.0, -220.0, 0.0);
pub const STARTING_CAMERA_POSTION: Vec3 = Vec3::new(-300.0, 0.0, 0.0);

#[derive(Component)]
struct Jump {
    value: f32,
    is_jumping: bool,
    rotation_value: f32,
}

#[derive(Resource)]
pub struct LevelState {
    pub attempts: u32,
    pub active: bool,
}

#[derive(Debug, Deserialize)]
pub struct BlockInfo {
    id: u8,
    name: String,
    marker_type: u8, //0- spike, 1- block, 2- ClippedBlockMarker
    coords: (i32, i32),
}

#[derive(Default)]
pub struct RespawnPlayerEvent;

fn level_open(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    game_state: Res<GameState>,
    mut level_state: ResMut<LevelState>,
) {
    if game_state.variant == GameStateVariant::Level && level_state.active == false {
        let file = File::open("test_map.json").unwrap();
        let reader = BufReader::new(file);
        let block_info_vec: Vec<BlockInfo> = serde_json::from_reader(reader).unwrap();

        for block_info in block_info_vec {
            let block_type = match block_info.marker_type {
                0 => BlockType::Spike(SpikeMarker),
                1 => BlockType::Ground(GroundMarker),
                _ => BlockType::Clipped(ClippedBlockMarker),
            };

            commands
                .spawn(SpriteBundle {
                    texture: game_assets.blocks[block_info.id as usize].clone(),
                    ..default()
                })
                .insert(Transform {
                    translation: Vec3::new(block_info.coords.0 as f32, block_info.coords.1 as f32, 0.0),
                    scale: Vec3::new(2.0, 2.0, 1.0),
                    ..default()
                })
                .insert(block_type)
                .insert(Collider::cuboid(4.0, 10.0))
                .insert(Name::new(block_info.name));
        }

        commands
            .spawn(SpriteBundle {
                texture: game_assets.cube0.clone(),
                ..default()
            })
            .insert(Transform {
                translation: STARTING_PLAYER_POSTION,
                scale: Vec3::new(2.0, 2.0, 1.0),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(GravityScale(31.0))
            .insert(Velocity {
                linvel: Vec2::new(10.0, 0.0),
                angvel: 0.0,
            })
            .insert(Collider::cuboid(15.0, 15.0))
            .insert(Jump {
                value: PLAYER_JUMP_VALUE,
                is_jumping: true,
                rotation_value: 0.0,
            })
            .insert(PlayerMarker)
            .insert(Name::new("Player"));

        commands.spawn((
            Camera2dBundle {
                transform: Transform {
                    translation: STARTING_CAMERA_POSTION,
                    ..default()
                },
                ..default()
            },
            LevelCameraMarker,
            Name::new("LevelCamera"),
        ));

        let text_style = TextStyle {
            font: game_assets.font_roboto_black.clone(),
            font_size: 33.5,
            color: Color::WHITE,
        };

        commands
            .spawn(Text2dBundle {
                text: Text::from_section("Attempts 0", text_style.clone()),
                ..default()
            })
            .insert(Transform {
                translation: Vec3::new(-660.0, -155.0, 0.0),
                ..default()
            })
            .insert(AttemptsTextMarker);

        level_state.active = true;
    }
}

fn level_close(
    mut commands: Commands,
    player_query: Query<Entity, (With<PlayerMarker>, Without<LevelCameraMarker>)>,
    camera_query: Query<Entity, With<LevelCameraMarker>>,
    game_state: Res<GameState>,
    mut level_state: ResMut<LevelState>,
) {
    if game_state.variant == GameStateVariant::Editor && level_state.active == true {
        for player_id in player_query.iter() {
            for camera_id in camera_query.iter() {
                commands.entity(camera_id).despawn();
                commands.entity(player_id).despawn();
                level_state.active = false;
            }
        }
    }
}

fn update_attemps_text(
    mut respawn_player_ev: EventReader<RespawnPlayerEvent>,
    mut level_state: ResMut<LevelState>,
    mut attempts_text: Query<&mut Text, With<AttemptsTextMarker>>,
    game_assets: Res<GameAssets>,
) {
    for _ in respawn_player_ev.iter() {
        for mut text in attempts_text.iter_mut() {
            level_state.attempts += 1;

            let text_style = TextStyle {
                font: game_assets.font_roboto_black.clone(),
                font_size: 33.5,
                color: Color::WHITE,
            };

            *text = Text::from_section(
                format!("Attempts: {}", level_state.attempts),
                text_style.clone(),
            );
        }
    }
}

fn player_movement_linear(
    mut player_query: Query<&mut Transform, (With<PlayerMarker>, Without<LevelCameraMarker>)>,
    mut camera_query: Query<&mut Transform, With<LevelCameraMarker>>,
    time: Res<Time>,
    level_state: Res<LevelState>,
) {
    if level_state.active {
        for mut transform in player_query.iter_mut() {
            for mut camera_transform in camera_query.iter_mut() {
                transform.translation.x += PLAYER_SPEED * time.delta_seconds();
                camera_transform.translation.x += PLAYER_SPEED * time.delta_seconds();
            }
        }
    }
}

fn player_movement_jump(
    mut player_query: Query<(&mut Jump, &mut Velocity), With<PlayerMarker>>,
    keys: Res<Input<KeyCode>>,
    level_state: Res<LevelState>,
) {
    if level_state.active {
        for (mut jump, mut velocity) in player_query.iter_mut() {
            if keys.pressed(KeyCode::Up) && !jump.is_jumping {
                velocity.linvel = Vec2::new(0.0, jump.value).into();
                jump.is_jumping = true;
            }
        }
    }
}

fn player_jump_animation(
    mut player_query: Query<(&mut Jump, &mut Transform), With<PlayerMarker>>,
    time: Res<Time>,
) {
    for (mut jump, mut transform) in player_query.iter_mut() {
        if jump.is_jumping {
            let rotation_value = 45.0 * time.delta_seconds() * PLAYER_ROTATION_SPEED * -1.0;
            jump.rotation_value += rotation_value;
            transform.rotation = Quat::from_rotation_z(jump.rotation_value * 3.1415 / 180.0 as f32);
        }
    }
}

fn ceil_to_full_rotation(rotation_value: f32) -> f32 {
    let mut inum = rotation_value as i32;
    inum = inum - 90 - (inum % 90);
    let fnum = inum as f32;
    fnum
}

fn reset_player_jump(
    mut player_query: Query<
        (Entity, &mut Jump, &mut Transform),
        (With<PlayerMarker>, Without<GroundMarker>),
    >,
    ground_query: Query<Entity, With<GroundMarker>>,
    rapier_context: Res<RapierContext>,
) {
    for ground_id in ground_query.iter() {
        for (player_id, mut jump, mut transform) in player_query.iter_mut() {
            if let Some(_contact_pair) = rapier_context.contact_pair(player_id, ground_id) {
                jump.is_jumping = false;
                transform.rotation = Quat::from_rotation_z(
                    ceil_to_full_rotation(jump.rotation_value) * 3.1415 / 180.0 as f32,
                );
            }
        }
    }
}

fn player_death(
    mut player_query: Query<Entity, (With<PlayerMarker>, Without<SpikeMarker>)>,
    mut spike_queries: Query<Entity, With<SpikeMarker>>,
    rapier_context: Res<RapierContext>,
    mut respawn_player_ev: EventWriter<RespawnPlayerEvent>,
) {
    for player_id in player_query.iter_mut() {
        for spike_id in spike_queries.iter_mut() {
            if let Some(_contact_pair) = rapier_context.contact_pair(player_id, spike_id) {
                respawn_player_ev.send_default();
            }
        }
    }
}

fn reset_player_state(
    mut respawn_player_ev: EventReader<RespawnPlayerEvent>,
    mut player_query: Query<&mut Transform, (With<PlayerMarker>, Without<LevelCameraMarker>)>,
    mut camera_query: Query<&mut Transform, With<LevelCameraMarker>>,
) {
    for _ in respawn_player_ev.iter() {
        for mut player_transform in player_query.iter_mut() {
            for mut camera_transform in camera_query.iter_mut() {
                player_transform.translation = STARTING_PLAYER_POSTION;
                camera_transform.translation = STARTING_CAMERA_POSTION;
            }
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RespawnPlayerEvent>()
            .add_system(level_open)
            .add_system(level_close)
            .add_system(player_movement_linear)
            .add_system(player_movement_jump)
            .add_system(player_jump_animation.before(reset_player_jump))
            .add_system(reset_player_jump)
            .add_system(player_death)
            .add_system(update_attemps_text)
            .add_system(reset_player_state);
    }
}
