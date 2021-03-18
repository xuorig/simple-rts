use serde::Deserialize;
use std::fs;
use std::io;

#[derive(Copy, Clone)]
pub enum TileType {
    WALKABLE,
    UNWALKABLE,
}

pub struct Grid {
    grid: Vec<Vec<TileType>>,
}

#[derive(Deserialize, Debug)]
struct GridRepr {
    grid: Vec<Vec<String>>,
}

impl Grid {
    pub fn from_json(path: String) -> Result<Grid, io::Error> {
        let data = fs::read_to_string(path).expect("Unable to read path file");
        let g: GridRepr = serde_json::from_str(data.as_str())?;

        let mut grid = Grid { grid: vec![] };

        for row in g.grid.iter() {
            let mut new_row = vec![];

            for col in row.iter() {
                let tile_type = if col == "W" {
                    TileType::UNWALKABLE {}
                } else {
                    TileType::WALKABLE {}
                };

                new_row.push(tile_type);
            }

            grid.grid.push(new_row);
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
                && neighbor_location.0 < 32
                && neighbor_location.1 >= 0
                && neighbor_location.1 < 32;

            if inbounds {
                match self.at(neighbor_location) {
                    TileType::WALKABLE => {
                        result.push(neighbor_location);
                    }
                    _ => {}
                }
            }
        }

        result
    }
}
