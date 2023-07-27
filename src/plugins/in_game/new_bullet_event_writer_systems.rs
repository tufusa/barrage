use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::{
        bullet::bullet_spawn_event_writer::bullet_spawn_event_writer,
        bullets::{HomingBullet, PlayerStraightBullet, ReflectBullet, StraightBullet},
    },
};

impl Plugin for super::NewBulletEventWriterSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                bullet_spawn_event_writer::<StraightBullet>,
                bullet_spawn_event_writer::<PlayerStraightBullet>,
                bullet_spawn_event_writer::<ReflectBullet>,
                bullet_spawn_event_writer::<HomingBullet>,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}
