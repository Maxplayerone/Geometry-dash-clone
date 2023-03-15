use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod player;
use player::{LevelState, PlayerPlugin};

mod editor;
use editor::{EditorPlugin, EditorState};

//markers
#[derive(Component)]
struct GroundMarker;
#[derive(Component)]
struct SpikeMarker;

//others
const BG_COLOR: Color = Color::rgb(0.2, 0.36, 0.89);
pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Resource)]
struct GameAssets {
    texture_atlas: Handle<TextureAtlas>,
    font_roboto_black: Handle<Font>,
}

#[derive(Resource)]
pub struct GameState {
    variant: GameStateVariant,
}

#[derive(Eq, PartialEq)]
pub enum GameStateVariant {
    Editor, //you can move the camera, the player does not exist
    Level,  //you can control the player but no the camera
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: WIDTH,
                        height: HEIGHT,
                        title: "Bevy Tower Defense".to_string(),
                        resizable: false,
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin {
            mode: DebugRenderMode::empty(),
            ..default()
        })
        .add_plugin(PlayerPlugin)
        .add_plugin(EditorPlugin)
        .insert_resource(ClearColor(BG_COLOR))
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_startup_system(setup)
        //.add_system(update_attemps_text)
        .add_system(toggle_game_state)
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
    //spikes
    for i in 0..3 {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: game_assets.texture_atlas.clone(),
                sprite: TextureAtlasSprite::new(8),
                ..default()
            })
            .insert(Transform {
                translation: Vec3::new(i as f32 * 64.0, -256.0, 0.0),
                scale: Vec3::new(2.0, 2.0, 1.0),
                ..default()
            })
            .insert(SpikeMarker)
            .insert(Collider::cuboid(4.0, 10.0))
            .insert(Name::new("Spike01"));
    }

    //ground
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite::new(10),
            ..default()
        })
        .insert(Transform {
            translation: Vec3::new(0.0, -320.0, 0.0),
            scale: Vec3::new(500.0, 2.0, 1.0),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(GroundMarker)
        .insert(Name::new("Ground"));

    commands.insert_resource(GameState {
        variant: GameStateVariant::Editor,
    });

    commands.insert_resource(EditorState {
        active: false,
        picked_block_id: 0,
    });
    commands.insert_resource(LevelState {
        attempts: 0,
        active: false,
    });
}

fn toggle_game_state(keys: Res<Input<KeyCode>>, mut game_state: ResMut<GameState>) {
    if keys.just_pressed(KeyCode::Key1) {
        game_state.variant = GameStateVariant::Level;
        println!("In level");
    }

    if keys.just_pressed(KeyCode::Key2) {
        game_state.variant = GameStateVariant::Editor;
        println!("In editor");
    }
}
