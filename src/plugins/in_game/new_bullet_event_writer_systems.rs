use bevy::prelude::*;

use crate::{
    app_state::AppState,
    in_game::{
        bullet::bullet_spawn_event_writer::bullet_spawn_event_writer, bullets::StraightBullet,
    },
};

impl Plugin for super::NewBulletEventWriterSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (bullet_spawn_event_writer::<StraightBullet>,).in_set(OnUpdate(AppState::InGame)),
        );
    }
}
