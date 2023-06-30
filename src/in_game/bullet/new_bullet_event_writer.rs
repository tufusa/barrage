use bevy::prelude::*;

use super::{
    bullet_source,
    new_bullet::{self, NewBullet},
    new_bullet_timer, Bullet,
};

pub(crate) fn new_bullet_event_writer<B: Bullet>(
    bullet_source_query: Query<(&Transform, &bullet_source::BulletSource<B>)>,
    new_bullet_timer: Res<new_bullet_timer::NewBulletTimer<B>>,
    mut new_bullet_event: EventWriter<new_bullet::NewBullet<B>>,
) {
    if !new_bullet_timer.0.finished() {
        return;
    }

    bullet_source_query
        .iter()
        .for_each(|(transform, bullet_source)| {
            new_bullet_event.send(NewBullet {
                bullet: bullet_source.0,
                translation: transform.translation.truncate(),
            })
        })
}
