use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::{
        bullet::collision::collision,
        bullets::{HomingBullet, PlayerStraightBullet, StraightBullet},
    },
};

impl Plugin for super::BulletCollisionSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                collision::<StraightBullet>,
                collision::<PlayerStraightBullet>,
                collision::<HomingBullet>,
            )
                .in_set(OnUpdate(AppState::InGame)),
        );
    }
}
