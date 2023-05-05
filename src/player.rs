use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{
    physics::Velocity,
    tee::{TeeBundle, TeeBundleChildren},
    MainCamera,
};

#[derive(Component)]
pub struct Player;

pub fn add_player(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<Image> = server.load("skins/default.png");

    let tee_bundle = TeeBundle::new("Player", Vec3::new(32.0, 32.0, 0.0));

    let tee_bundle_children = TeeBundleChildren::new(handle);

    commands
        .spawn((Player, tee_bundle, Velocity::new(100.0)))
        .with_children(|parent| {
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
    mut query_player: Query<&mut Transform, With<Player>>,
    query_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = query_camera.single();

    for ev in cursor_evr.iter() {
        let real_pos = camera.viewport_to_world_2d(camera_transform, ev.position);
        if let Some(real_pos) = real_pos {
            let mut transform = query_player.single_mut();
            let translation = transform.translation.xy();
            let vector = (real_pos - translation).normalize();

            let rotate_to_mouse = Quat::from_rotation_arc(Vec3::Y, vector.extend(0.0));

            transform.rotation = rotate_to_mouse;
        }
    }
}
