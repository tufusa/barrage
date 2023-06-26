use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct GameTimer(Timer);

pub(crate) fn setup(commands: &mut Commands) {
    commands.insert_resource(GameTimer(Timer::new(
        std::time::Duration::from_millis(10),
        TimerMode::Repeating,
    )))
}

pub(crate) fn tick(mut game_timer: ResMut<GameTimer>, time: Res<Time>) {
    game_timer.0.tick(time.delta());
}
