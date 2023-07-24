use bevy::{prelude::*, window::*};

pub(crate) fn position(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    let cursor_window_pos = window.cursor_position()?;
    camera.viewport_to_world_2d(camera_transform, cursor_window_pos)
}
