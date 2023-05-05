use std::borrow::Cow;

use bevy::prelude::*;

#[derive(Component)]
pub struct Tee;

#[derive(Component)]
pub struct Body;

#[derive(Component)]
pub struct LeftFoot;

#[derive(Component)]
pub struct RightFoot;

#[derive(Bundle)]
pub struct TeeBundle {
    pub tee: Tee,
    pub name: Name,
    pub spatial_bundle: SpatialBundle,
}

#[derive(Bundle)]
pub struct TeePartBundle<T: Component> {
    pub sprite: SpriteBundle,
    pub marker: T,
    pub name: Name,
}

pub struct TeeBundleChildren {
    pub body: TeePartBundle<Body>,
    pub left_foot: TeePartBundle<LeftFoot>,
    pub right_foot: TeePartBundle<RightFoot>,
}

impl TeeBundle {
    pub fn new(name: impl Into<Cow<'static, str>>, translation: Vec3) -> Self {
        Self {
            tee: Tee,
            name: Name::new(name),
            spatial_bundle: SpatialBundle {
                transform: Transform::from_translation(translation),
                ..Default::default()
            },
        }
    }
}

impl TeeBundleChildren {
    pub fn new(handle: Handle<Image>) -> Self {
        Self {
            body: TeePartBundle {
                sprite: SpriteBundle {
                    texture: handle.clone(),
                    sprite: Sprite {
                        rect: Some(Rect::from_corners(Vec2::ZERO, Vec2::new(96.0, 96.0))),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.5),
                    ..default()
                },
                marker: Body,
                name: Name::new("Body"),
            },
            left_foot: TeePartBundle {
                sprite: SpriteBundle {
                    texture: handle.clone(),
                    sprite: Sprite {
                        rect: Some(Rect::new(96.0 * 2.0, 32.0, 96.0 * 2.0 + 64.0, 32.0 + 32.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(-28.0, -5.0, 0.0)
                        .with_rotation(Quat::from_rotation_z(1.7)),
                    ..default()
                },
                marker: LeftFoot,
                name: Name::new("LeftFoot"),
            },
            right_foot: TeePartBundle {
                sprite: SpriteBundle {
                    texture: handle,
                    sprite: Sprite {
                        rect: Some(Rect::new(96.0 * 2.0, 32.0, 96.0 * 2.0 + 64.0, 32.0 + 32.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(28.0, -5.0, 0.0)
                        .with_scale(Vec3::new(-1.0, 1.0, 1.0))
                        .with_rotation(Quat::from_rotation_z(-1.7)),
                    ..default()
                },
                marker: RightFoot,
                name: Name::new("RightFoot"),
            },
        }
    }
}

/*
pub fn add_tee(commands: Commands, handle: Handle<Image>, translation: Vec3, name: &str) {
    TeeBundle {
        tee: Tee,
        name: Name::new(name.to_string()),
        spatial_bundle: SpatialBundle {
            transform: Transform::from_translation(translation),
            ..Default::default()
        },
    }
    .spawn(commands, handle);
}
*/
