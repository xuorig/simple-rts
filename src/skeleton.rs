use crate::grid::Grid;
use crate::mouse_position::MouseWorldPosition;
use crate::path_finding::PathFinder;
use crate::tiled::Map;
use bevy::prelude::*;

pub struct SkeletonPlugin;

impl Plugin for SkeletonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            // .add_system(animation.system())
            .add_system(order_system.system())
            .add_system(move_system.system());
    }
}

pub struct Skeleton {
    pub selected: bool,
}

fn spawn_skeleton(
    commands: &mut Commands,
    translation: Vec3,
    texture_atlas_handle: Handle<TextureAtlas>,
) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation,
                scale: Vec3::new(1.25, 1.25, 1.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: 0,
                color: Color::WHITE,
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(MoveOrder { path: vec![] })
        .with(Skeleton { selected: false });
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("dino.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 24, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    spawn_skeleton(
        commands,
        Vec3::new(100.0, 100.0, 500.0),
        texture_atlas_handle.clone(),
    );
    spawn_skeleton(
        commands,
        Vec3::new(0.0, 0.0, 500.0),
        texture_atlas_handle.clone(),
    );
    spawn_skeleton(
        commands,
        Vec3::new(-100.0, -100.0, 500.0),
        texture_atlas_handle.clone(),
    );
    spawn_skeleton(
        commands,
        Vec3::new(23.0, 42.0, 500.0),
        texture_atlas_handle.clone(),
    );
}

fn animation(time: Res<Time>, mut query: Query<(&Skeleton, &mut Timer, &mut TextureAtlasSprite)>) {
    for (_skeleton, mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta().as_secs_f32());

        if timer.finished() {
            sprite.index = (sprite.index + 1) % 11;
        }
    }
}

pub struct MoveOrder {
    pub path: Vec<(i32, i32)>,
}

fn order_system(
    mouse_buttons: Res<Input<MouseButton>>,
    mouse_position: Res<MouseWorldPosition>,
    grid: Res<Grid>,
    map: Res<Map>,
    mut query: Query<(&Skeleton, &Transform, &mut MoveOrder)>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        for (skeleton, transform, mut move_order) in query.iter_mut() {
            if skeleton.selected {
                let path_finder = PathFinder::new(&map, &grid);
                let best_path = path_finder.path(transform.translation, mouse_position.0);

                debug!("Mouse Click {:?}", mouse_position);
                debug!("Best Path: {:?}", best_path);

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
    mut query: Query<(&Skeleton, &mut Transform, &mut MoveOrder, &mut Timer)>,
) {
    for (_skeleton, mut transform, mut move_order, mut timer) in query.iter_mut() {
        timer.tick(time.delta().as_secs_f32());

        if timer.finished() {
            if !move_order.path.is_empty() {
                let order = move_order.path[0];

                let mut x = transform.translation.x;
                let mut y = transform.translation.y;

                let world_coords = tile_to_world_coord(order, &map);

                debug!("Going to Tile: {:?}", order);

                let order_x = world_coords.x;
                let order_y = world_coords.y;

                debug!("In World Coordinates: {:?}", world_coords);

                if (order_x - transform.translation.x).abs() < 5.0 {
                    x = order_x;
                } else if order_x > transform.translation.x {
                    x = transform.translation.x + 5.0;
                } else if order_x < transform.translation.x {
                    x = transform.translation.x - 5.0;
                }

                if (order_y - transform.translation.y).abs() < 5.0 {
                    y = order_y
                } else if order_y > transform.translation.y {
                    y = transform.translation.y + 5.0;
                } else if order_y < transform.translation.y {
                    y = transform.translation.y - 5.0;
                }

                if x == order_x && y == order_y {
                    // We completed that path node
                    move_order.path.remove(0);
                }

                transform.translation = Vec3::new(x, y, 1.0);
            }
        }
    }
}
