use crate::{misc, physics, player};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // on enter
            .add_systems((game_setup, player::add_player).in_schedule(OnEnter(AppState::InGame)))
            // on update
            .add_systems(
                (
                    physics::move_system,
                    player::player_input.before(physics::move_system),
                    player::player_mouse.before(physics::move_system),
                    misc::aim_target_system.after(physics::move_system),
                    player::player_camera.after(misc::aim_target_system),
                )
                    .in_set(OnUpdate(AppState::InGame)),
            )
            // on exit
            .add_systems(
                (crate::despawn_screen::<OnGameScreen>,).in_schedule(OnExit(AppState::InGame)),
            );
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

fn game_setup(mut commands: Commands, server: Res<AssetServer>) {
    let tilemap_handle: Handle<Image> = server.load("generic_clear.png");

    let map_size = TilemapSize { x: 20, y: 20 };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex((x % 6) + 1),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 64.0, y: 64.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(tilemap_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
