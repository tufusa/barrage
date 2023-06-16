use bevy::prelude::*;

mod delta;

pub(crate) fn setup(mut commands: Commands) {
    delta::setup(&mut commands);
}
