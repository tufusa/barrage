use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::{bullet::Bullet, bullets::StraightBullet},
};

impl Plugin for super::BulletSpawnSystems {
    fn build(&self, app: &mut App) {
        app.add_systems((StraightBullet::spawn,).in_set(OnUpdate(AppState::InGame)));
    }
}
