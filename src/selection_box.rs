use bevy::prelude::*;

use crate::mouse_position::MouseWorldPosition;
use crate::unit::Unit;

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
            transform: Transform::from_xyz(0.0, -215.0, 500.0),
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
    mut unit_query: Query<(&Transform, &mut Unit, &mut TextureAtlasSprite)>,
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

                debug!("Selection Box {} {} {} {}", min_x, max_x, min_y, max_y);

                for (unit_transform, mut unit, mut texture_atlas_sprite) in unit_query.iter_mut() {
                    let trans = unit_transform.translation;

                    if trans.x > min_x && trans.x < max_x && trans.y > min_y && trans.y < max_y {
                        unit.selected = true;
                        texture_atlas_sprite.color = Color::RED;
                    } else {
                        unit.selected = false;
                        texture_atlas_sprite.color = Color::WHITE;
                    }
                }

                selection_box.active = false;
                visible.is_visible = false;
            }
        }
    }
}
