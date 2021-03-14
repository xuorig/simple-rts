mod mouse_position;
mod selection_box;
mod skeleton;

use bevy::prelude::*;

use mouse_position::MousePositionPlugin;
use selection_box::SelectionBoxPlugin;
use skeleton::SkeletonPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(MousePositionPlugin)
        .add_plugin(SelectionBoxPlugin)
        .add_plugin(SkeletonPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands) {
    // Add the game's entities to our world
    commands
        // cameras
        .spawn(OrthographicCameraBundle::new_2d());
}
