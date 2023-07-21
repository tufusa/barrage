use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::bullets::StraightBullet,
    in_game::{bullet::Bullet, bullets::PlayerStraightBullet},
};

impl Plugin for super::BulletRunSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (StraightBullet::run, PlayerStraightBullet::run).in_set(OnUpdate(AppState::InGame)),
        );
    }
}
