use bevy::prelude::*;

use crate::in_game::phase::Phase;
use crate::in_game::phase::{phase1, phase2, phase3, phase4, phase5};

impl Plugin for super::PhaseSystems {
    fn build(&self, app: &mut App) {
        app.add_state::<Phase>()
            .add_systems(OnEnter(Phase::Phase1), phase1::setup)
            .add_systems(OnEnter(Phase::Phase2), phase2::setup)
            .add_systems(OnEnter(Phase::Phase3), phase3::setup)
            .add_systems(OnEnter(Phase::Phase4), phase4::setup)
            .add_systems(OnEnter(Phase::Phase5), phase5::setup)
            .add_systems(Update, phase1::check_clear.run_if(in_state(Phase::Phase1)))
            .add_systems(Update, phase2::check_clear.run_if(in_state(Phase::Phase2)))
            .add_systems(Update, phase3::check_clear.run_if(in_state(Phase::Phase3)))
            .add_systems(Update, phase4::check_clear.run_if(in_state(Phase::Phase4)))
            .add_systems(Update, phase5::check_clear.run_if(in_state(Phase::Phase5)))
            .add_systems(OnExit(Phase::Phase1), phase1::cleanup)
            .add_systems(OnExit(Phase::Phase2), phase2::cleanup)
            .add_systems(OnExit(Phase::Phase3), phase3::cleanup)
            .add_systems(OnExit(Phase::Phase4), phase4::cleanup)
            .add_systems(OnExit(Phase::Phase5), phase5::cleanup);
    }
}
