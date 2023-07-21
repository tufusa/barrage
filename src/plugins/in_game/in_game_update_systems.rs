use bevy::prelude::*;

use crate::{app_state::AppState, hierarchy, in_game};

impl Plugin for super::InGameUpdateSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                in_game::game_timer::tick,
                hierarchy::sync::<in_game::enemy::Enemy>,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_plugin(super::NewBulletEventWriterSystems)
        .add_plugin(super::BulletSpawnClockSystems)
        .add_plugin(super::BulletSpawnSystems)
        .add_plugin(super::BulletRunSystems)
        .add_plugin(super::BulletDespawnSystems)
        .add_plugin(super::EnemyRunSystems)
        .add_plugin(super::DeltaUpdateSystems);
    }
}
