use bevy::prelude::*;

use crate::{app_state::AppState, in_game::bullet::Bullet, in_game::bullets::StraightBullet};

impl Plugin for super::BulletRunSystems {
    fn build(&self, app: &mut App) {
        app.add_systems((StraightBullet::run,).in_set(OnUpdate(AppState::InGame)));
    }
}
