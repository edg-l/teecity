use bevy::{math::Vec3Swizzles, prelude::*, window::PrimaryWindow, winit::WinitWindows};

use crate::{
    misc::AimTarget,
    physics::Velocity,
    tee::{TeeBundle, TeeBundleChildren},
    MainCamera,
};

#[derive(Component)]
pub struct Player;

pub fn add_player(mut commands: Commands, server: Res<AssetServer>) {
    let skin_handle: Handle<Image> = server.load("skins/default.png");
    let game_handle: Handle<Image> = server.load("game.png");

    let tee_bundle = TeeBundle::new("Player", Vec3::new(32.0, 32.0, 1.0));

    let tee_bundle_children = TeeBundleChildren::new(skin_handle, game_handle, 1.0);

    commands
        .spawn((
            Player,
            tee_bundle,
            Velocity::new(400.0),
            AimTarget::default(),
        ))
        .with_children(|parent| {
            parent.spawn(tee_bundle_children.weapon);
            parent.spawn(tee_bundle_children.body);
            parent.spawn(tee_bundle_children.left_foot);
            parent.spawn(tee_bundle_children.right_foot);
        });
}

pub fn player_input(
    keys: Res<Input<KeyCode>>,
    mut query_player: Query<&mut Velocity, With<Player>>,
) {
    let mut dir = Vec2::ZERO;

    if keys.pressed(KeyCode::W) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::S) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        dir.x += 1.0;
    }
    if keys.pressed(KeyCode::A) {
        dir.x -= 1.0;
    }

    let mut v = query_player.single_mut();
    v.vel = dir.normalize_or_zero() * v.speed;
}

pub fn player_mouse(
    mut cursor_evr: EventReader<CursorMoved>,
    mut query_player: Query<&mut AimTarget, With<Player>>,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = query_camera.single();
    let mut target = query_player.single_mut();

    for ev in cursor_evr.iter() {
        let real_pos = camera.viewport_to_world_2d(camera_transform, ev.position);
        if let Some(real_pos) = real_pos {
            target.0 = Some(real_pos);
        }
    }
}

pub fn player_camera(
    mut query_player: Query<(&mut AimTarget, &Transform), (With<Player>, Without<MainCamera>)>,
    mut query_camera: Query<&mut Transform, With<MainCamera>>,
) {
    let mut camera_transform = query_camera.single_mut();
    let (mut aim_target, player_transform) = query_player.single_mut();

    let old = camera_transform.translation;

    camera_transform.translation = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        camera_transform.translation.z,
    );

    // Keep the aim same when camera is moving
    let diff = camera_transform.translation - old;

    if diff.length() > 0.0 {
        if let Some(t) = aim_target.0.as_mut() {
            *t += diff.xy();
        }
    }
}
