use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use std::env;

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
    cube0: Handle<Image>,
    cube1: Handle<Image>,
    blocks: [Handle<Image>; 3],
    font_roboto_black: Handle<Font>,
}

#[derive(Resource)]
pub struct GameState {
    variant: GameStateVariant,
}

static mut GAME_STATE_EARLY: i32 = 0; //setting up the game state early via
                                      //cla and then changing the actual GameState based on this value

//1- game
//2- editor

#[derive(Eq, PartialEq)]
pub enum GameStateVariant {
    Editor, //you can move the camera, the player does not exist
    Level,  //you can control the player but no the camera
}

fn main() {
    match env::args().nth(1) {
        Some(input) if input == "game" => {
            println!("You entered 'game");
            unsafe {
                GAME_STATE_EARLY = 1;
            }
        }
        Some(input) if input == "editor" => {
            println!("You entered 'editor'.");
            unsafe {
                GAME_STATE_EARLY = 2;
            }
        }
        Some(input) => {
            println!(
                "You entered '{}', but I was expecting either 'game' or 'editor'.",
                input
            );
        }
        None => {
            println!("Please provide an argument.");
        }
    }

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
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn asset_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cube0 = asset_server.load("cube0.png");
    let cube1 = asset_server.load("cube1.png");
    let block0 = asset_server.load("block0.png");
    let block1 = asset_server.load("block1.png");
    let block2 = asset_server.load("block2.png");

    let font_roboto_black = asset_server.load("fonts/Roboto-Black.ttf");

    commands.insert_resource(GameAssets {
        font_roboto_black,
        cube0,
        cube1,
        blocks: [block0, block1, block2],
    });
}

fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    //spikes
    /*
    for i in 0..3 {
        commands
            .spawn(SpriteBundle {
                texture: game_assets.blocks[0].clone(),
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
    */

    //ground
    commands
        .spawn(SpriteBundle {
            texture: game_assets.blocks[2].clone(),
            ..default()
        })
        .insert(Transform {
            translation: Vec3::new(0.0, -320.0, 0.0),
            scale: Vec3::new(500.0, 2.0, 1.0),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(150.0, 15.0))
        .insert(GroundMarker)
        .insert(Name::new("Ground"));

    commands.insert_resource(GameState {
        variant: unsafe {
            match GAME_STATE_EARLY {
                1 => GameStateVariant::Level,
                _ => GameStateVariant::Editor,
            }
        },
    });

    commands.insert_resource(EditorState {
        active: false,
        picked_block_id: 0,
        freeze_block_placing: false,
    });
    commands.insert_resource(LevelState {
        attempts: 0,
        active: false,
    });
}
