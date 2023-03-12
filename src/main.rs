use bevy::{
    input::mouse::MouseScrollUnit, 
    input::mouse::MouseWheel, 
    prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod player;
use player::{PlayerPlugin, RespawnPlayerEvent};

//markers
#[derive(Component)]
struct GroundMarker;
#[derive(Component)]
struct CameraMarker;
#[derive(Component)]
struct SpikeMarker;
#[derive(Component)]
struct AttemptsTextMarker;

//others
const BG_COLOR: Color = Color::rgb(0.2, 0.36, 0.89);
const CAMERA_ZOOM_SPEED: f32 = 50.0;

#[derive(Resource)]
struct GameAssets {
    texture_atlas: Handle<TextureAtlas>,
    font_roboto_black: Handle<Font>,
}

//probably will move attemps to another state
#[derive(Resource)]
struct LevelState{
    attempts: u32,
}

#[derive(Resource)]
pub enum GameState{
    Editor,
    Level,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin {
            mode: DebugRenderMode::empty(),
            ..default()
        })
        .add_plugin(PlayerPlugin)
        .insert_resource(ClearColor(BG_COLOR))
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_startup_system(setup)
        .add_event::<MouseWheel>()
        .add_system(camera_movement)
        .add_system(camera_zoom)
        .add_system(update_attemps_text)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn asset_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("gj_sheet0.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 8, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let font_roboto_black = asset_server.load("fonts/Roboto-Black.ttf");

    commands.insert_resource(GameAssets {
        texture_atlas: texture_atlas_handle,
        font_roboto_black,
    });
}

fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(-300.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        CameraMarker,
        Name::new("Camera2D"),
    ));

    //spikes
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite::new(8),
            ..default()
        })
        .insert(Transform {
            translation: Vec3::new(0.0, -260.0, 0.0),
            scale: Vec3::new(2.0, 2.0, 1.0),
            ..default()
        })
        .insert(SpikeMarker)
        .insert(Collider::cuboid(4.0, 10.0))
        .insert(Name::new("Spike01"));

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite::new(8),
            ..default()
        })
        .insert(Transform {
            translation: Vec3::new(64.0, -260.0, 0.0),
            scale: Vec3::new(2.0, 2.0, 1.0),
            ..default()
        })
        .insert(SpikeMarker)
        .insert(Collider::cuboid(4.0, 10.0))
        .insert(Name::new("Spike02"));

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite::new(8),
            ..default()
        })
        .insert(Transform {
            translation: Vec3::new(128.0, -260.0, 0.0),
            scale: Vec3::new(2.0, 2.0, 1.0),
            ..default()
        })
        .insert(SpikeMarker)
        .insert(Collider::cuboid(4.0, 10.0))
        .insert(Name::new("Spike03"));

    //ground
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Transform {
            translation: Vec3::new(0.0, -325.0, 0.0),
            scale: Vec3::new(5000.0, 70.0, 1.0),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(GroundMarker)
        .insert(Name::new("Ground"));

    let text_style = TextStyle {
        font: game_assets.font_roboto_black.clone(),
        font_size: 33.5,
        color: Color::WHITE,
    };

    commands.spawn(Text2dBundle {
        text: Text::from_section("Attempts 0", text_style.clone()),
        ..default()
    })
    .insert(Transform {
        translation: Vec3::new(-660.0, -155.0, 0.0),
        ..default()
    })
    .insert(AttemptsTextMarker);

    commands.insert_resource(LevelState{attempts: 0});
}

fn update_attemps_text(
    mut respawn_player_ev: EventReader<RespawnPlayerEvent>,
    mut state: ResMut<LevelState>,
    mut attempts_text: Query<&mut Text, With<AttemptsTextMarker>>, 
    game_assets: Res<GameAssets>,
){
    for _ in respawn_player_ev.iter(){
        for mut text in attempts_text.iter_mut(){
            state.attempts += 1;

            let text_style = TextStyle {
                font: game_assets.font_roboto_black.clone(),
                font_size: 33.5,
                color: Color::WHITE,
            };

            *text = Text::from_section(
                format!("Attempts: {}", state.attempts),
                text_style.clone(),
            );
        }
    }
}

/*
fn toggle_game_state(
    keys: Res<Input<KeyCode>>,
    mut player_settings: ResMut<PlayerSettings>,
) {
    if keys.pressed(KeyCode::Key2) {
        player_settings.freeze_movement = true;
    }

    if keys.pressed(KeyCode::Key1) {
        player_settings.freeze_movement = false;
    }
}
*/

fn camera_zoom(
    mut camera_query: Query<&mut Transform, With<CameraMarker>>,
    mut scroll_ev: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    let mut transform = camera_query.single_mut();
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

fn camera_movement(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<CameraMarker>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();
    let mut left = camera.left();
    left = left.normalize();

    let speed = 1000.0;

    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= Vec3::Y * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::W) {
        camera.translation += Vec3::Y * time.delta_seconds() * speed;
    }
}
