use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Tee city"),
                ..Default::default()
            }),
            ..default()
        }))
        .add_plugin(TilemapPlugin)
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
