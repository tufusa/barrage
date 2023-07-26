use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::{
        bullet::Bullet,
        bullets::{HomingBullet, PlayerStraightBullet, StraightBullet},
    },
};

impl Plugin for super::BulletSpawnSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                StraightBullet::spawn,
                PlayerStraightBullet::spawn,
                HomingBullet::spawn,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
