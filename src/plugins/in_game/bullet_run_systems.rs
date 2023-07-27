use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::bullets::{HomingBullet, ReflectBullet, StraightBullet},
    in_game::{bullet::Bullet, bullets::PlayerStraightBullet},
};

impl Plugin for super::BulletRunSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                StraightBullet::run,
                PlayerStraightBullet::run,
                ReflectBullet::run,
                HomingBullet::run,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
