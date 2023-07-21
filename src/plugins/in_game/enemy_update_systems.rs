use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::enemy::{boss, normal1},
};

impl Plugin for super::EnemyRunSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                boss::run,
                boss::check_despawn,
                normal1::run,
                normal1::check_despawn,
            )
                .in_set(OnUpdate(AppState::InGame)),
        );
    }
}
