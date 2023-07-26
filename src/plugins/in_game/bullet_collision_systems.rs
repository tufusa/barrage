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
            Update,
            (
                collision::<StraightBullet>,
                collision::<PlayerStraightBullet>,
                collision::<HomingBullet>,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
