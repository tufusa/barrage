use bevy::prelude::*;

use crate::{app_state::AppState, in_game::bullet::bullet_spawn_clock};

impl Plugin for super::BulletSpawnClockSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (bullet_spawn_clock::tick,).run_if(in_state(AppState::InGame)),
        );
    }
}
