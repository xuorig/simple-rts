use crate::mouse_position::MouseWorldPosition;
use bevy::prelude::*;

pub struct SkeletonPlugin;

impl Plugin for SkeletonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(animation.system())
            .add_system(order_system.system())
            .add_system(move_system.system());
    }
}

pub struct Skeleton {
    pub selected: bool,
}

fn spawn_skeleton(
    commands: &mut Commands,
    material_handle: Handle<ColorMaterial>,
    translation: Vec3,
) {
    commands
        .spawn(SpriteBundle {
            material: material_handle,
            transform: Transform {
                translation,
                ..Default::default()
            },
            sprite: Sprite::new(Vec2::new(20.0, 20.0)),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(MoveOrder { order: None })
        .with(Skeleton { selected: false });
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let material_handle = materials.add(Color::rgb(0.7, 0.7, 0.7).into());

    spawn_skeleton(
        commands,
        material_handle.clone(),
        Vec3::new(100.0, 100.0, 1.0),
    );
    spawn_skeleton(commands, material_handle.clone(), Vec3::new(0.0, 0.0, 1.0));
    spawn_skeleton(
        commands,
        material_handle.clone(),
        Vec3::new(-100.0, -100.0, 1.0),
    );
    spawn_skeleton(
        commands,
        material_handle.clone(),
        Vec3::new(23.0, 42.0, 1.0),
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
    order: Option<Vec2>,
}

fn order_system(
    mouse_buttons: Res<Input<MouseButton>>,
    mouse_position: Res<MouseWorldPosition>,
    mut query: Query<(&Skeleton, &mut MoveOrder)>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        for (skeleton, mut move_order) in query.iter_mut() {
            if skeleton.selected {
                move_order.order = Some(Vec2::new(mouse_position.0.x, mouse_position.0.y));
            }
        }
    }
}

fn move_system(
    time: Res<Time>,
    mut query: Query<(&Skeleton, &mut Transform, &MoveOrder, &mut Timer)>,
) {
    for (_skeleton, mut transform, move_order, mut timer) in query.iter_mut() {
        timer.tick(time.delta().as_secs_f32());

        if timer.finished() {
            if let Some(order) = move_order.order {
                info!("Moving!");
                info!("{}, {}", order, transform.translation);

                let mut x = transform.translation.x;
                let mut y = transform.translation.y;

                if (order.x - transform.translation.x).abs() < 5.0 {
                    x = order.x;
                } else if order.x > transform.translation.x {
                    x = transform.translation.x + 5.0;
                } else if order.x < transform.translation.x {
                    x = transform.translation.x - 5.0;
                }

                if (order.y - transform.translation.y).abs() < 5.0 {
                    y = order.y
                } else if order.y > transform.translation.y {
                    y = transform.translation.y + 5.0;
                } else if order.y < transform.translation.y {
                    y = transform.translation.y - 5.0;
                }

                transform.translation = Vec3::new(x, y, 1.0);
            }
        }
    }
}
