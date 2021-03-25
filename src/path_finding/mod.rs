mod funnel;
mod path_finder;

pub mod grid;

use bevy::prelude::*;

use crate::path_finding::funnel::Funnel;
use crate::path_finding::grid::Grid;
use crate::path_finding::path_finder::PathFinder;

use self::funnel::Portal;

pub fn astar(start: Vec2, end: Vec2, grid: &Grid) -> Vec<(i32, i32)> {
    let path_finder = PathFinder { grid };
    path_finder.path(start, end)
}

pub fn funnel_portals(start: Vec2, end: Vec2, grid: &Grid) -> Vec<Portal> {
    let path_finder = PathFinder { grid };
    let path = path_finder.path(start, end);

    let funnel = Funnel::from_path(
        start,
        end,
        path,
        grid.tile_size,
        grid.map_width(),
        grid.map_height(),
    );

    funnel.portals
}

pub fn find_path(start: Vec2, end: Vec2, grid: &Grid) -> Vec<Vec2> {
    // Step 1: Run Astar on the Grid for a global best path
    let path_finder = PathFinder { grid };
    let path = path_finder.path(start, end);

    info!("ASTAR PATH: {:?}", path);

    // Step 2: Run the funnel algorithm to find the optimal path withing
    // the grid path.
    let funnel = Funnel::from_path(
        start,
        end,
        path,
        grid.tile_size,
        grid.map_width(),
        grid.map_height(),
    );

    let funnel_path = funnel.string_pull();
    info!("FUNNEl PATH: {:?}", funnel_path);

    funnel_path
}

pub fn draw_astar_path(
    path: Vec<(i32, i32)>,
    commands: &mut Commands,
    color: Handle<ColorMaterial>,
) {
    for p in path.iter() {
        commands.spawn(SpriteBundle {
            material: color.clone(),
            transform: Transform::from_xyz(
                p.0 as f32 * 32.0 + 16.0 - 800.0,
                p.1 as f32 * 32.0 + 16.0 - 800.0,
                500.0,
            ),
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            visible: Visible {
                is_visible: true,
                is_transparent: false,
            },
            ..Default::default()
        });
    }
}

pub fn draw_funnel_portals(
    portals: Vec<Portal>,
    commands: &mut Commands,
    color: Handle<ColorMaterial>,
) {
    for p in portals.iter() {
        let midpoint = (p.left + p.right) / 2.0;
        let diff = p.left - p.right;
        let length = diff.length();
        let angle = Vec2::new(1.0, 0.0).angle_between(diff);
        let x = midpoint.x;
        let y = midpoint.y;

        commands.spawn(SpriteBundle {
            material: color.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 500.0),
                rotation: Quat::from_rotation_z(angle),
                ..Default::default()
            },
            sprite: Sprite::new(Vec2::new(length, 4.0)),
            visible: Visible {
                is_visible: true,
                is_transparent: false,
            },
            ..Default::default()
        });
    }
}

pub fn draw_funnel_path(path: Vec<Vec2>, commands: &mut Commands, color: Handle<ColorMaterial>) {
    for i in 0..path.len() - 1 {
        let p1 = path[i];
        let p2 = path[i + 1];

        let midpoint = (p1 + p2) / 2.0;
        let diff = p2 - p1;
        let length = diff.length();
        let angle = Vec2::new(1.0, 0.0).angle_between(diff);
        let x = midpoint.x;
        let y = midpoint.y;

        commands.spawn(SpriteBundle {
            material: color.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 500.0),
                rotation: Quat::from_rotation_z(angle),
                ..Default::default()
            },
            sprite: Sprite::new(Vec2::new(length, 4.0)),
            visible: Visible {
                is_visible: true,
                is_transparent: false,
            },
            ..Default::default()
        });
    }
}
