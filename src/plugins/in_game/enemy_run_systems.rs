use bevy::prelude::*;

use crate::{app_state::AppState, in_game};

impl Plugin for super::EnemyRunSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (in_game::enemy::boss::run, in_game::enemy::normal1::run)
                .in_set(OnUpdate(AppState::InGame)),
        );
    }
}
