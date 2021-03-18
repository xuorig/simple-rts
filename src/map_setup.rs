use crate::grid::{Grid, TileType};
use bevy::prelude::*;

const TILE_SIZE: f32 = 32.;

const MAP_HEIGHT: i32 = 32;
const MAP_WIDTH: i32 = 32;

pub fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    map: Res<Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let grass_handle = asset_server.load("grass.png");
    let water_handle = asset_server.load("water.png");

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let texture = match map.at((x, y)) {
                TileType::UNWALKABLE => water_handle.clone(),
                _ => grass_handle.clone(),
            };

            commands.spawn(SpriteBundle {
                material: materials.add(texture.into()),
                transform: Transform {
                    translation: Vec3::new(
                        (x as f32 * TILE_SIZE) + (TILE_SIZE / 2.0) - 512.0,
                        (y as f32 * TILE_SIZE) + (TILE_SIZE / 2.0) - 512.0,
                        0.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
