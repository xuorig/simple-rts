mod funnel;
mod path_finder;

pub mod grid;

use bevy::prelude::*;

use crate::path_finding::funnel::Funnel;
use crate::path_finding::grid::Grid;
use crate::path_finding::path_finder::PathFinder;

pub fn find_path(start: Vec2, end: Vec2, grid: &Grid) -> Vec<Vec2> {
    // Step 1: Run Astar on the Grid for a global best path
    let path_finder = PathFinder { grid };
    let path = path_finder.path(start, end);

    info!("ASTAR RESULT: {:?}", path);

    // Normalize some stuff
    // Bevy has 0 in the center, but our path finding code does not
    let start = Vec2::new(
        start.x + grid.map_width() / 2.0,
        start.y + grid.map_height() / 2.0,
    );
    let end = Vec2::new(
        end.x + grid.map_width() / 2.0,
        end.y + grid.map_height() / 2.0,
    );

    // Step 2: Run the funnel algorithm to find the optimal path withing
    // the grid path.
    let funnel = Funnel::from_path(start, end, path, grid.tile_size);

    let optimal_path = funnel.string_pull();
    info!("FUNNEL RESULT: {:?}", optimal_path);

    // Before sending back, go back to bevy coordinates
    // With 0 at the senter
    optimal_path
        .iter()
        .map(|path_item| {
            Vec2::new(
                path_item.x - grid.map_width() / 2.0,
                path_item.y - grid.map_height() / 2.0,
            )
        })
        .collect()
}
