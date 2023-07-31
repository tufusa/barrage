use bevy::{prelude::*, text::TextLayoutInfo};

use crate::app_state;

#[derive(Component)]
pub struct Title;

pub(crate) fn check_next(
    input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<app_state::AppState>>,
) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(app_state::AppState::InGame);
    }
}

pub(crate) fn cleanup(mut commands: Commands, title_query: Query<Entity, With<Title>>) {
    title_query
        .iter()
        .for_each(|title| commands.entity(title).despawn());
}
