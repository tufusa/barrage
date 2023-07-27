use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::{
        bullet::force_despawn,
        bullets::{HomingBullet, PlayerStraightBullet, ReflectBullet, StraightBullet},
    },
};

impl Plugin for super::BulletDespawnSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                force_despawn::<StraightBullet>,
                force_despawn::<PlayerStraightBullet>,
                force_despawn::<ReflectBullet>,
                force_despawn::<HomingBullet>,
                ReflectBullet::check_despawn,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
