use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::{bullet::despawn, bullets::StraightBullet},
};

impl Plugin for super::BulletDespawnSystems {
    fn build(&self, app: &mut App) {
        app.add_systems((despawn::<StraightBullet>,).in_set(OnUpdate(AppState::InGame)));
    }
}
