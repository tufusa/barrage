use bevy::prelude::*;

use crate::app_state;

#[derive(Component)]
pub struct Title;

pub(crate) fn setup(mut next_state: ResMut<NextState<app_state::AppState>>) {
    next_state.set(app_state::AppState::InGame);
}

pub(crate) fn cleanup(mut commands: Commands, title_query: Query<Entity, With<Title>>) {
    title_query
        .iter()
        .for_each(|title| commands.entity(title).despawn());
}
