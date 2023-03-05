use bevy::{input::mouse::MouseScrollUnit, input::mouse::MouseWheel, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod player;
use player::PlayerPlugin;

//other
const BG_COLOR: Color = Color::rgb(0.2, 0.36, 0.89);
#[derive(Component)]
struct GroundMarker;

#[derive(Component)]
struct CameraMarker;
const CAMERA_ZOOM_SPEED: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .insert_resource(ClearColor(BG_COLOR))
        .add_startup_system(setup)
        .add_event::<MouseWheel>()
        .add_system(camera_movement)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        CameraMarker,
        Name::new("Camera2D"),
    ));

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
}

fn camera_movement(
    mut camera_query: Query<&mut Transform, With<CameraMarker>>,
    mut scroll_ev: EventReader<MouseWheel>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = camera_query.single_mut();
    for ev in scroll_ev.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if keys.pressed(KeyCode::Key1){
                    transform.scale.x -= time.delta_seconds() * CAMERA_ZOOM_SPEED * ev.y;
                    transform.scale.y -= time.delta_seconds() * CAMERA_ZOOM_SPEED * ev.y;
                }
            }
            MouseScrollUnit::Pixel => println!("jfsalkjflksadjfklasjfklasdjfklda"),
        }
    }
}
