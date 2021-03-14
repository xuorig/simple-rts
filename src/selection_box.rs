use bevy::prelude::*;

use crate::mouse_position::MouseWorldPosition;
use crate::skeleton::Skeleton;

pub struct SelectionBoxPlugin;

pub struct SelectionBox {
    pub active: bool,
    pub initial_position: Vec3,
}

impl Plugin for SelectionBoxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(selection_box_system.system());
    }
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgba(0.0, 255.0, 0.0, 0.2).into()),
            transform: Transform::from_xyz(0.0, -215.0, 0.0),
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            visible: Visible {
                is_visible: false,
                is_transparent: false,
            },
            ..Default::default()
        })
        .with(SelectionBox {
            active: false,
            initial_position: Vec3::default(),
        });
}

fn selection_box_system(
    mouse_position: Res<MouseWorldPosition>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut SelectionBox, &mut Transform, &mut Sprite, &mut Visible)>,
    mut skeleton_query: Query<(&Transform, &mut Skeleton, &mut Handle<ColorMaterial>)>,
) {
    if let Some((mut selection_box, mut transform, mut sprite, mut visible)) =
        query.iter_mut().next()
    {
        if mouse_buttons.just_pressed(MouseButton::Left) {
            selection_box.active = true;
            selection_box.initial_position = mouse_position.0;
            visible.is_visible = true;
        }

        if mouse_buttons.pressed(MouseButton::Left) {
            sprite.size.x = (mouse_position.0.x - selection_box.initial_position.x).abs();
            sprite.size.y = (mouse_position.0.y - selection_box.initial_position.y).abs();

            transform.translation.x = (mouse_position.0.x + selection_box.initial_position.x) / 2.0;
            transform.translation.y = (mouse_position.0.y + selection_box.initial_position.y) / 2.0;
        } else {
            if selection_box.active {
                // Select Units
                let min_x = transform.translation.x - (sprite.size.x / 2.0);
                let max_x = transform.translation.x + (sprite.size.x / 2.0);
                let min_y = transform.translation.y - (sprite.size.y / 2.0);
                let max_y = transform.translation.y + (sprite.size.y / 2.0);

                info!("{} {} {} {}", min_x, max_x, min_y, max_y);

                let selected_material = materials.add(Color::rgb(0.0, 255.0, 0.0).into());
                let unselected = materials.add(Color::rgb(0.7, 0.7, 0.7).into());

                for (skeleton_transform, mut skeleton, mut color_material) in
                    skeleton_query.iter_mut()
                {
                    let trans = skeleton_transform.translation;
                    info!("{:?}", trans);

                    if trans.x > min_x && trans.x < max_x && trans.y > min_y && trans.y < max_y {
                        skeleton.selected = true;
                        *color_material = selected_material.clone();
                    } else {
                        skeleton.selected = false;
                        *color_material = unselected.clone();
                    }
                }

                // Remove Box
                selection_box.active = false;
                visible.is_visible = false;
            }
        }
    }
}
