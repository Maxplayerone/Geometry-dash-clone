use crate::{GameState, GameStateVariant};
use bevy::{input::mouse::MouseScrollUnit, input::mouse::MouseWheel, prelude::*};
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

#[derive(Component)]
struct EditorCameraMarker;

const CAMERA_ZOOM_SPEED: f32 = 10.0;
const BLOCK_SIZE: f32 = 64.0;

#[derive(Resource)]
pub struct EditorState {
    pub active: bool,
}

//used once when transitioning from level to editor
fn editor_open(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut editor_state: ResMut<EditorState>,
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
        editor_state.active = true;
    }
}

fn editor_close(
    mut commands: Commands,
    camera_query: Query<Entity, With<EditorCameraMarker>>,
    game_state: Res<GameState>,
    mut editor_state: ResMut<EditorState>,
) {
    if game_state.variant == GameStateVariant::Level && editor_state.active {
        for entity in camera_query.iter() {
            commands.entity(entity).despawn();
            editor_state.active = false;
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

fn draw_editor_lines(mut lines: ResMut<DebugLines>) {
    let offset = 32.0;
    for i in 0..30 {
        lines.line_gradient(
            Vec3::new(-10000.0, (i as f32 * BLOCK_SIZE + -1280.0) + offset, 0.0),
            Vec3::new(10000.0, (i as f32 * BLOCK_SIZE + -1280.0) + offset, 0.0),
            0.0,
            Color::WHITE,
            Color::PINK,
        );
    }

    for i in 0..30 {
        lines.line_gradient(
            Vec3::new((i as f32 * BLOCK_SIZE + -1280.0) + offset, -1000.0, 0.0),
            Vec3::new((i as f32 * BLOCK_SIZE + -1280.0) + offset, 1000.0, 0.0),
            0.0,
            Color::WHITE,
            Color::PINK,
        );
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
            .add_system(draw_editor_lines);
    }
}
