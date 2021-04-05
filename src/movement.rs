use std::collections::HashMap;

use crate::animation::Animations;
use crate::mouse_position::MouseWorldPosition;
use crate::path_finding;
use crate::path_finding::grid::Grid;
use crate::unit::*;

use bevy::prelude::*;

use bevy::sprite::collide_aabb::collide;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(order_system.system())
            .add_system(velocity_system.system())
            .add_system(animation_system.system())
            .add_system(physics_system.system());
    }
}

fn animation_system(mut query: Query<(&Unit, &mut Animations)>) {
    for (unit, mut animations) in query.iter_mut() {
        if unit.velocity > Vec2::zero() {
            animations.play("idle".to_string());
        } else {
            animations.play("moving".to_string());
        }
    }
}

fn physics_system(
    time: Res<Time>,
    mut q: QuerySet<(
        Query<(Entity, &Unit, &Transform, &MoveOrder)>,
        Query<(Entity, &Unit, &mut Transform, &mut MoveOrder)>,
    )>,
) {
    let mut movements = HashMap::<Entity, Vec2>::new();

    for (entity, unit, transform, _) in q.q0().iter() {
        let new_translation = unit.velocity * time.delta_seconds();
        let mut is_colliding = false;

        for (other_entity, _, other_transform, _) in q.q0().iter() {
            if entity == other_entity {
                break;
            }

            // Very simple collision handling.
            // TODO:
            //   - Actual collision component
            //   - Not only other units
            //   - Actual sizes, hard coded to 32 rn
            let collide_result = collide(
                Vec3::new(
                    transform.translation.x + new_translation.x,
                    transform.translation.y + new_translation.y,
                    transform.translation.z,
                ),
                Vec2::new(24.0, 24.0),
                other_transform.translation,
                Vec2::new(24.0, 24.0),
            );

            if collide_result.is_some() {
                is_colliding = true;
                break;
            }
        }

        if !is_colliding {
            movements.insert(entity, new_translation);
        }
    }

    for (entity, _, mut transform, mut move_order) in q.q1_mut().iter_mut() {
        if let Some(order_coords) = move_order.path.get(0) {
            if let Some(new_translation) = movements.get(&entity) {
                transform.translation.x += new_translation.x;
                transform.translation.y += new_translation.y;
                info!("New Translation: {}", transform.translation);

                let diff = transform.translation.truncate() - *order_coords;

                // TODO: Is there a better solution to this? To avoid turning around until we find the
                // precise point, when we're close enough we move to the next path segment
                if diff.length() < 6.0 {
                    move_order.path.remove(0);
                }
            }
        }
    }
}

fn velocity_system(time: Res<Time>, mut query: Query<(&mut Unit, &Transform, &MoveOrder)>) {
    for (mut unit, transform, move_order) in query.iter_mut() {
        if let Some(order_coords) = move_order.path.get(0) {
            let desired = *order_coords - transform.translation.truncate();
            info!("Desired Length {}", desired);

            if desired.length() != 0.0 {
                let desired_velocity = desired * (unit.max_speed / desired.length());
                info!("Desired Velocity {}", desired_velocity);
                let force = desired_velocity - unit.velocity;
                info!("Force {}", force);
                let seek = force * (unit.max_force / unit.max_speed);
                info!("Seek {}", seek);

                info!("New Velocity: {}", unit.velocity);
                unit.velocity += seek * time.delta_seconds();

                let speed = unit.velocity.length();

                if speed > unit.max_speed {
                    unit.velocity = unit.velocity * (4.0 / speed);
                }
            }
        } else {
            unit.velocity = Vec2::zero();
        }
    }
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
                let blue = materials.add(Color::rgba(0.0, 0.0, 255.0, 0.2).into());
                let red = materials.add(Color::rgba(255.0, 0.0, 0.0, 0.2).into());

                debug_path(commands, transform, &mouse_position, &grid, blue, red);

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

fn debug_path(
    commands: &mut Commands,
    transform: &Transform,
    mouse_position: &MouseWorldPosition,
    grid: &Grid,
    blue: Handle<ColorMaterial>,
    red: Handle<ColorMaterial>,
) {
    let astar_path = path_finding::astar(
        Vec2::from(transform.translation),
        Vec2::from(mouse_position.0),
        &grid,
    );

    path_finding::draw_astar_path(astar_path, commands, blue);

    let portals = path_finding::funnel_portals(
        Vec2::from(transform.translation),
        Vec2::from(mouse_position.0),
        &grid,
    );
    path_finding::draw_funnel_portals(portals, commands, red);
}
