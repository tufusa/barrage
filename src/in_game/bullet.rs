use bevy::prelude::*;

pub(crate) trait Bullet: Component {
    fn run(&self, transform: &mut Transform);
}

pub(crate) fn run<B: Bullet>(mut bullet_query: Query<(&B, &mut Transform)>) {
    bullet_query.iter_mut().for_each(|(bullet, mut transform)| {
        bullet.run(&mut transform);
    })
}
