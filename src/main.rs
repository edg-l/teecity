#![allow(clippy::type_complexity)]

use bevy::window::PresentMode;
use bevy::{prelude::*, window::WindowMode};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use misc::AimTarget;
use physics::Velocity;

pub mod game;
pub mod menu;
pub mod misc;
pub mod physics;
pub mod player;
pub mod tee;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Editor,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

fn main() {
    App::new()
        .register_type::<Velocity>()
        .register_type::<AimTarget>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Tee city"),
                mode: WindowMode::BorderlessFullscreen,
                present_mode: PresentMode::AutoVsync,
                ..Default::default()
            }),
            ..default()
        }))
        .insert_resource(Volume(10))
        .add_plugin(TilemapPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_state::<AppState>()
        .add_startup_system(setup)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
