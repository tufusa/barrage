use bevy::prelude::*;

use crate::{app_state::AppState, in_game::bullet::bullet_spawn_clock};

impl Plugin for super::BulletSpawnClockSystems {
    fn build(&self, app: &mut App) {
        app.add_systems((bullet_spawn_clock::tick,).in_set(OnUpdate(AppState::InGame)));
    }
}
