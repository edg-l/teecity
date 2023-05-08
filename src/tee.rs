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

#[derive(Component, Default)]
pub enum Weapon {
    Gun,
    #[default]
    Grenade,
    Katana,
    Laser,
    Shotgun,
}

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
    pub weapon: TeePartBundle<Weapon>,
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
    pub fn new(skin_handle: Handle<Image>, game_handle: Handle<Image>, base_z: f32) -> Self {
        Self {
            weapon: TeePartBundle {
                sprite: SpriteBundle {
                    texture: game_handle,
                    sprite: Sprite {
                        rect: Some(Weapon::default().get_texture_rect()),
                        ..Default::default()
                    },
                    transform: Weapon::default().get_transform(base_z),
                    ..default()
                },
                marker: Weapon::default(),
                name: Name::new("Weapon"),
            },
            body: TeePartBundle {
                sprite: SpriteBundle {
                    texture: skin_handle.clone(),
                    sprite: Sprite {
                        rect: Some(Rect::from_corners(Vec2::ZERO, Vec2::new(96.0, 96.0))),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, base_z + 0.5),
                    ..default()
                },
                marker: Body,
                name: Name::new("Body"),
            },
            left_foot: TeePartBundle {
                sprite: SpriteBundle {
                    texture: skin_handle.clone(),
                    sprite: Sprite {
                        rect: Some(Rect::new(96.0 * 2.0, 32.0, 96.0 * 2.0 + 64.0, 32.0 + 32.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(-28.0, -5.0, base_z + 0.0)
                        .with_rotation(Quat::from_rotation_z(1.7)),
                    ..default()
                },
                marker: LeftFoot,
                name: Name::new("LeftFoot"),
            },
            right_foot: TeePartBundle {
                sprite: SpriteBundle {
                    texture: skin_handle,
                    sprite: Sprite {
                        rect: Some(Rect::new(96.0 * 2.0, 32.0, 96.0 * 2.0 + 64.0, 32.0 + 32.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(28.0, -5.0, base_z + 0.0)
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

impl Weapon {
    pub fn get_texture_rect(&self) -> Rect {
        match self {
            Weapon::Gun => Rect::new(64.0, 32.0 + 96.0, 64.0 + 128.0, 32.0 + 96.0 + 64.0),
            Weapon::Shotgun => Rect::new(
                64.0,
                32.0 + 96.0 + 64.0,
                64.0 + 256.0,
                32.0 + 96.0 + 64.0 + 64.0,
            ),
            Weapon::Grenade => Rect::new(
                64.0,
                32.0 + 96.0 + 64.0 * 2.0,
                64.0 + 256.0,
                32.0 + 96.0 + 64.0 * 2.0 + 64.0,
            ),
            Weapon::Katana => Rect::new(
                64.0 + 32.0,
                32.0 + 96.0 + 64.0 * 3.0,
                64.0 + 256.0,
                32.0 + 96.0 + 64.0 * 3.0 + 64.0,
            ),
            Weapon::Laser => Rect::new(
                64.0,
                32.0 + 96.0 + 64.0 * 4.0,
                64.0 + 224.0,
                32.0 + 96.0 + 64.0 * 4.0 + 96.0,
            ),
        }
    }

    pub fn get_transform(&self, base_z: f32) -> Transform {
        match self {
            Weapon::Katana => Transform::from_xyz(11.5, 50.0, base_z + 0.25)
                .with_scale(Vec3::splat(0.5))
                .with_rotation(Quat::from_rotation_z(1.8)),
            Weapon::Gun => Transform::from_xyz(2.0, 38.0, base_z + 0.25),
            _ => Transform::from_xyz(2.0, 60.0, base_z + 0.25)
                .with_scale(Vec3::splat(0.5))
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
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
