use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::enemy::{boss, normal1, normal2, normal3},
};

impl Plugin for super::EnemyRunSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                boss::run,
                boss::check_despawn,
                normal1::run,
                normal1::check_despawn,
                normal2::run,
                normal2::check_despawn,
                normal3::run,
                normal3::check_despawn,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
