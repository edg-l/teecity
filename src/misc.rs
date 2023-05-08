use bevy::{math::Vec3Swizzles, prelude::*};

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct AimTarget(pub Option<Vec2>);

pub fn aim_target_system(mut query: Query<(&mut Transform, &AimTarget)>) {
    for (mut t, target) in query.iter_mut().filter(|(_, b)| b.0.is_some()) {
        if let Some(target) = target.0 {
            let translation = t.translation.xy();
            let vector = (target - translation).normalize();
            let rotate_to_target = Quat::from_rotation_arc(Vec3::Y, vector.extend(0.0));
            t.rotation = rotate_to_target;
        }
    }
}
