use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;

pub struct MousePositionPlugin;

#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
pub struct MousePosition(pub Vec2);

#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
pub struct MouseWorldPosition(pub Vec3);

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(MouseWorldPosition::default())
            .insert_resource(MousePosition::default())
            .add_system(track_mouse_orthogonal_position.system())
            .add_system(track_mouse_position.system());
    }
}

fn track_mouse_position(
    mut mouse_position: ResMut<MousePosition>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    for event in cursor_moved_events.iter() {
        mouse_position.0 += event.position;
    }
}

fn track_mouse_orthogonal_position(
    mut mouse_world_position: ResMut<MouseWorldPosition>,
    mut event_reader: EventReader<CursorMoved>,
    cameras: Query<(&GlobalTransform, &OrthographicProjection)>,
) {
    if let Some(event) = event_reader.iter().rev().next() {
        let (global_transform, projection) = cameras
            .iter()
            .next()
            .expect("Could not find an orthographic camera");

        mouse_world_position.0 = global_transform.mul_vec3(
            event.position.extend(0.0)
                + Vec3::new(projection.left, projection.bottom, projection.near),
        );
    }
}
