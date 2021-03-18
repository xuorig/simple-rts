mod grid;
mod map_setup;
mod mouse_position;
mod path_finding;
mod selection_box;
mod skeleton;

use bevy::prelude::*;

use grid::Grid;
use mouse_position::MousePositionPlugin;
use selection_box::SelectionBoxPlugin;
use skeleton::SkeletonPlugin;

fn main() {
    let map_grid = Grid::from_json("assets/map.json".to_string()).unwrap();

    App::build()
        .insert_resource(WindowDescriptor {
            title: "Simple RTS Demo".to_string(),
            width: 1024.,
            height: 1024.,
            vsync: false,
            ..Default::default()
        })
        .insert_resource(map_grid)
        .add_plugins(DefaultPlugins)
        .add_plugin(MousePositionPlugin)
        .add_plugin(SelectionBoxPlugin)
        .add_plugin(SkeletonPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(map_setup::setup.system())
        .run();
}

fn setup(commands: &mut Commands) {
    // Add the game's entities to our world
    commands
        // cameras
        .spawn(OrthographicCameraBundle::new_2d());
}
