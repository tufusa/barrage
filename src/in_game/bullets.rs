use bevy::prelude::*;

pub(crate) mod straight;

#[derive(Component)]
pub(crate) struct StraightBullet {
    speed: f32,
    angle: f32,
}
