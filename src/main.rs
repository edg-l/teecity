use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use misc::AimTarget;
use physics::Velocity;

pub mod misc;
pub mod physics;
pub mod player;
pub mod tee;

fn main() {
    App::new()
        .register_type::<Velocity>()
        .register_type::<AimTarget>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(general_setup)
        .add_startup_system(player::add_player)
        .add_systems((player::player_input, player::player_mouse).before(physics::move_system))
        .add_system(physics::move_system)
        .add_system(misc::aim_target_system.after(physics::move_system))
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn general_setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
