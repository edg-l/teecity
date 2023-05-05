use bevy::prelude::*;

#[derive(Debug, Reflect, Component, Default, Clone)]
pub struct Velocity {
    pub vel: Vec2,
    pub speed: f32,
}

impl Velocity {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            ..Default::default()
        }
    }
}

#[derive(Component)]
pub struct BoundingBox {
    pub rect: Rect,
}

pub fn move_system(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (vel, mut transform) in query.iter_mut() {
        transform.translation += vel.vel.extend(0.0) * time.delta_seconds();
    }
}
