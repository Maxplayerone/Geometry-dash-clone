use bevy::{input::mouse::MouseScrollUnit, input::mouse::MouseWheel, prelude::*};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy_rapier2d::prelude::*;
//use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::{GameAssets, GameState, GameStateVariant, GroundMarker};

#[derive(Component)]
struct EditorCameraMarker;
#[derive(Component)]
struct NodeForBlockPlacingButtonsMarker;

#[derive(Component)]
struct BlockButton {
    id: u8,
}

const CAMERA_ZOOM_SPEED: f32 = 10.0;
const BLOCK_SIZE: f32 = 64.0;

#[derive(Resource)]
pub struct EditorState {
    pub active: bool,
    pub picked_block_id: u8,
    pub freeze_block_placing: bool,
}

//used once when transitioning from level to editor
fn editor_open(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut editor_state: ResMut<EditorState>,
    game_assets: Res<GameAssets>,
) {
    if game_state.variant == GameStateVariant::Editor && !editor_state.active {
        commands.spawn((
            Camera2dBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            },
            EditorCameraMarker,
            Name::new("EditorCamera"),
        ));

        spawn_button(commands, game_assets, game_state);
        editor_state.active = true;
        editor_state.freeze_block_placing = false;
    }
}

fn editor_close(
    mut commands: Commands,
    camera_query: Query<Entity, With<EditorCameraMarker>>,
    node_query: Query<
        Entity,
        (
            With<NodeForBlockPlacingButtonsMarker>,
            Without<EditorCameraMarker>,
        ),
    >,
    game_state: Res<GameState>,
    mut editor_state: ResMut<EditorState>,
) {
    if game_state.variant == GameStateVariant::Level && editor_state.active {
        for entity in camera_query.iter() {
            for node_entity in node_query.iter() {
                commands.entity(entity).despawn();
                commands.entity(node_entity).despawn_recursive();
                editor_state.active = false;
            }
        }
    }
}

fn camera_zoom(
    mut camera_query: Query<&mut Transform, With<EditorCameraMarker>>,
    mut scroll_ev: EventReader<MouseWheel>,
    time: Res<Time>,
    game_state: Res<GameState>,
    editor_state: Res<EditorState>,
) {
    if game_state.variant == GameStateVariant::Editor && editor_state.active {
        for mut transform in camera_query.iter_mut() {
            for ev in scroll_ev.iter() {
                match ev.unit {
                    MouseScrollUnit::Line => {
                        transform.scale.x -= time.delta_seconds() * CAMERA_ZOOM_SPEED * ev.y;
                        transform.scale.y -= time.delta_seconds() * CAMERA_ZOOM_SPEED * ev.y;
                    }
                    MouseScrollUnit::Pixel => println!("jfsalkjflksadjfklasjfklasdjfklda"),
                }
            }
        }
    }
}

fn camera_movement(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<EditorCameraMarker>>,
    time: Res<Time>,
    game_state: Res<GameState>,
    editor_state: Res<EditorState>,
) {
    if game_state.variant == GameStateVariant::Editor && editor_state.active {
        for mut transform in camera_query.iter_mut() {
            let mut left = transform.left();
            left = left.normalize();

            let speed = 1000.0;

            if keyboard.pressed(KeyCode::A) {
                transform.translation += left * time.delta_seconds() * speed;
            }
            if keyboard.pressed(KeyCode::D) {
                transform.translation -= left * time.delta_seconds() * speed;
            }
            if keyboard.pressed(KeyCode::S) {
                transform.translation -= Vec3::Y * time.delta_seconds() * speed;
            }
            if keyboard.pressed(KeyCode::W) {
                transform.translation += Vec3::Y * time.delta_seconds() * speed;
            }
        }
    }
}

