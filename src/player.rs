use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn add_player(mut commands: Commands) {
    commands.spawn((Player, Name::new("Ryo"), Transform::default()));
}
