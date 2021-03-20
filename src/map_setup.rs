use crate::tiled::Map;
use bevy::prelude::*;

pub fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let map_width = (map.width * map.tile_width) as f32;
    let map_height = (map.height * map.tile_height) as f32;

    // Assuming only one tileset for now
    let tile_set_path = &map.tilesets[0].image;
    let texture_handle = asset_server.load(tile_set_path.as_str());
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(map.tile_width as f32, map.tile_height as f32),
        map.tilesets[0].columns as usize,
        (map.tilesets[0].tile_count / map.tilesets[0].columns) as usize,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for layer in map.layers.iter() {
        for y in 0..layer.height {
            for x in 0..layer.width {
                let sprite_index = layer.data[(y * layer.width + x) as usize] - 1;

                if sprite_index == -1 {
                    continue;
                }

                // Tiled renders top down
                let real_y = layer.height - y - 1;

                let translation = Vec3::new(
                    (x * map.tile_width) as f32 + (map.tile_width / 2) as f32 - (map_width / 2.0),
                    (real_y * map.tile_height) as f32 + (map.tile_height / 2) as f32
                        - (map_height / 2.0),
                    layer.id as f32,
                );

                commands.spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
                        index: sprite_index as u32,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        }
    }
}
