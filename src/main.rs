mod animation;
mod map_setup;
mod mouse_position;
mod movement;
mod path_finding;
mod selection_box;
mod tiled;
mod unit;

use bevy::prelude::*;
use bevy::render::camera::Camera;

use animation::AnimationPlugin;
use mouse_position::MousePositionPlugin;
use path_finding::grid::Grid;
use selection_box::SelectionBoxPlugin;
use unit::UnitPlugin;

use movement::MovementPlugin;

fn main() {
    let map = tiled::Map::from_json_file("assets/basic_map.json").expect("Couldnt load map");
    let path_finding_grid = Grid::from_tiled_map(&map).expect("Failed to generate collision grid");

    App::build()
        .insert_resource(WindowDescriptor {
            title: "Simple RTS Demo".to_string(),
            width: 1024.,
            height: 1024.,
            vsync: false,
            ..Default::default()
        })
        .insert_resource(path_finding_grid)
        .insert_resource(map)
        .add_plugin(MovementPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(MousePositionPlugin)
        .add_plugin(SelectionBoxPlugin)
        .add_plugin(UnitPlugin)
        .add_plugin(AnimationPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(map_setup::setup.system())
        .add_system(camera_movement.system())
        .run();
}

fn setup(commands: &mut Commands) {
    commands
        // cameras
        .spawn(OrthographicCameraBundle::new_2d());
}

fn camera_movement(keys: Res<Input<KeyCode>>, mut query: Query<(&Camera, &mut Transform)>) {
    if let Some((_camera, mut transform)) = query.iter_mut().next() {
        if keys.pressed(KeyCode::Right) {
            transform.translation.x += 6.0
        }

        if keys.pressed(KeyCode::Left) {
            transform.translation.x -= 6.0
        }

        if keys.pressed(KeyCode::Up) {
            transform.translation.y += 6.0
        }

        if keys.pressed(KeyCode::Down) {
            transform.translation.y -= 6.0
        }
    }
}
