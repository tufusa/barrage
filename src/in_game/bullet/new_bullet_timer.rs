use std::marker::PhantomData;

use bevy::prelude::*;

use crate::in_game::bullets::StraightBullet;

use super::Bullet;

#[derive(Resource, Default)]
pub(crate) struct NewBulletTimer<B: Bullet>(pub(crate) Timer, PhantomData<B>);

pub(crate) fn setup(commands: &mut Commands) {
    commands.insert_resource(NewBulletTimer::<StraightBullet>(
        Timer::new(std::time::Duration::from_millis(200), TimerMode::Repeating),
        PhantomData,
    ));
}

pub(crate) fn tick<B: Bullet>(mut new_bullet_timer: ResMut<NewBulletTimer<B>>, time: Res<Time>) {
    new_bullet_timer.0.tick(time.delta());
}
