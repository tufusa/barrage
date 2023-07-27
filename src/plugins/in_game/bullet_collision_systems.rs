use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::{
        bullet::collision::collision,
        bullets::{HomingBullet, PlayerStraightBullet, ReflectBullet, StraightBullet},
    },
};

impl Plugin for super::BulletCollisionSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                collision::<StraightBullet>,
                collision::<PlayerStraightBullet>,
                collision::<ReflectBullet>,
                collision::<HomingBullet>,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
