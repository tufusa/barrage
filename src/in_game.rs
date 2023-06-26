use bevy::prelude::*;

pub(crate) mod bullet;
pub(crate) mod bullets;
pub(crate) mod delta;
pub(crate) mod game_timer;
mod gumowski_mira;
pub(crate) mod tracer;

#[derive(Component)]
pub(crate) struct InGame;

pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    delta::spawn(&mut commands, &mut meshes, &mut materials, InGame);
}

pub(crate) fn cleanup(mut commands: Commands, in_game_query: Query<Entity, With<InGame>>) {
    in_game_query
        .iter()
        .for_each(|in_game| commands.entity(in_game).despawn());
}
