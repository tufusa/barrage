use bevy::prelude::*;

pub(crate) mod bullet;
pub(crate) mod bullets;
pub(crate) mod delta;
pub(crate) mod enemy;
pub(crate) mod game_timer;
pub(crate) mod hp;
pub(crate) mod phase;
pub(crate) mod player_bullet;

#[derive(Component)]
pub(crate) struct InGame;

pub(crate) fn setup(mut commands: Commands) {
    delta::spawn(&mut commands, InGame);
    // enemy::boss::spawn(&mut commands, &mut meshes, &mut materials, InGame);
}

pub(crate) fn cleanup(mut commands: Commands, in_game_query: Query<Entity, With<InGame>>) {
    in_game_query
        .iter()
        .for_each(|in_game| commands.entity(in_game).despawn());
}
