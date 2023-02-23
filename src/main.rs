use bevy::{input::mouse::MouseScrollUnit, input::mouse::MouseWheel, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

//other
const BG_COLOR: Color = Color::rgb(0.2, 0.36, 0.89);
#[derive(Component)]
struct GroundMarker;

#[derive(Component)]
struct CameraMarker;
const CAMERA_ZOOM_SPEED: f32 = 50.0;

//player
#[derive(Component)]
struct PlayerMarker;

#[derive(Component)]
struct Jump{
    value: f32,
    is_jumping: bool,
}

const PLAYER_SPEED: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(BG_COLOR))
        .add_startup_system(setup)
        .add_event::<MouseWheel>()
        .add_system(player_movement_linear)
        .add_system(player_movement_jump)
        .add_system(reset_player_jump)
        .add_system(display_events)
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
            scale: Vec3::new(1500.0, 70.0, 1.0),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5))   
        .insert(GroundMarker)
        .insert(Name::new("Ground"));   

    //player
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.49, 1.0, 0.52),
                ..default()
            },
            ..default()
        })
        .insert(Transform {
            translation: Vec3::new(-320.0, -220.0, 0.0),
            scale: Vec3::new(50.0, 50.0, 1.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(10.0))
        .insert(Velocity {
            linvel: Vec2::new(10.0, 0.0),
            angvel: 0.0,
        })
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(Jump {value: 400.0, is_jumping: false})
        .insert(PlayerMarker)
        .insert(Name::new("Player"));
}

fn player_movement_linear(
    mut player_query: Query<&mut Transform, (With<PlayerMarker>, Without<CameraMarker>)>,
    mut camera_query: Query<&mut Transform, With<CameraMarker>>,
    time: Res<Time>,
) {
    let mut transform = player_query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    transform.translation.x += PLAYER_SPEED * time.delta_seconds();
    camera_transform.translation.x += PLAYER_SPEED * time.delta_seconds();
}

fn player_movement_jump(
    mut player_query: Query<(&mut Jump, &mut Velocity), With<PlayerMarker>>,
    keys: Res<Input<KeyCode>>,
){
    for (mut jump, mut velocity) in player_query.iter_mut(){
        if keys.just_pressed(KeyCode::Up) && !jump.is_jumping{
            velocity.linvel = Vec2::new(0.0, jump.value).into();
            jump.is_jumping = true;
        }
    }
}

fn reset_player_jump(
    mut player_query: Query<(Entity, &mut Jump), (With<PlayerMarker>, Without<GroundMarker>)>,
    mut ground_query: Query<Entity, With<GroundMarker>>,
    rapier_context: Res<RapierContext>,
){
    let (player_id, mut jump) = player_query.single_mut();

    let ground_id = ground_query.single_mut();

    if let Some(contact_pair) = rapier_context.contact_pair(player_id, ground_id) {
       jump.is_jumping = false;
    }
}


fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }
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
