use bevy::prelude::*;

use crate::{app_state::AppState, hierarchy, in_game};

impl Plugin for super::InGameUpdateSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                in_game::game_timer::tick,
                hierarchy::sync::<in_game::enemy::Enemy>,
                hierarchy::sync::<in_game::delta::Delta>,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_plugins((
            super::NewBulletEventWriterSystems,
            super::BulletSpawnClockSystems,
            super::BulletSpawnSystems,
            super::BulletRunSystems,
            super::BulletDespawnSystems,
            super::EnemyRunSystems,
            super::DeltaUpdateSystems,
            super::PhaseSystems,
            super::BulletCollisionSystems,
        ));
    }
}
