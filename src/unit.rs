use crate::animation::{Animation, Animations};
use crate::mouse_position::MouseWorldPosition;
use crate::path_finding;
use crate::path_finding::grid::Grid;
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
    pub velocity: Vec2,
    pub max_speed: f32,
    pub max_force: f32,
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
        .with(Unit {
            selected: false,
            velocity: Vec2::zero(),
            max_speed: 100.0,
            max_force: 250.0,
        });
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
    pub path: Vec<Vec2>,
}

fn order_system(
    commands: &mut Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    mouse_position: Res<MouseWorldPosition>,
    grid: Res<Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Transform, &mut Unit, &mut MoveOrder)>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        for (transform, mut unit, mut move_order) in query.iter_mut() {
            if unit.selected {
                let astar_path = path_finding::astar(
                    Vec2::from(transform.translation),
                    Vec2::from(mouse_position.0),
                    &grid,
                );

                let blue = materials.add(Color::rgba(0.0, 0.0, 255.0, 0.2).into());
                path_finding::draw_astar_path(astar_path, commands, blue);

                let portals = path_finding::funnel_portals(
                    Vec2::from(transform.translation),
                    Vec2::from(mouse_position.0),
                    &grid,
                );
                let red = materials.add(Color::rgba(255.0, 0.0, 0.0, 0.2).into());
                path_finding::draw_funnel_portals(portals, commands, red);

                let mut best_path = path_finding::find_path(
                    Vec2::from(transform.translation),
                    Vec2::from(mouse_position.0),
                    &grid,
                );

                let black = materials.add(Color::rgba(0.0, 0.0, 0.0, 0.2).into());
                path_finding::draw_funnel_path(best_path.clone(), commands, black);

                unit.velocity = Vec2::zero();

                // We're here already
                best_path.remove(0);

                move_order.path = best_path
            }
        }
    }
}

fn move_system(
    time: Res<Time>,
    map: Res<Map>,
    mut query: Query<(&mut Unit, &mut Transform, &mut MoveOrder, &mut Animations)>,
) {
    for (mut unit, mut transform, mut move_order, mut animations) in query.iter_mut() {
        if move_order.path.is_empty() {
            animations.play("idle".to_string());
        } else {
            animations.play("moving".to_string());

            let order_coords = move_order.path[0];

            let desired = order_coords - transform.translation.truncate();
            let desired_velocity = desired * (unit.max_speed / desired.length());
            let force = desired_velocity - unit.velocity;
            let seek = force * (unit.max_force / unit.max_speed);

            unit.velocity += seek * time.delta_seconds();
            // unit.velocity = desired_velocity;

            let speed = unit.velocity.length();

            if speed > unit.max_speed {
                unit.velocity = unit.velocity * (4.0 / speed);
            }

            let new_translation = unit.velocity * time.delta_seconds();

            transform.translation.x += new_translation.x;
            transform.translation.y += new_translation.y;

            let diff = transform.translation.truncate() - order_coords;

            if diff.length() < 12.0 {
                move_order.path.remove(0);
            }
        }
    }
}
