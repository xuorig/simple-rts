use crate::tiled::{Map, PropertyValue};

const WALKABLE: &str = "walkable";

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileType {
    WALKABLE,
    UNWALKABLE,
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<TileType>>,
    pub tile_size: f32,
}

#[derive(Debug, Clone)]
pub struct GridError;

impl Grid {
    /// Builds a collision grid for path finding from a Tiled map
    pub fn from_tiled_map(map: &Map) -> Result<Grid, GridError> {
        let mut grid = Grid {
            grid: vec![],
            tile_size: map.tile_width as f32,
        };

        for y in 0..map.height {
            let mut current_row = vec![];

            for x in 0..map.width {
                let mut tile_type = TileType::WALKABLE;

                for layer in map.layers.iter() {
                    // Check for a collision on each layer
                    // Collisions are defined as custom properties on the tiles
                    let tile_number = layer.data[(y * layer.width + x) as usize];
                    // Assuming a single tileset at the moment
                    let tileset = &map.tilesets[0];

                    // Indexing is weird?
                    let tile = tileset.tiles.iter().find(|&t| t.id == tile_number - 1);

                    if let Some(tile) = tile {
                        let walkable_property =
                            tile.properties.iter().find(|&p| p.name == WALKABLE);

                        if let Some(walk) = walkable_property {
                            if let PropertyValue::Bool(w) = walk.value {
                                if !w {
                                    tile_type = TileType::UNWALKABLE;
                                }
                            }
                        }
                    }
                }

                current_row.push(tile_type);
            }

            grid.grid.insert(0, current_row);
        }

        Ok(grid)
    }

    pub fn at(&self, position: (i32, i32)) -> TileType {
        self.grid[position.1 as usize][position.0 as usize]
    }

    pub fn accessible_neighbors(&self, position: (i32, i32)) -> Vec<(i32, i32)> {
        let neighbor_deltas = vec![
            (0, -1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut result = vec![];

        for (i, j) in &neighbor_deltas {
            let neighbor_location = (position.0 + i, position.1 + j);

            let inbounds = neighbor_location.0 >= 0
                && neighbor_location.0 < self.grid[0].len() as i32
                && neighbor_location.1 >= 0
                && neighbor_location.1 < self.grid.len() as i32;

            if inbounds {
                // If we're a diagonal, we consider the tiles on each side too
                if neighbor_location.0 != position.0 && neighbor_location.1 != position.1 {
                    let surrounding_tile_x = (position.0 + i, position.1);
                    let surrounding_tile_y = (position.0, position.1 + j);

                    if self.at(neighbor_location) == TileType::WALKABLE
                        && self.at(surrounding_tile_x) == TileType::WALKABLE
                        && self.at(surrounding_tile_y) == TileType::WALKABLE
                    {
                        result.push(neighbor_location);
                    }
                } else {
                    match self.at(neighbor_location) {
                        TileType::WALKABLE => {
                            result.push(neighbor_location);
                        }
                        _ => {}
                    };
                }
            }
        }

        result
    }

    pub fn map_width(&self) -> f32 {
        if self.grid.first().is_some() {
            self.grid.first().unwrap().len() as f32 * self.tile_size
        } else {
            0.0
        }
    }

    pub fn map_height(&self) -> f32 {
        self.grid.len() as f32 * self.tile_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_tiled_map() {
        let map = Map::from_json_file("assets/basic_map.json").expect("Failed to load map");

        let grid = Grid::from_tiled_map(&map).expect("Failed to build grid from Tiled map");

        let tile_walk_type = grid.at((0, 0));
        assert_eq!(TileType::WALKABLE, tile_walk_type);
    }
}