fn draw_editor_lines(
    mut lines: ResMut<DebugLines>,
    camera_query: Query<&Transform, With<EditorCameraMarker>>,
) {
    for transform in camera_query.iter() {
        if transform.scale.x < 2.0 {
            let offset = 32.0;
            for i in 0..50 {
                lines.line_gradient(
                    Vec3::new(-1000.0, (i as f32 * BLOCK_SIZE + -1280.0) + offset, 0.0),
                    Vec3::new(1000.0, (i as f32 * BLOCK_SIZE + -1280.0) + offset, 0.0),
                    0.0,
                    Color::WHITE,
                    Color::PINK,
                );
            }

            for i in 0..50 {
                lines.line_gradient(
                    Vec3::new((i as f32 * BLOCK_SIZE + -1280.0) + offset, -1000.0, 0.0),
                    Vec3::new((i as f32 * BLOCK_SIZE + -1280.0) + offset, 1000.0, 0.0),
                    0.0,
                    Color::WHITE,
                    Color::PINK,
                );
            }
        }
    }
}

fn place_blocks(
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    game_assets: Res<GameAssets>,
    mut commands: Commands,
    editor_state: Res<EditorState>,
    camera_query: Query<(&Camera, &GlobalTransform), With<EditorCameraMarker>>,
) {
    let window = windows.get_primary().unwrap();

    if editor_state.freeze_block_placing == true {
        return;
    }

    if buttons.just_pressed(MouseButton::Left) {
        for (camera, camera_transform) in camera_query.iter() {
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                let mut pos = Vec3::new(0.0, 0.0, 0.0);
                let r = world_position.x as i32 % BLOCK_SIZE as i32;
                if r < (BLOCK_SIZE / 2.0) as i32 {
                    pos.x = world_position.x - r as f32;
                } else {
                    pos.x = world_position.x + (BLOCK_SIZE - r as f32);
                }

                let r = world_position.y as i32 % BLOCK_SIZE as i32;
                if r < (BLOCK_SIZE / 2.0) as i32 {
                    pos.y = world_position.y - r as f32;
                } else {
                    pos.y = world_position.y + (BLOCK_SIZE - r as f32);
                }

                commands
                    .spawn(SpriteBundle {
                        texture: game_assets.blocks[editor_state.picked_block_id as usize].clone(),
                        ..default()
                    })
                    .insert(Transform {
                        translation: pos,
                        scale: Vec3::new(2.0, 2.0, 1.0),
                        ..default()
                    })
                    .insert(RigidBody::Fixed)
                    .insert(Collider::cuboid(15.0, 15.0))
                    .insert(GroundMarker);
            }
        }
    }
}

fn button_clicked(
    interaction: Query<(&Interaction, &BlockButton), Changed<Interaction>>,
    mut editor_state: ResMut<EditorState>,
) {
    for (interaction, button) in &interaction {
        if matches!(interaction, Interaction::Clicked) {
            editor_state.picked_block_id = button.id;
        } else if matches!(interaction, Interaction::Hovered) {
            editor_state.freeze_block_placing = true;
        } else {
            editor_state.freeze_block_placing = false;
        }
    }
}

fn spawn_button(mut commands: Commands, game_assets: Res<GameAssets>, game_state: Res<GameState>) {
    if game_state.variant == GameStateVariant::Editor {
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .insert(Name::new("Node"))
            .insert(NodeForBlockPlacingButtonsMarker)
            .with_children(|commands| {
                for i in 0..3 {
                    commands
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Percent(15.0 * 9.0 / 16.0),
                                    Val::Percent(15.0),
                                ),
                                align_self: AlignSelf::FlexEnd,
                                margin: UiRect::all(Val::Percent(2.0)),
                                ..default()
                            },
                            image: game_assets.blocks[i as usize].clone().into(),
                            ..default()
                        })
                        .insert(BlockButton { id: i })
                        .insert(Name::new("Button"));
                }
            });
    }
}

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DebugLinesPlugin::default())
            .add_event::<MouseWheel>()
            .add_system(editor_open)
            .add_system(editor_close)
            .add_system(camera_movement)
            .add_system(camera_zoom)
            .add_system(place_blocks)
            //.add_system(draw_editor_lines)
            .add_system(button_clicked.after(place_blocks));
    }
}
