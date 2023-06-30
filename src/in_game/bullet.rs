pub(crate) mod bullet_source;
pub(crate) mod new_bullet;
pub(crate) mod new_bullet_event_writer;
pub(crate) mod new_bullet_timer;

use bevy::prelude::*;

pub(crate) trait Bullet: Component + Copy + Clone {
    fn run(&self, transform: &mut Transform, time: &Time);
}

pub(crate) fn run<B: Bullet>(mut bullet_query: Query<(&B, &mut Transform)>, time: Res<Time>) {
    bullet_query.iter_mut().for_each(|(bullet, mut transform)| {
        bullet.run(&mut transform, &time);
    });
}

pub(crate) fn despawn<B: Bullet>(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<B>>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    let max = Vec2 {
        x: window.width() / 2.,
        y: window.height() / 2.,
    };

    bullet_query.iter().for_each(|(entity, transform)| {
        let (x, y) = (transform.translation.x, transform.translation.y);

        if x < -max.x || max.x < x || y < -max.y || max.y < y {
            commands.entity(entity).despawn();
        }
    });
}
