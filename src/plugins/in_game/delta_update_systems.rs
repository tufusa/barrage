use bevy::prelude::*;

use crate::{app_state::AppState, in_game::delta};

impl Plugin for super::DeltaUpdateSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            
            (delta::control, delta::sync_cursor, delta::check_death)
                .run_if(in_state(AppState::InGame)),
        
        );
    }
}
