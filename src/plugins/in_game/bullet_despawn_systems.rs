use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::{
        bullet::force_despawn,
        bullets::{HomingBullet, PlayerStraightBullet, StraightBullet},
    },
};

impl Plugin for super::BulletDespawnSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                force_despawn::<StraightBullet>,
                force_despawn::<PlayerStraightBullet>,
                force_despawn::<HomingBullet>,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
