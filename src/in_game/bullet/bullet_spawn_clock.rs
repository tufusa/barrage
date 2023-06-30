use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct BulletSpawnClock(pub(crate) Timer);

impl BulletSpawnClock {
    pub(crate) fn new(millis: u64) -> Self {
        Self(Timer::new(
            std::time::Duration::from_millis(millis),
            TimerMode::Repeating,
        ))
    }
}

pub(crate) fn tick(mut bullet_spawn_clock_query: Query<&mut BulletSpawnClock>, time: Res<Time>) {
    bullet_spawn_clock_query.iter_mut().for_each(|mut clock| {
        clock.0.tick(time.delta());
    })
}
