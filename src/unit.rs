use crate::animation::{Animation, Animations};
use crate::grid::Grid;
use crate::mouse_position::MouseWorldPosition;
use crate::path_finding::PathFinder;
use crate::tiled::Map;
use bevy::prelude::*;

use std::collections::HashMap;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            // .add_system(animation.system())
            .add_system(order_system.system())
            .add_system(move_system.system());
    }
}

pub struct Unit {
    pub selected: bool,
}

fn spawn_unit(
    commands: &mut Commands,
    translation: Vec3,
    texture_atlas_handle: Handle<TextureAtlas>,
) {
    let mut animations = HashMap::<String, Animation>::new();

    animations.insert("idle".to_string(), Animation::new(vec![1, 2, 3, 4]));
    animations.insert("moving".to_string(), Animation::new(vec![4, 5, 6, 7, 8, 9]));

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation,
                scale: Vec3::new(1.25, 1.25, 999.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: 0,
                color: Color::WHITE,
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Animations::new("idle".to_string(), animations))
        .with(MoveOrder { path: vec![] })
        .with(Unit { selected: false });
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("dino.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 24, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    spawn_unit(
        commands,
        Vec3::new(100.0, 100.0, 500.0),
        texture_atlas_handle.clone(),
    );
    spawn_unit(
        commands,
        Vec3::new(0.0, 0.0, 500.0),
        texture_atlas_handle.clone(),
    );
    spawn_unit(
        commands,
        Vec3::new(-100.0, -100.0, 500.0),
        texture_atlas_handle.clone(),
    );
    spawn_unit(
        commands,
        Vec3::new(23.0, 42.0, 500.0),
        texture_atlas_handle.clone(),
    );
}

pub struct MoveOrder {
    pub path: Vec<(i32, i32)>,
}

fn order_system(
    mouse_buttons: Res<Input<MouseButton>>,
    mouse_position: Res<MouseWorldPosition>,
    grid: Res<Grid>,
    map: Res<Map>,
    mut query: Query<(&Unit, &Transform, &mut MoveOrder)>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        for (unit, transform, mut move_order) in query.iter_mut() {
            if unit.selected {
                let path_finder = PathFinder::new(&map, &grid);
                let best_path = path_finder.path(transform.translation, mouse_position.0);

                info!("Mouse Click {:?}", mouse_position);
                info!("Best Path: {:?}", best_path);

                move_order.path = best_path;
            }
        }
    }
}

fn tile_to_world_coord(tile_pos: (i32, i32), map: &Map) -> Vec2 {
    Vec2::new(
        (tile_pos.0 * map.tile_width) as f32 + (map.tile_width / 2) as f32
            - (map.width * map.tile_width / 2) as f32,
        (tile_pos.1 * map.tile_height) as f32 + (map.tile_height / 2) as f32
            - (map.height * map.tile_height / 2) as f32,
    )
}

fn move_system(
    time: Res<Time>,
    map: Res<Map>,
    mut query: Query<(&Unit, &mut Transform, &mut MoveOrder, &mut Animations)>,
) {
    for (_unit, mut transform, mut move_order, mut animations) in query.iter_mut() {
        if move_order.path.is_empty() {
            animations.current_animation = "idle".to_string();
        } else {
            animations.current_animation = "moving".to_string();

            let order = move_order.path[0];

            let mut x = transform.translation.x;
            let mut y = transform.translation.y;

            let world_coords = tile_to_world_coord(order, &map);

            let order_x = world_coords.x;
            let order_y = world_coords.y;

            let speed = 48.0;

            if (order_x - transform.translation.x).abs() < time.delta_seconds() * speed {
                x = order_x;
            } else if order_x > transform.translation.x {
                transform.rotation = Quat::default();
                x = transform.translation.x + time.delta_seconds() * speed;
            } else if order_x < transform.translation.x {
                transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
                x = transform.translation.x - time.delta_seconds() * speed;
            }

            if (order_y - transform.translation.y).abs() < time.delta_seconds() * speed {
                y = order_y
            } else if order_y > transform.translation.y {
                y = transform.translation.y + time.delta_seconds() * speed;
            } else if order_y < transform.translation.y {
                y = transform.translation.y - time.delta_seconds() * speed;
            }

            if x == order_x && y == order_y {
                // We completed that path node
                move_order.path.remove(0);
            }

            transform.translation = Vec3::new(x, y, 999.0);
        }
    }
}
