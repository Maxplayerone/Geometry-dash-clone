use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::CameraMarker;
use crate::GroundMarker;
use crate::GameAssets;
use crate::SpikeMarker;

#[derive(Component)]
struct PlayerMarker;

#[derive(Component)]
struct Jump {
    value: f32,
    is_jumping: bool,
    rotation_value: f32,
}

const PLAYER_SPEED: f32 = 450.0;
const PLAYER_ROTATION_SPEED: f32 = 5.0;
const PLAYER_JUMP_VALUE: f32 = 900.0;

const STARTING_POSTION: Vec3 = Vec3::new(-600.0, -220.0, 0.0);

//flag for freezing player movement
//(used for debugging)
#[derive(Resource)]
struct PlayerSettings {
    freeze_movement: bool,
}

fn player_spawn(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: game_assets.texture_atlas.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..default()
        })
        .insert(Transform {
            translation: STARTING_POSTION,
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
        .insert(Collider::cuboid(13.0, 13.0))
        .insert(Jump {
            value: PLAYER_JUMP_VALUE,
            is_jumping: true,
            rotation_value: 0.0,
        })
        .insert(PlayerMarker)
        .insert(Name::new("Player"));

    commands.insert_resource(PlayerSettings { freeze_movement: false });
}

fn toggle_player_movement_state(
    keys: Res<Input<KeyCode>>,
    mut player_settings: ResMut<PlayerSettings>,
){
    if keys.pressed(KeyCode::Key2) {
        player_settings.freeze_movement = !player_settings.freeze_movement;
    }
}

fn player_movement_linear(
    mut player_query: Query<&mut Transform, (With<PlayerMarker>, Without<CameraMarker>)>,
    mut camera_query: Query<&mut Transform, With<CameraMarker>>,
    time: Res<Time>,
    player_settings: Res<PlayerSettings>,
) {
    if !player_settings.freeze_movement {
        for mut transform in player_query.iter_mut(){
            //let mut transform = player_query.single_mut();
            let mut camera_transform = camera_query.single_mut();

            transform.translation.x += PLAYER_SPEED * time.delta_seconds();
            camera_transform.translation.x += PLAYER_SPEED * time.delta_seconds();
        }
    }
}

fn player_movement_jump(
    mut player_query: Query<(&mut Jump, &mut Velocity), With<PlayerMarker>>,
    keys: Res<Input<KeyCode>>,
    player_settings: Res<PlayerSettings>,
) {
    if !player_settings.freeze_movement {
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
    //let (mut jump, mut transform) = player_query.single_mut();

    for (mut jump, mut transform) in player_query.iter_mut(){
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
    mut ground_query: Query<Entity, With<GroundMarker>>,
    rapier_context: Res<RapierContext>,
) {
    //let (player_id, mut jump, mut transform) = player_query.single_mut();

    let ground_id = ground_query.single_mut();

    for (player_id, mut jump, mut transform) in player_query.iter_mut(){
        if let Some(_contact_pair) = rapier_context.contact_pair(player_id, ground_id) {
            jump.is_jumping = false;
            transform.rotation = Quat::from_rotation_z(
                ceil_to_full_rotation(jump.rotation_value) * 3.1415 / 180.0 as f32,
            );
        }
    }
}

fn player_death(
    mut player_query: Query<(Entity, &mut Transform), (With<PlayerMarker>, Without<SpikeMarker>)>,
    mut spike_queries: Query<Entity, With<SpikeMarker>>,
    rapier_context: Res<RapierContext>,
    mut player_settings: ResMut<PlayerSettings>,
    mut commands: Commands,
){
    //let (player_id, mut transform) = player_query.single_mut();
    
    for (player_id, transform) in player_query.iter_mut(){
        for spike_id in spike_queries.iter_mut(){
            if let Some(_contact_pair) = rapier_context.contact_pair(player_id, spike_id){
                player_settings.freeze_movement = true;
                commands.entity(player_id).despawn();
                println!("Death");
            }
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_spawn)
            .add_system(player_movement_linear)
            .add_system(player_movement_jump)
            .add_system(player_jump_animation.before(reset_player_jump))
            .add_system(reset_player_jump)
            .add_system(player_death)
            .add_system(toggle_player_movement_state);
    }
}
