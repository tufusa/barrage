use bevy::prelude::*;

use crate::{app_state::AppState, in_game::phase::Phase};

pub(crate) fn check_reset(
    input: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_phase: ResMut<NextState<Phase>>,
) {
    if input.just_pressed(KeyCode::Space) {
        next_app_state.set(AppState::InGame);
        next_phase.set(Phase::Phase1);
    }
}
