use bevy::prelude::*;

use crate::in_game::phase::Phase;
use crate::in_game::phase::{phase1, phase2, phase3, phase4, phase5};

impl Plugin for super::PhaseSystems {
    fn build(&self, app: &mut App) {
        app.add_state::<Phase>()
            .add_system(phase1::setup.in_schedule(OnEnter(Phase::Phase1)))
            .add_system(phase1::check_clear.in_set(OnUpdate(Phase::Phase1)))
            .add_system(phase1::cleanup.in_schedule(OnExit(Phase::Phase1)))
            .add_system(phase2::setup.in_schedule(OnEnter(Phase::Phase2)))
            .add_system(phase2::check_clear.in_set(OnUpdate(Phase::Phase2)))
            .add_system(phase2::cleanup.in_schedule(OnExit(Phase::Phase2)))
            .add_system(phase3::setup.in_schedule(OnEnter(Phase::Phase3)))
            .add_system(phase3::cleanup.in_schedule(OnExit(Phase::Phase3)))
            .add_system(phase4::setup.in_schedule(OnEnter(Phase::Phase4)))
            .add_system(phase4::cleanup.in_schedule(OnExit(Phase::Phase4)))
            .add_system(phase5::setup.in_schedule(OnEnter(Phase::Phase5)))
            .add_system(phase5::cleanup.in_schedule(OnExit(Phase::Phase5)));
    }
}
