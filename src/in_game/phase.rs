use bevy::prelude::*;

pub(crate) mod phase1;
pub(crate) mod phase2;
pub(crate) mod phase3;
pub(crate) mod phase4;
pub(crate) mod phase5;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(crate) enum Phase {
    Phase1,
    Phase2,
    #[default]
    Phase3,
    Phase4,
    Phase5,
}
