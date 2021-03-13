use bevy::prelude::*;

use crate::mouse_position::MouseWorldPosition;

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
    mut query: Query<(&mut SelectionBox, &mut Transform, &mut Sprite, &mut Visible)>,
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
            sprite.size.x = mouse_position.0.x - selection_box.initial_position.x;
            sprite.size.y = mouse_position.0.y - selection_box.initial_position.y;
            transform.translation.x = mouse_position.0.x - sprite.size.x / 2.0;
            transform.translation.y = mouse_position.0.y - sprite.size.y / 2.0;
        } else {
            if selection_box.active {
                selection_box.active = false;
                visible.is_visible = false;
            }
        }
    }
}
