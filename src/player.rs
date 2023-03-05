use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

//player
#[derive(Component)]
struct PlayerMarker;

#[derive(Component)]
struct Jump{
    value: f32,
    is_jumping: bool,
    rotation_value: f32,
}

const PLAYER_SPEED: f32 = 300.0;
const PLAYER_ROTATION_SPEED: f32 = 5.0;

use crate::CameraMarker;
use crate::GroundMarker;

fn player_spawn(
    mut commands: Commands
){
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
     .insert(GravityScale(30.0))
     .insert(Velocity {
         linvel: Vec2::new(10.0, 0.0),
         angvel: 0.0,
     })
     .insert(Collider::cuboid(0.5, 0.5))
     .insert(Jump {value: 800.0, is_jumping: true, rotation_value: 0.0})
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

        if keys.pressed(KeyCode::Up) && !jump.is_jumping{

            velocity.linvel = Vec2::new(0.0, jump.value).into();
            jump.is_jumping = true;
        }
    }
}

fn player_jump_animation(
    mut player_query: Query<(&mut Jump, &mut Transform), With<PlayerMarker>>,
    time: Res<Time>,
){
    let (mut jump, mut transform) = player_query.single_mut();

        if jump.is_jumping{
            let rotation_value = 45.0 * time.delta_seconds() * PLAYER_ROTATION_SPEED * -1.0;
            jump.rotation_value += rotation_value;
            transform.rotation =  Quat::from_rotation_z(jump.rotation_value * 3.1415 / 180.0 as f32);
        }
}

fn ceil_to_full_rotation(rotation_value: f32) -> f32{
    let mut inum = rotation_value as i32;
    inum = inum - 90 - (inum % 90);
    let fnum = inum as f32;
    fnum
}

fn reset_player_jump(
    mut player_query: Query<(Entity, &mut Jump, &mut Transform), (With<PlayerMarker>, Without<GroundMarker>)>,
    mut ground_query: Query<Entity, With<GroundMarker>>,
    rapier_context: Res<RapierContext>,
){
    let (player_id, mut jump, mut transform) = player_query.single_mut();

    let ground_id = ground_query.single_mut();

    if let Some(_contact_pair) = rapier_context.contact_pair(player_id, ground_id) {
        jump.is_jumping = false;
        transform.rotation =  Quat::from_rotation_z(ceil_to_full_rotation(jump.rotation_value) * 3.1415 / 180.0 as f32);
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app
        .add_startup_system(player_spawn)
        .add_system(player_movement_linear)
        .add_system(player_movement_jump)
        .add_system(player_jump_animation.before(reset_player_jump))
        .add_system(reset_player_jump);
    }
}