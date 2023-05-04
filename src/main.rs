use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod entity;
pub mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(player::add_player)
        .run();
}

fn print_position_system(query: Query<&Transform>) {
    for transform in &query {
        println!("position: {:?}", transform.translation);
    }
}
