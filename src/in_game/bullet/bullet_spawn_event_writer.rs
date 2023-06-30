use bevy::prelude::*;

use super::{
    bullet_source::BulletSource,
    bullet_spawn_clock::BulletSpawnClock,
    new_bullet::{self, NewBullet},
    Bullet,
};

pub(crate) fn bullet_spawn_event_writer<B: Bullet>(
    bullet_source_query: Query<(&Transform, &BulletSource<B>, &BulletSpawnClock)>,
    mut new_bullet_event: EventWriter<new_bullet::NewBullet<B>>,
) {
    bullet_source_query
        .iter()
        .for_each(|(transform, bullet_source, clock)| {
            if !clock.0.finished() {
                return;
            }

            new_bullet_event.send(NewBullet {
                bullet: bullet_source.bullet,
                translation: transform.translation.truncate(),
            })
        })
}
