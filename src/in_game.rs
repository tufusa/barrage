use bevy::prelude::*;

mod delta;

#[derive(Component)]
pub(crate) struct InGame;

pub(crate) fn setup(mut commands: Commands) {
    delta::spawn(&mut commands, InGame);
}
