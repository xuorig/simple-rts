use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use bevy::prelude::*;

use crate::grid::Grid;
use crate::tiled::Map;

pub struct PathFinder<'a> {
    pub map: &'a Map,
    pub grid: &'a Grid,
    pub world_width: i32,
    pub world_height: i32,
}

type Location = (i32, i32);

impl<'a> PathFinder<'a> {
    pub fn new(map: &'a Map, grid: &'a Grid) -> Self {
        Self {
            map,
            grid,
            world_width: map.width * map.tile_width,
            world_height: map.height * map.tile_height,
        }
    }

    pub fn path(&self, from: Vec3, to: Vec3) -> Vec<Location> {
        let from_location = self.world_to_grid_coordinates(from);
        let to_location = self.world_to_grid_coordinates(to);

        let mut came_from = HashMap::<Location, Location>::new();
        let mut cost_so_far = HashMap::<Location, i32>::new();

        let mut open_list = BinaryHeap::new();
        open_list.push(PathNodePriority {
            loc: from_location,
            f_score: 0,
        });
        cost_so_far.insert(from_location, 0);

        while !open_list.is_empty() {
            let current = open_list.pop().unwrap();

            if current.loc.0 == to_location.0 && current.loc.1 == to_location.1 {
                return self.reconstruct_path(current.loc, came_from);
            }

            for neighbor_location in self.grid.accessible_neighbors(current.loc) {
                let new_cost = cost_so_far.get(&current.loc).unwrap() + 10;
                let neighbhor_cost = cost_so_far.get(&neighbor_location);

                if neighbhor_cost.is_none() || &new_cost < neighbhor_cost.unwrap() {
                    cost_so_far.insert(neighbor_location, new_cost);
                    let priority = new_cost + self.heuristic(neighbor_location, to_location);
                    open_list.push(PathNodePriority {
                        loc: neighbor_location,
                        f_score: priority,
                    });
                    came_from.insert(neighbor_location, current.loc);
                }
            }
        }

        // TODO Let's return a result here when path is not found
        vec![]
    }

    fn world_to_grid_coordinates(&self, position: Vec3) -> Location {
        (
            ((position.x + self.world_width as f32 / 2.0) / self.map.tile_width as f32) as i32,
            ((position.y + self.world_height as f32 / 2.0) / self.map.tile_height as f32) as i32,
        )
    }

    fn reconstruct_path(
        &self,
        end: Location,
        came_from: HashMap<Location, Location>,
    ) -> Vec<Location> {
        let mut path = vec![];
        path.push(end);

        let mut current = &end;

        while let Some(next) = came_from.get(&current) {
            current = next;
            path.push(next.clone());
        }

        path.reverse();
        path
    }

    fn heuristic(&self, a: Location, b: Location) -> i32 {
        // Euclidean Distance
        // let euclidian_dist = (((a.0 - b.1).pow(2) + (a.0 - b.1).pow(2)) as f64).sqrt();
        // euclidian_dist as i32

        // Chebyshev Distance
        std::cmp::max(b.0 - a.0, b.1 - a.1)
    }
}

pub struct PathNodePriority {
    pub loc: Location,
    pub f_score: i32,
}

impl Ord for PathNodePriority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.cmp(&other.f_score).reverse()
    }
}

impl PartialOrd for PathNodePriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PathNodePriority {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}

impl Eq for PathNodePriority {}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_path_finding() {
//         let pf = PathFinder {};
//         let path = pf.path((1, 1), (4, 6));
//         assert_eq!(vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (4, 6)], path);
//     }
// }
